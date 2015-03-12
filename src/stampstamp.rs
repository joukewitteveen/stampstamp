// generate extremal stamp chains
#![feature(collections, env)]
use std::cmp::Ordering;
use std::collections::{BitVec, LinkedList};
use std::env::args;

static START: usize = 2;


fn check(xs: &BitVec, n: usize) -> bool {
    (2..(n / 2 + 1)).find(|&i| xs[i] && xs[n - i]).is_some()
}

fn check_ext(xs: &BitVec, n: usize) -> bool {
    ((n - xs.len() + 1)..(n / 2 + 1)).find(|&i| xs[i] && xs[n - i]).is_some()
}

fn search(xs: BitVec, acc: usize, mut m: usize) -> (usize, LinkedList<BitVec>) {
    if m >= xs.len() {
        let xsize = xs.iter().skip(START).filter(|x| *x).count();
        let mut xss = LinkedList::new();
        xss.push_back(xs);
        return (xsize, xss);
    }
    let mut ys = xs.clone();
    ys.set(m, false);
    m += 1;
    //let mut xsearch = std::sync::Future::spawn(move || {search(xs, acc + 1, m)});
    if !check(&ys, m) {
        //return = xsearch.get();
        return search(xs, acc + 1, m);
    }
    let (ysize, mut yss) = search(ys, acc, m);
    if ysize <= acc {
        return (ysize, yss);
    }
    //let (xsize, mut xss) = xsearch.get();
    let (xsize, mut xss) = search(xs, acc + 1, m);
    match xsize.cmp(&ysize) {
        Ordering::Less    => (xsize, xss),
        Ordering::Greater => (ysize, yss),
        Ordering::Equal   => {
            xss.append(&mut yss);
            (xsize, xss)
        }
    }
}

fn main() {
    let n = args().nth(1).expect("Usage: rset NUM").parse().unwrap();
    let (xsize, xss) = search(BitVec::from_elem(n, true), 1, START + 1);
    for xs in xss.iter().filter(|&xs| xs[START]) {
        print!("{}", START);
        for i in ((START + 1)..xs.len()).filter(|&i| xs[i]) {
            print!(", {:>3}", i);
        }
        println!("  (valid up to {})",
                 (n..).find(|&i| !check_ext(&xs, i + 1)).unwrap());
        //println!("({:?})", xs);
    }
    println!("{} collection(s) of {} numbers", xss.len(), xsize);
}


// vim: et sw=4:
