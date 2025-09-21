pub fn slist_sort_internal(mut list: Ptr<Ptr<SListEntry>>, mut compare_func: SListCompareFunc) -> Ptr<SListEntry> {
    let mut pivot: Ptr<SListEntry> = Default::default();
    let mut rover: Ptr<SListEntry> = Default::default();
    let mut less_list: Ptr<SListEntry> = Default::default();
    let mut more_list: Ptr<SListEntry> = Default::default();
    let mut less_list_end: Ptr<SListEntry> = Default::default();
    let mut more_list_end: Ptr<SListEntry> = Default::default();

    if (*list == NULL!()).as_bool() || ((*list).next == NULL!()).as_bool() {
        return *list;
    }

    pivot = *list;

    less_list = NULL!();
    more_list = NULL!();
    rover = (*list).next;

    while (rover != NULL!()).as_bool() {
        let mut next: Ptr<SListEntry> = rover.next;

        if compare_func(rover.data.cast(), pivot.data.cast()) < 0 {
            rover.next = less_list;
            less_list = rover;
        } else {
            rover.next = more_list;
            more_list = rover;
        }

        rover = next;
    }

    less_list_end = slist_sort_internal(c_ref!(less_list).cast(), compare_func.cast());
    more_list_end = slist_sort_internal(c_ref!(more_list).cast(), compare_func.cast());

    *list = less_list;

    if (less_list == NULL!()).as_bool() {
        *list = pivot;
    } else {
        less_list_end.next = pivot;
    }

    pivot.next = more_list;

    if (more_list == NULL!()).as_bool() {
        return pivot;
    } else {
        return more_list_end;
    }
}
