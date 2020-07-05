#![feature(core_intrinsics)]

fn main() {
    let distance = 100;
    let power = -2.345f32;
    let super_power = 22342.23423;
    let chara = 'a';
    let name = "alice"; // vec<u8>

    println!("{:.2}", distance);
    println!("{:+.2}", power);
    println!("{:x<10}", super_power);
    println!("{:?}",chara);
    print_type_of(&name);
}

fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}