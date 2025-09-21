pub fn queue_new() -> Ptr<Queue> {
    let mut queue: Ptr<Queue> = c_malloc!(c_sizeof!(Queue));

    if (queue == NULL!()).as_bool() {
        return NULL!();
    }

    queue.head = NULL!();
    queue.tail = NULL!();

    return queue.cast();
}
