use conways::Conways;

mod conways;

fn main() {
    let mut conway = Conways::new(5, 5);
    conway.set_alive(1,0);
    conway.set_alive(1,1);
    conway.set_alive(1,2);
    println!("{}", conway);
    conway.tick();
    println!("{}", conway);
}
