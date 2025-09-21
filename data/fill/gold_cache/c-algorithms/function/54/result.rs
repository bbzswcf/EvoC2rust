pub fn queue_free(mut queue: Ptr<Queue>) {
    while !(queue_is_empty(queue.cast()).as_bool()) {
        queue_pop_head(queue.cast());
    }
    c_free!(queue);
}
