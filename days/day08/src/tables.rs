use crate::pattern::Pattern;

pub const NUMBER_TO_PATTERN: [Pattern; 10] = [
    Pattern::from_char_list(&['a', 'b', 'c', 'e', 'f', 'g']),
    Pattern::from_char_list(&['c', 'f']),
    Pattern::from_char_list(&['a', 'c', 'd', 'e', 'g']),
    Pattern::from_char_list(&['a', 'c', 'd', 'f', 'g']),
    Pattern::from_char_list(&['b', 'c', 'd', 'f']),
    Pattern::from_char_list(&['a', 'b', 'd', 'f', 'g']),
    Pattern::from_char_list(&['a', 'b', 'd', 'e', 'f', 'g']),
    Pattern::from_char_list(&['a', 'c', 'f']),
    Pattern::from_char_list(&['a', 'b', 'c', 'd', 'e', 'f', 'g']),
    Pattern::from_char_list(&['a', 'b', 'c', 'd', 'f', 'g']),
];

pub const LEN_TO_NUMS: &[&[u8]] = &[&[], &[], &[1], &[7], &[4], &[2, 3, 5], &[0, 6, 9], &[8]];

pub const LEN_TO_UNION: [Pattern; 8] = [
    Pattern::from_numbers_union(LEN_TO_NUMS[0]),
    Pattern::from_numbers_union(LEN_TO_NUMS[1]),
    Pattern::from_numbers_union(LEN_TO_NUMS[2]),
    Pattern::from_numbers_union(LEN_TO_NUMS[3]),
    Pattern::from_numbers_union(LEN_TO_NUMS[4]),
    Pattern::from_numbers_union(LEN_TO_NUMS[5]),
    Pattern::from_numbers_union(LEN_TO_NUMS[6]),
    Pattern::from_numbers_union(LEN_TO_NUMS[7]),
];

pub const LEN_TO_INTERSECTION: [Pattern; 8] = [
    Pattern::from_numbers_intersection(LEN_TO_NUMS[0]),
    Pattern::from_numbers_intersection(LEN_TO_NUMS[1]),
    Pattern::from_numbers_intersection(LEN_TO_NUMS[2]),
    Pattern::from_numbers_intersection(LEN_TO_NUMS[3]),
    Pattern::from_numbers_intersection(LEN_TO_NUMS[4]),
    Pattern::from_numbers_intersection(LEN_TO_NUMS[5]),
    Pattern::from_numbers_intersection(LEN_TO_NUMS[6]),
    Pattern::from_numbers_intersection(LEN_TO_NUMS[7]),
];
