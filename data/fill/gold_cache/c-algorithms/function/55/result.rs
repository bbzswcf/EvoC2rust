pub fn queue_push_head(mut queue: Ptr<Queue>, mut data: QueueValue) -> i32 {
    let mut new_entry: Ptr<QueueEntry>;

    new_entry = c_malloc!(c_sizeof!(QueueEntry));

    if (new_entry == NULL!()).as_bool() {
        return 0;
    }

    new_entry.data = data.cast();
    new_entry.prev = NULL!();
    new_entry.next = queue.head.cast();

    if (queue.head == NULL!()).as_bool() {
        queue.head = new_entry.cast();
        queue.tail = new_entry.cast();
    } else {
        queue.head.prev = new_entry.cast();
        queue.head = new_entry.cast();
    }

    return 1;
}
