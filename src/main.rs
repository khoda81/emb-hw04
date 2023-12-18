use std::{convert::Infallible, fmt::Display, sync::mpsc, thread};

fn f<T>(u: mpsc::Receiver<T>, v: mpsc::Receiver<T>, w: mpsc::SyncSender<T>) -> Infallible
where
    T: Clone + Display,
{
    loop {
        let t = u.recv().unwrap();
        w.send(t.clone()).unwrap();
        println!("F:  Sent {t} from `u`");

        let t = v.recv().unwrap();
        w.send(t.clone()).unwrap();
        println!("F:  Sent {t} from `v`");
    }
}

fn g<T>(u: mpsc::Receiver<T>, v: mpsc::SyncSender<T>, w: mpsc::SyncSender<T>) -> Infallible
where
    T: Clone + Display,
{
    loop {
        let t = u.recv().unwrap();
        v.send(t.clone()).unwrap();
        println!("G:  Sent {t} to `v`");

        let t = u.recv().unwrap();
        w.send(t.clone()).unwrap();
        println!("G:  Sent {t} to `w`");
    }
}

fn h<T>(x: T, u: mpsc::Receiver<T>, v: mpsc::SyncSender<T>) -> Infallible
where
    T: Clone + Display,
{
    v.send(x.clone()).unwrap();
    println!("H{x}: Sent {x}");
    loop {
        let t = u.recv().unwrap();
        v.send(t.clone()).unwrap();
        println!("H{x}: Sent {t}");
    }
}

fn main() {
    let (tx, rx) = mpsc::sync_channel(4);
    let (ty, ry) = mpsc::sync_channel(4);
    let (tz, rz) = mpsc::sync_channel(4);
    let (ts, rs) = mpsc::sync_channel(4);
    let (tt, rt) = mpsc::sync_channel(4);

    thread::spawn(move || f(ry, rz, tx));
    thread::spawn(move || g(rx, ts, tt));
    thread::spawn(move || h(0, rs, ty));
    thread::spawn(move || h(1, rt, tz));

    thread::park()
}
