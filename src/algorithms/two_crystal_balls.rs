use rand::Rng;

pub fn two_crystal_balls(break_array: &[bool]) -> i32{
    let len = break_array.len();
    let mut prev: usize = 0;
    let mut idx = (len as f64).sqrt() as usize;
    let mut traverse = false;
    loop {
        if traverse == true {
            let out = loop{
                if break_array[prev] == false {
                    if prev == len - 1 {
                        break -1
                    }
                    prev += 1;
                } else {
                    break prev as i32;
                }
            };
            break out as i32;
        } else {
            if idx * 2 > len -1 || break_array[idx] == true {
                traverse = true
            } else {
                prev = idx;
                idx += idx;
            }
        }
    }
}

pub fn two_crystal_balls_test(){
    const ARRAYSIZE: usize = 10000;
    let idx = rand::thread_rng().gen_range(0..=ARRAYSIZE);
    let mut break_array: [bool;ARRAYSIZE] = [false;ARRAYSIZE];
    let out = two_crystal_balls(&break_array);
    assert_eq!(-1, out);
    for n in idx..ARRAYSIZE{
        break_array[n] = true;
    }
    let out = two_crystal_balls(&break_array);
    assert_eq!(idx as i32, out)
}