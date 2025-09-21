pub fn queue_pop_tail(mut queue: Ptr<Queue>) -> QueueValue {
    let mut entry: Ptr<QueueEntry> = Default::default();
    let mut result: QueueValue = Default::default();

    if queue_is_empty(queue.cast()).as_bool() {
        return QUEUE_NULL!();
    }

    entry = queue.tail.cast();
    queue.tail = entry.prev.cast();
    result = entry.data.cast();

    if (queue.tail == NULL!()).as_bool() {
        queue.head = NULL!();
    } else {
        queue.tail.next = NULL!();
    }

    c_free!(entry);

    return result.cast();
}
