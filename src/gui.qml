import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import RustCode 1.0;

ApplicationWindow {
    id: window
    visible: true
    width: 500
    height: 500
    title: "QML Demo"

    Slice {id: s}  // works
    Pie {}    // works

    // does not work:
    Pie {
        slice: s
    }

    // does not work:
    Pie {
        slice: Slice {}
    }
}