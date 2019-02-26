# Example: Assigning to custom properties

This minimal example shall demonstrate assigning a custom implemented property struct to a custom qml type, within qml.

The following things do currently not work:
1. binding the Slice to the Pie (see `src/gui.qml`, line 17)
2. binding the Slice to the Pie using an identifier,
3. implementing a getter (and the setter may be garbage?).

