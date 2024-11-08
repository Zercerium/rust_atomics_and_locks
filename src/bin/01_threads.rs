use std::thread;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello from the main thread.");
    t1.join().unwrap();
    t2.join().unwrap();

    let numbers = vec![1, 2, 3];

    thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();

    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();
    println!("average: {average}");
}

fn f() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
