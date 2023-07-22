use slint::ModelRc;
use std::collections::HashMap;


pub fn main(objects : HashMap<(usize, usize), parser::Object>) {

    let mut acc = vec![];

    for (_, o) in objects {
        parse_value_helper(0., parser::Value::Dict(o.dict), &mut acc);
    }
    
    let tree_view = View::new().unwrap();

    tree_view.set_items(ModelRc::from(&acc[..]));
    
    tree_view.run().unwrap();
}

fn parse_value_helper(indent: f32, value: parser::Value, acc: &mut Vec<ViewData>) {
    use parser::Value::*;
    match value {
        Number(n) => acc.push(ViewData { indent, text: n.to_string().into() }),
        String(s) => acc.push(ViewData { indent, text: format!("String({s:?})").into() }),
        Key(s) => acc.push(ViewData { indent, text: format!("Key({s:?})").into() }),
        List(l) => for x in l {
            parse_value_helper(indent + 10., x, acc);
        }
        Ref(x, y) => acc.push(ViewData { indent, text: format!("Ref({x}, {y})").into() }),
        Dict(m) => {
            acc.push(ViewData { indent, text: format!("dict").into() });
            for (k, v) in m {
                acc.push(ViewData { indent: indent + 10., text: format!("{k}:").into() });
                parse_value_helper(indent + 10., v, acc);
            }
        }
    }
}

slint::slint! {
    import { ScrollView, VerticalBox } from "std-widgets.slint";
    export struct ViewData {
        text: string,
        indent: length,
    }
    
    export component View inherits Window {
        width: 800px;
        height: 800px;
        in property <[ViewData]> items;
        ScrollView {
            VerticalBox {
                for item in root.items: HorizontalLayout {
                    Rectangle {
                        width: item.indent;
                    }
                    Text {
                        horizontal-alignment: left;
                        text: item.text;
                    }
                }
            }
        }
    }
}
