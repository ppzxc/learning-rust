mod spawn;
mod threads;
mod passing;
mod mtx;
mod referenceCount;

fn main() {
    spawn::spawn();
    threads::t();
    passing::passing();
    mtx::mtx();
    referenceCount::referenceCount();
}
