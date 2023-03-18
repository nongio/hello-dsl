# Simple parser for swiftui like declarative syntax
a rust playground to try to parse a syntax similar to swiftui and produce imperative calls

given the input:
```
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
```

outputs:
```
let layer_3 = Layer ("text")
let layer_2 = Layer ("background") . background (red) . size (100.0 , 200.0)
layer_2 . add_child (layer_3)

let layer_4 = Layer ("text")

let layer_5 = Layer ("text")

let layer_6 = Layer ("text")
let layer_1 = Flexbox ("root") . justify_elements (center) . align_element (center)
layer_1 . add_child (layer_2)
layer_1 . add_child (layer_4)
layer_1 . add_child (layer_5)
layer_1 . add_child (layer_6)
```