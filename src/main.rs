use core::fmt;
use std::{*, sync::atomic::{AtomicUsize, Ordering}};

use syn::{__private::{quote::quote, Span}};

#[allow(unused_imports)] // typical / pervasive syn imports
use ::syn::{*,
    parse::{Parse, Parser, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Result, // explicitly shadow it
};

static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

struct LayerItem {
    ident: Ident,
    init: Expr,
    children: Vec<LayerItem>,
}
impl fmt::Display for LayerItem {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let ident = &self.ident;
        let init_code = &self.init;
        self.children.iter().for_each(|child|{
            fmt.write_str("\n");
            fmt.write_str(&child.to_string());
            fmt.write_str("\n");
        });
        let return_string= quote!(let #ident = #init_code).to_string();
        fmt.write_str(&return_string);
        self.children.iter().for_each(|child|{
            let child_ident = &child.ident;
            let return_string= quote!(#ident . add_child( #child_ident )).to_string();
            fmt.write_str("\n");
            fmt.write_str(&return_string);
        });
        Ok(())
    }
}
fn parse_layer_item(input: ParseStream<'_>) -> Result<LayerItem>{

    let call:Expr = input.parse()?;
    let init_code: Result<Expr> = match call {
        Expr::MethodCall(method_call) => {
            Ok(Expr::MethodCall(method_call))
        },
        Expr::Call(call) => {
            Ok(Expr::Call(call))
        }
        _ => Err(syn::Error::new(Span::call_site(), "noooo")),
    };
    if let Ok(init_code) = init_code {
        CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        let variable_name = std::fmt::format(format_args!("layer_{}", CALL_COUNT.load(Ordering::SeqCst)));
        let varname = syn::Ident::new(&variable_name, Span::call_site());
        
        let mut children: Vec<LayerItem> = Vec::new();
        let parsed_children = parse_layer_item_children(input);
        if let Ok(parsed_children) = parsed_children {
            children = parsed_children;
        }
        Ok(LayerItem { ident: varname, init: init_code, children })
    } else {
        Err(syn::Error::new(Span::call_site(), "li morté"))
    }
}

fn parse_layer_item_children(input: ParseStream<'_>) -> Result<Vec<LayerItem>> {
    let mut children:Vec<LayerItem> = Vec::new();
    if input.peek(token::Brace) {
        let content;
        braced!(content in input);

        loop {
            if let Ok(child) = parse_layer_item(&content) {
                children.push(child);
            } else {
                break;
            }
        }
    }
    Ok(children)
}
impl Parse for LayerItem {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let root = parse_layer_item(input)?;
        Ok(root)
    }
}
fn main ()
{
    if let Ok(root) = ::syn::parse2::<LayerItem>(quote!(
        Flexbox("root")
            .justify_elements(center)
            .align_element(center)
        {
            Layer("background")
                .background(red)
                .size(100.0, 200.0)
            {
                Layer("text")
            }
            Layer("text")
            Layer("text")
            Layer("text")
        }
    )) {
        println!("{}", root);
    }
}
