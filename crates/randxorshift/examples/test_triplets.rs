/*

let triplet8=[(1,1,2),(1,1,3),(1,7,3),(1,7,6),(1,7,7),(2,1,1),(2,5,5),(3,1,1),(3,1,5),(3,5,4),(3,5,5),(3,5,7),(3,7,1),(4,5,3),(5,1,3),(5,3,6),(5,3,7),(5,5,2),(5,5,3),(6,3,5),(6,7,1),(7,3,5),(7,5,3),(7,7,1)];
let triplet16=[(1,1,14),(1,1,15),(1,5,2),(1,7,4),(1,7,11),(1,11,3),(1,15,6),(1,15,7),(2,5,1),(2,5,13),(2,5,15),(2,7,13),(2,7,15),(3,1,12),(3,1,15),(3,5,11),(3,11,1),(3,11,11),(3,13,9),(4,3,7),(4,7,1),(4,11,11),(5,7,14),(5,9,8),(5,11,6),(5,11,11),(6,7,13),(6,11,5),(6,15,1),(7,1,11),(7,3,4),(7,9,8),(7,9,13),(7,15,1),(8,9,5),(8,9,7),(9,7,13),(9,13,3),(11,1,7),(11,3,13),(11,5,3),(11,7,1),(11,11,3),(11,11,4),(11,11,5),(12,1,3),(12,3,13),(13,3,11),(13,3,12),(13,5,2),(13,7,2),(13,7,6),(13,7,9),(13,9,7),(14,1,1),(14,7,5),(15,1,1),(15,1,3),(15,5,2),(15,7,2)];
let triplet32=...

*/
fn test_triplets(w: u8, a: u8, b: u8, c: u8) -> bool {
    let max: u128 = match w {
        8 => u8::MAX as u128,
        16 => u16::MAX as u128,
        32 => u32::MAX as u128,
        64 => u64::MAX as u128,
        128 => u128::MAX,
        _ => todo!(),
    };
    let mut state: u128 = 1;
    let mut visited = vec![false; 1 << w as usize];
    let mut count: u128 = 0;

    let and = ((1 << w as usize) - 1) & max;
    loop {
        state ^= state << a;
        state &= and;
        state ^= state >> b;
        state &= and;
        state ^= state << c;
        state &= and;

        if visited[state as usize] {
            break;
        }

        visited[state as usize] = true;
        count += 1;
    }
    count == (1 << w as usize) - 1
}

/// w: register size
fn run(w: u8) {
    print!("let triplet{}=[", w);
    for a in 1..w {
        for b in 1..w {
            for c in 1..w {
                if test_triplets(w, a, b, c) {
                    print!("({},{},{}),", a, b, c);
                }
            }
        }
    }
    println!("];");
}

fn main() {
    println!("========== 8 bits valid triplet (a,b,c) ==========");
    run(8);
    println!("========== 16 bits valid triplet (a,b,c) ==========");
    run(16);
    println!("========== 32 bits valid triplet (a,b,c) ==========");
    run(32);
    println!("========== 64 bits valid triplet (a,b,c) ==========");
    run(64);
    println!("========== 128 bits valid triplet (a,b,c) ==========");
    run(128);
}
