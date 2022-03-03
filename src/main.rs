/** 
 * バブルソートアルゴリズムを Rust で実装してください。
 *
 * 仕様
 * -1000 以上 1000 以下のランダムな整数を100回発生させ、その整数を昇順に並び替える。
 */

use rand::Rng;

fn main() {
    println!("Bubble Sort!");

    let min = -1000;
    let max = 1000;
    let num = 10;
    let rand_vec = gen_rand_vec(min, max, num);

    let sort_vec = bubble_sort_vec(rand_vec);
    
    print_vec(sort_vec);
}

/**
 * gen_rand_vec
 * 
 * : ランダムな整数を発生させ、ベクタ配列として取得します。
 * : 引数で整数の範囲、回数を指定します。
 * 
 * * `min` - 範囲の下限
 * * `max` - 範囲の上限
 * * `num` - 回数
 */
fn gen_rand_vec (min: i32, max: i32, num: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..(num + 1)).map(|_| rng.gen_range(min, max)).collect()
}

/**
 * bubble_sort_vec
 * 
 * : ベクタ配列をバブルソートします。
 * 
 * * `src` - 対象のベクタ配列
 */
fn bubble_sort_vec (src: Vec<i32>) -> Vec<i32> {
    let mut dest = src.clone();
    for i in 1..dest.len() {
        let mut is_swapped = false;
        for j in 0..(dest.len() - i) {
            if dest[j + 1] < dest[j] {
                dest.swap(j, j + 1);
                is_swapped = true;
            }
        }
        if !is_swapped { break };
    }
    dest
}

/**
 * print_vec
 * 
 * : ベクタ配列を出力します。
 * 
 * * `vec` - 対象のベクタ配列
 */
fn print_vec (vec: Vec<i32>) {
    for v in vec.iter() {
        println!("{}", v);
    }
}