#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if contains_sublist(first_list, second_list) {
        Comparison::Superlist
    } else if contains_sublist(second_list, first_list) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

fn contains_sublist<T: PartialEq>(bigger_list: &[T], smaller_list: &[T]) -> bool {
    if bigger_list.is_empty() {
        false
    } else if bigger_list.starts_with(&smaller_list) {
        true
    } else {
        contains_sublist(&bigger_list[1..], smaller_list)
    }
}
