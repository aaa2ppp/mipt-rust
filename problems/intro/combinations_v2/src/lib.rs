#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();

    if k <= arr.len() {
        let mut buf: Vec<usize> = vec![0; k];
        let mut i = 0;
        let mut j = 0;
        let n = arr.len() - k;

        loop {
            while i < buf.len() && j <= n + i {
                buf[i] = j;
                j += 1;
                i += 1;
            }
            if i == buf.len() {
                res.push(buf.iter().map(|j| arr[*j]).collect());
            }
            if i == 0 {
                break;
            }
            i -= 1;
            j = buf[i] + 1;
        }
    }

    res
}
