pub fn queue_peek_tail(mut queue: Ptr<Queue>) -> QueueValue {
    if queue_is_empty(queue.cast()).as_bool() {
        return QUEUE_NULL!();
    } else {
        return queue.tail.data.cast();
    }
}
