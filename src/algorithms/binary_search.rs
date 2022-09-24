use crate::algorithms::HAYSTACK;

pub fn binary_search(haystack: &[i32], needle: &i32) -> bool {
    let mut low: usize = 0;
    let mut high = haystack.len() - 1;
    let mut mid: usize = (high - low)/2;
    loop {
        if needle == &haystack[mid]{
            break true;
        } 
        if mid == low || mid == high || high <= low {break false;}
        if needle > &haystack[mid] {
            low = mid;
            mid += if(high-low)/2 == 0 {1} else {(high-low)/2};
        } 
        if needle < &haystack[mid] {
            high = mid;
            mid -= if(high-low)/2 == 0 {1} else {(high-low)/2};
        }
    }
}   

pub fn binary_search_test(){
    assert_eq!(binary_search(&HAYSTACK, &69420), true);
    assert_eq!(binary_search(&HAYSTACK, &420), true);
    assert_eq!(binary_search(&HAYSTACK, &1337), true);
    assert_eq!(binary_search(&HAYSTACK, &1), true);
    assert_eq!(binary_search(&HAYSTACK, &3), true);
    assert_eq!(binary_search(&HAYSTACK, &4), true);
    assert_eq!(binary_search(&HAYSTACK, &69), true);
    assert_eq!(binary_search(&HAYSTACK, &71), true);
    assert_eq!(binary_search(&HAYSTACK, &81), true);
    assert_eq!(binary_search(&HAYSTACK, &90), true);
    assert_eq!(binary_search(&HAYSTACK, &99), true);
    assert_eq!(binary_search(&HAYSTACK, &1337), true);
    assert_eq!(binary_search(&HAYSTACK, &69420), true);
    assert_eq!(binary_search(&HAYSTACK, &0), false);
    assert_eq!(binary_search(&HAYSTACK, &2), false);
    assert_eq!(binary_search(&HAYSTACK, &5), false);
    assert_eq!(binary_search(&HAYSTACK, &68), false);
    assert_eq!(binary_search(&HAYSTACK, &70), false);
    assert_eq!(binary_search(&HAYSTACK, &80), false);
    assert_eq!(binary_search(&HAYSTACK, &91), false);
    assert_eq!(binary_search(&HAYSTACK, &100), false);
    assert_eq!(binary_search(&HAYSTACK, &421), false);
    assert_eq!(binary_search(&HAYSTACK, &1336), false);
    assert_eq!(binary_search(&HAYSTACK, &69421), false);
}