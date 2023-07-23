use std::collections::HashMap;
use parser::{Value, PDF};
use std::rc::Rc;
use slint::{VecModel, ModelRc, Model};


pub fn main(pdf : PDF) {

    let tree_view = View::new().unwrap();

    let mut vec = Vec::new();

    let mut track_list = vec![];

    let (track, tab) = make_tab(pdf.get_meta());

    track_list.push(track);

    let tab_data = TabData {
        title: "Meta".into(),
        data: tab,
    };

    vec.push(tab_data);
    
    for (_, o) in pdf.get_objects() {
        let (track, tab) = make_tab(o.dict());
        track_list.push(track);
        vec.push(TabData {
            title: format!("{:?}", o.id()).into(),
            data: tab,
        })
    }

    let model_rc = ModelRc::from(Rc::new(VecModel::from(vec)));


    let model_clone = model_rc.clone();

    tree_view.on_toggle(move |n, id| {
        let tab = model_clone.row_data(n as _).unwrap();
        let mut row = tab.data.row_data(id as _).unwrap();
        let open =  row.open;
        row.open = !open;
        tab.data.set_row_data(id as _, row);

        let mut children = Vec::new();
        all_children(&track_list[n as usize], id as _, &mut children);
        for child in children {
            let mut row = tab.data.row_data(child as _).unwrap();
            row.hide = open;
            row.open = !open;
            tab.data.set_row_data(child as _, row);
        }

        let mut height = 0.;
        for i in 0 .. tab.data.row_count() {
            let mut row = tab.data.row_data(i).unwrap();
            row.y = height;
            if !row.hide {
                height += 16.;
            }
            tab.data.set_row_data(i, row);
        }
    });

    tree_view.set_tabs(model_rc);
    
    tree_view.run().unwrap();
}

fn make_tab(data: &HashMap<String, Value>) -> (HashMap<i32, Vec<i32>>, ModelRc<ViewData>) {

    let mut acc = vec![];

    let mut track: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut make_track = |id, parent| {
        if !track.contains_key(&parent) {
            track.insert(parent, vec![]);
        }
        let list =  track.get_mut(&parent).unwrap();
        list.push(id);
    };

    for (idx, o) in data {
        let id = acc.len() as i32;
        acc.push(view(id, 0., format!("{idx:?}:"), false));
        parse_value_helper(10., id, o, &mut acc, &mut make_track);
    }
    
    
    (track, ModelRc::from(Rc::new(VecModel::from(acc))))
}

fn all_children(source: &HashMap<i32, Vec<i32>>, id: i32, acc: &mut Vec<i32>) {
    if let Some(children) = source.get(&id) {
        for child in children {
            acc.push(*child);
            all_children(source, *child, acc);
        }
    }
}

fn view(id: i32, indent: f32, text: String, leaf: bool) -> ViewData {
    ViewData { id, text: text.into(), x: indent, y: id as f32 * 16., hide: false, open: true, leaf }
}

fn parse_value_helper(indent: f32, parent: i32, value: &parser::Value, acc: &mut Vec<ViewData>, track: &mut dyn FnMut(i32, i32)) {
    use parser::Value::*;
    let id = acc.len() as i32;
    track(id, parent);
    match value {
        Number(n) => acc.push(view(id, indent, n.to_string(), true)),
        String(s) => acc.push(view(id, indent, format!("String({s:?})"), true)),
        Key(s) => acc.push(view(id, indent, format!("Key({s:?})").into(), true)),
        List(l) => {
            acc.push(view(id, indent, "list:".into(), false));
            for x in l {
                parse_value_helper(indent + 10., id, x, acc, track);
            }
        }
        Ref(x, y) => acc.push(view(id, indent, format!("Ref({x}, {y})"), true)),
        Dict(m) => {
            acc.push(view(id, indent, format!("dict"), false));
            let parent = id;
            for (k, v) in m {
                let key_id = acc.len() as _;
                track(key_id, parent);
                acc.push(view(key_id, indent + 10., format!("{k}:"), false));
                parse_value_helper(indent + 20., key_id, v, acc, track);
            }
        }
    }
}

slint::slint! {
    import { ScrollView, VerticalBox, TabWidget } from "std-widgets.slint";
    export struct ViewData {
        id: int,
        text: string,
        x: length,
        y: length,
        open: bool,
        hide: bool,
        leaf: bool,
    }
    export struct TabData {
        title: string,
        data: [ViewData],
    }
    
    export component View inherits Window {
        width: 800px;
        height: 800px;
        in property <[TabData]> tabs;
        property <int> current-tab: 0;
        callback toggle(int, int);
        ScrollView {
            HorizontalLayout {
                x: 0;
                y: 0;
                width: parent.width;
                height: 16px;
                for tab[index] in tabs: Rectangle {
                    border-width: 1px;
                    border-color: green;
                    if current-tab == index: Text {
                        color: red;
                        text: tab.title;
                    }
                    if current-tab != index: Text {
                        text: tab.title;
                        TouchArea {
                            clicked => { current-tab = index }
                        }
                    }
                }
            }
            for item in tabs[current-tab].data: Rectangle {
                x: 0;
                y: item.y + 16px;
                height: 16px;
                if !item.hide && !item.leaf: Rectangle {
                    x: 16px;
                    width: item.x;
                    height: 16px;
                    Text {
                        text: item.open ? "↓" : "→";
                        TouchArea {
                            clicked => { toggle(current-tab, item.id) }
                        }
                    }
                }
                if !item.hide: Text {
                    x: item.x + 16px;
                    text: item.text;
                }

            }
        }
    }
}
