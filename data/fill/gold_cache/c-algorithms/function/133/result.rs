pub fn set_register_free_function(mut set: Ptr<Set>, mut free_func: SetFreeFunc) {
    set.free_func = free_func.cast();
}
