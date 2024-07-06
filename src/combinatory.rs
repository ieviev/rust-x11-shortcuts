///  11010 -> [1, 3, 4]
pub fn get_ones_indexes(n: u32) -> Vec<u32> {
    let mut result = vec![];
    let mut n = n; // Clone to mut

    let mut index = 0;
    while n != 0 {
        if n & 1 == 1 {
            result.push(index);
        }

        n >>= 1;
        index += 1;
    }    

    result
}

/// [1, 3, 4], 1 -> 001000
pub fn map_indexes_combination(indexes: &Vec<u32>, combination: u32) -> u32 {
    let mut result = 0;

    let combination_indexes = get_ones_indexes(combination);

    for index in combination_indexes {
        match indexes.get(index as usize) {
            Some(v) => {
                result |= 1 << v
            }
            None => panic!("Something went wrong mapping a combination")
        };
    }

    result
}

pub fn total_combinations(indexes_len: usize) -> usize {
    1 << indexes_len
}