pub fn queue_pop_head(mut queue: Ptr<Queue>) -> QueueValue {
    let mut entry: Ptr<QueueEntry> = Default::default();
    let mut result: QueueValue = Default::default();

    if queue_is_empty(queue.cast()).as_bool() {
        return QUEUE_NULL!();
    }

    entry = queue.head.cast();
    queue.head = entry.next.cast();
    result = entry.data.cast();

    if (queue.head == NULL!()).as_bool() {
        queue.tail = NULL!();
    } else {
        queue.head.prev = NULL!();
    }

    c_free!(entry);

    return result.cast();
}
