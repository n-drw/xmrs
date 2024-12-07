use alloc::vec::Vec;

pub fn parse_orders(orders: &[u8]) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();
    let mut current_group: Vec<usize> = Vec::new();

    for order in orders {
        match *order {
            254 | 255 => {
                if !current_group.is_empty() {
                    result.push(current_group);
                    current_group = Vec::new();
                }
            }
            o => {
                current_group.push(o as usize);
            }
        }
    }

    if !current_group.is_empty() {
        result.push(current_group);
    }

    result
}
