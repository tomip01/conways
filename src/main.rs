use conways::Conways;

mod conways;

fn main() {
    let mut conway = Conways::new(6, 6);
    conway.set_alive(1,0);
    conway.set_alive(1,1);
    conway.set_alive(1,2);

    conway.set_alive(5,5);
    conway.set_alive(5,4);
    conway.set_alive(4,4);
    conway.set_alive(4,5);

    println!("{}", conway);

    for _ in 0..10 {
        conway.tick();
        println!("{}", conway);
    }
}
