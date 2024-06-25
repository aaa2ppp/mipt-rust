#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();

    if k <= arr.len() {
        let mut lug = Lug {
            arr,
            buf: &mut vec![0; k][..],
            res: &mut res,
        };

        dfs(&mut lug, 0, 0, arr.len() - k);
    }

    res
}

struct Lug<'a> {
    arr: &'a [i32],
    buf: &'a mut [usize],
    res: &'a mut Vec<Vec<i32>>,
}

fn dfs(lug: &mut Lug, i: usize, start: usize, end: usize) {
    if i == lug.buf.len() {
        lug.res.push(lug.buf.iter().map(|j| lug.arr[*j]).collect());
        return;
    }

    for j in start..=end {
        lug.buf[i] = j;
        dfs(lug, i + 1, j + 1, end + 1)
    }
}
