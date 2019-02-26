use qmetaobject::*;
use std::cell::RefCell;
use std::ffi::CStr;

#[derive(Default, QObject)]
struct Slice {
    base: qt_base_class!(trait QQuickItem),
}

#[derive(Default, QObject)]
struct Pie {
    base: qt_base_class!(trait QQuickItem),
    slice: qt_property!(RefCell<Slice>; /*READ get_slice WRITE set_slice*/),
}

impl Pie {
    //fn get_slice(&self) -> RefCell<Slice> {
    //    self.slice // how?
    //}
    //fn set_slice(&mut self, s: RefCell<Slice>) {
    //    self.slice = s;
    //}
}
impl QQuickItem for Slice {}
impl QQuickItem for Pie {}

qrc!(gui,
    "gui" {
        "src/gui.qml" as "main.qml",
    },
);

fn register_all_types() {
    qml_register_type::<Slice>(
        CStr::from_bytes_with_nul(b"RustCode\0").unwrap(),
        1,
        0,
        CStr::from_bytes_with_nul(b"Slice\0").unwrap(),
    );
    qml_register_type::<Pie>(
        CStr::from_bytes_with_nul(b"RustCode\0").unwrap(),
        1,
        0,
        CStr::from_bytes_with_nul(b"Pie\0").unwrap(),
    );
}

fn main() {
    gui();
    register_all_types();
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/gui/main.qml".into());
    engine.exec();
}
