initSidebarItems({"mod":[["cell","`Cell` is used for interior mutability of types that are lightweight and implement `Clone`"],["rc","`Rc` is a type that implements reference counting at runtime it's purpose is, unlike `Cell` and `RefCell`, not to supply interior mutability but to facilitate sharing a value in multiple places when lifetimes are not certain an `Rc<T>` does not give out means by which to mutate the `T` only ones to get reference to it"],["refcell","`RefCell` is a type that provides interior mutability at runtime for types that are less efficiently `Copy`"]]});