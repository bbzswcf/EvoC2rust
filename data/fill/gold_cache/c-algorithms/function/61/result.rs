pub fn queue_is_empty(mut queue: Ptr<Queue>) -> i32 {
    return (queue.head == NULL!()).cast();
}
