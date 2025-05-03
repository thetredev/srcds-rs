fn main() {
    println!("Getting Plugin instance from C++...");
    let plugin = srcds::Plugin::get_instance();

    println!("Calling Plugin::hello() from Rust:");
    plugin.hello(30);
}
