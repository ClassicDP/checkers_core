use std::mem::swap;
use rand::Rng;


pub struct ListP {
    v: Vec<i32>,
    tmp: Vec<i32>,
}

impl ListP {
    pub fn new(n: usize) -> ListP {
        let mut v = Vec::new();
        for _i in 0..n { v.push(rand::thread_rng().gen_range(0..100000)); }
        ListP { v, tmp: vec![0; n] }
    }

    pub fn quick_sort(xs: &mut [i32], tmp: &mut [i32]) {
        if xs.len() <= 1 { return; }
        let len = xs.len();
        if xs.len() == 2 && xs[0] > xs[1] { xs.swap(0, 1); }
        let mid = xs.len() / 2;
        let (lo, hi) = xs.split_at_mut(mid);
        let (t1, t2) = tmp.split_at_mut(mid);
        rayon::join(|| ListP::quick_sort(lo, t1), || ListP::quick_sort(hi, t2));
        let mut j = 0;
        let mut k = 0;
        for i in 0..len {
            if j < lo.len() && (k == hi.len() || lo[j] < hi[k]) {
                tmp[i] = lo[j];
                j += 1;
            } else {
                tmp[i] = hi[k];
                k += 1;
            }
        }
        xs[0..len].copy_from_slice(tmp);
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

use wasm_bindgen::prelude::*;
use crate::log;

#[wasm_bindgen]
pub fn test_q () {
    let n: usize = 10000000;
    let mut l = ListP::new(n);
    use std::time::Instant;
    let t = Instant::now();
    ListP::quick_sort(&mut l.v, &mut l.tmp);
    log(&format!("par {:.2?} \n", t.elapsed()));

    let mut l = ListP::new(n);
    let t = Instant::now();
    l.v.sort();
    log(&format!("lib {:.2?}", t.elapsed()));
}


#[cfg(test)]
mod tests {
    use std::time::Instant;
    use js_sys::Math::min;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::MThreadTest::ListP;

    #[test]
    pub fn min_test_parallel() {
        let n: usize = 10000000;
        let mut l = ListP::new(n);
        use std::time::Instant;
        let t = Instant::now();
        ListP::quick_sort(&mut l.v, &mut l.tmp);
        print!("par {:.2?} \n", t.elapsed());

        let mut l = ListP::new(n);
        let t = Instant::now();
        l.v.sort();
        print!("lib {:.2?}", t.elapsed());
    }

}
