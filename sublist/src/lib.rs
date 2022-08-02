#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if is_sublist(_first_list, _second_list) && is_sublist(_second_list, _first_list){
        Comparison::Equal
    } else if is_sublist(_first_list, _second_list){
        Comparison::Sublist
    } else if is_sublist(_second_list, _first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    let l = _first_list.len();
    if l == 0 {
        return true;
    }
    if _second_list.len() < l {
        return false;
    }
    let mut iter = _second_list.windows(l);
    while true {
        let this = iter.next();
        if this.is_none(){
            break;
        }
        if this.unwrap() == _first_list {
            return true;
        }
    }
    false
}
