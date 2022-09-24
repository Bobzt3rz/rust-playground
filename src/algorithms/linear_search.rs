use crate::algorithms::HAYSTACK;

pub fn linear_search(haystack: &[i32], needle: &i32) -> bool{
    for n in haystack {
        if n == needle {
            return true;
        }
    }
    return false;
}

pub fn linear_search_test() {
    assert_eq!(linear_search(&HAYSTACK, &69420), true);
    assert_eq!(linear_search(&HAYSTACK, &420), true);
    assert_eq!(linear_search(&HAYSTACK, &1337), true);
    assert_eq!(linear_search(&HAYSTACK, &1), true);
    assert_eq!(linear_search(&HAYSTACK, &3), true);
    assert_eq!(linear_search(&HAYSTACK, &4), true);
    assert_eq!(linear_search(&HAYSTACK, &69), true);
    assert_eq!(linear_search(&HAYSTACK, &71), true);
    assert_eq!(linear_search(&HAYSTACK, &81), true);
    assert_eq!(linear_search(&HAYSTACK, &90), true);
    assert_eq!(linear_search(&HAYSTACK, &99), true);
    assert_eq!(linear_search(&HAYSTACK, &1337), true);
    assert_eq!(linear_search(&HAYSTACK, &69420), true);
    assert_eq!(linear_search(&HAYSTACK, &0), false);
    assert_eq!(linear_search(&HAYSTACK, &2), false);
    assert_eq!(linear_search(&HAYSTACK, &5), false);
    assert_eq!(linear_search(&HAYSTACK, &68), false);
    assert_eq!(linear_search(&HAYSTACK, &70), false);
    assert_eq!(linear_search(&HAYSTACK, &80), false);
    assert_eq!(linear_search(&HAYSTACK, &91), false);
    assert_eq!(linear_search(&HAYSTACK, &100), false);
    assert_eq!(linear_search(&HAYSTACK, &421), false);
    assert_eq!(linear_search(&HAYSTACK, &1336), false);
    assert_eq!(linear_search(&HAYSTACK, &69421), false);
} 