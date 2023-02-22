use std::mem::swap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

pub struct ListP {
    v: Vec<i32>,
}

impl ListP {
    pub fn new(n: i32) -> ListP {
        let mut v = Vec::new();
        for _i in 0..n { v.push(rand::thread_rng().gen_range(0..100000)); }
        ListP { v }
    }

    pub fn quick_sort(xs: &mut [i32]) {
        if xs.len() <= 1 { return; }
        if xs.len() == 2 && xs[0] > xs[1] { xs.swap(0, 1); }
        let mid = xs.len() / 2;
        let (lo, hi) = xs.split_at_mut(mid);
        rayon::join(|| ListP::quick_sort(lo), || ListP::quick_sort(hi));
        let mut i: usize = 0;
        while i < hi.len() {
            if i < lo.len() && lo[i] > hi[i] { swap(&mut lo[i], &mut hi[i]); } else { i += 1; }
        }
    }

    pub fn max(&mut self, th_cnt: i8) -> i32 {
        let len = self.v.len();
        let part_cnt = len / th_cnt as usize;
        let mut v: &mut [i32] = &mut self.v;
        let mut v0 = Vec::new();
        loop {
            let vx = v.split_at_mut(v.len() / 2);
            v0.push(vx.0);
            v = vx.1;
            if v.len() == 0 { break; }
        }
        let mut lm: Vec<_> = Vec::new();
        crossbeam::scope(|scope| {
            for v in v0 {
                let x = scope.spawn(move |_| {
                    let max = v.iter().max();
                    max
                }).join();
                lm.push(x.unwrap().unwrap());
            }
        }).expect("TODO: panic message");
        *lm.iter().map(|x| *x).max().unwrap()
    }
}


fn criterion_benchmark(c: &mut Criterion) {
    let mut l = ListP::new(100);
    // c.bench_function("parallel", |b| b.iter(|| l.max(8)));
    // c.bench_function("iterator", |b| b.iter(|| l.v.iter().max()));
    c.bench_function("q_sort", |b| b.iter(|| ListP::quick_sort(&mut l.v)));
    print!("{:?}", &l.v);
    let mut l = ListP::new(100);
    c.bench_function("sort", |b| b.iter(|| l.v.sort()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::ListP;

    #[test]
    fn min_test_parallel() {
        let mut l = ListP::new(100000);
        ListP::quick_sort(&mut l.v);
    }
}
