use Unsafe_OneShot_Channel::Channel;
use std::thread;


fn main() {
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello world");
            t.unpark();
        });
        while !channel.is_ready() {
            thread::park()
        }
        assert_eq!(channel.receive(), "hello world");
    });
    println!("this ran");
}
