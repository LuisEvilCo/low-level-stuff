mod my;
mod primitives;

fn main() {
    println!("Hello, world!");
    file_hierarchy();
    primitives();

}

fn file_hierarchy() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}

fn function() {
    println!("called `function()`");

}

fn primitives() {
    primitives::main();
}
