use wavefront::Obj;

#[test]
fn basic() {
    let obj = Obj::from_reader(include_bytes!("ship.obj") as &[u8]).unwrap();

    println!("{}", obj);
}
