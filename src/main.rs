mod plot;

fn main() {
    let mut count = 0;
    const EPS: f64 = 0.0000001;
    let mut x = count as f64 * EPS;
    let mut plot = Vec::new();
    while x < 1.0 {
        let m: usize = find_factor(x, 100000);
        plot.push((x, m));
        x = count as f64 * EPS;
        count += 1;
    }

    plot::plot(&plot).unwrap();
}

fn find_factor(x: f64, limit: usize) -> usize {
    let mut n = 1;
    while !is_whole(n as f64 * x) && n < limit {
        n += 1
    }
    n
}

#[inline]
fn is_whole(x: f64) -> bool {
    x == (x as u64) as f64
}

#[allow(unused)]
#[inline]
fn is_whole_fract(x: f64) -> bool {
    x.fract() == 0.0
}
