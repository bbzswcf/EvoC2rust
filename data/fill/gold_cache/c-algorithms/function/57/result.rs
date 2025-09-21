pub fn queue_peek_head(mut queue: Ptr<Queue>) -> QueueValue {
    if queue_is_empty(queue.cast()).as_bool() {
        return QUEUE_NULL!();
    } else {
        return queue.head.data.cast();
    }
}
