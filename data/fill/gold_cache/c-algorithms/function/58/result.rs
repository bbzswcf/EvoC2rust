pub fn queue_push_tail(mut queue: Ptr<Queue>, mut data: QueueValue) -> i32 {
    let mut new_entry: Ptr<QueueEntry>;

    new_entry = c_malloc!(c_sizeof!(QueueEntry));

    if (new_entry == NULL!()).as_bool() {
        return 0;
    }

    new_entry.data = data.cast();
    new_entry.prev = queue.tail.cast();
    new_entry.next = NULL!();

    if (queue.tail == NULL!()).as_bool() {
        queue.head = new_entry.cast();
        queue.tail = new_entry.cast();
    } else {
        queue.tail.next = new_entry.cast();
        queue.tail = new_entry.cast();
    }

    return 1;
}
