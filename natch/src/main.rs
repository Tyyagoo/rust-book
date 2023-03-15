enum Nat {
    Zero,
    Succ(Box<Nat>),
}

fn main() {
    let zero = Nat::new(0);
    println!("{}", zero.show());
    println!("{}", zero.dec().show());

    let one = Nat::new(1);
    println!("{}", one.show());

    let two = one.inc();
    println!("{}", two.show());

    let one = two.dec();
    println!("{}", one.show());

    let thousand = Nat::new(1000);
    println!("{}", thousand.show());

    let five_hundred = Nat::new(501);
    let x = thousand.add(&five_hundred);
    println!("{}", x.show());
    println!("{}", x.sub(&five_hundred).show());

    println!(
        "10 - 1000 is: {}",
        Nat::sub(Nat::new(10), &Nat::new(1000)).show()
    );
}

impl Nat {
    fn new(n: u64) -> Self {
        let mut nat = Nat::Zero;

        for _ in 1..=n {
            nat = nat.inc();
        }

        nat
    }

    fn inc(self) -> Self {
        Nat::Succ(Box::new(self))
    }

    fn dec(self) -> Self {
        match self {
            Nat::Zero => Nat::Zero,
            Nat::Succ(mut_box) => *mut_box,
        }
    }

    fn add(mut self, other: &Self) -> Self {
        let mut curr = other;

        loop {
            match curr {
                Nat::Zero => break,
                Nat::Succ(prev) => {
                    self = self.inc();
                    curr = prev;
                }
            }
        }

        self
    }

    fn sub(mut self, other: &Self) -> Self {
        let mut curr = other;

        loop {
            match curr {
                Nat::Zero => break,
                Nat::Succ(prev) => {
                    self = self.dec();
                    curr = prev;
                }
            }
        }

        self
    }

    fn show(&self) -> u64 {
        let mut n: u64 = 0;
        let mut curr = self;

        loop {
            match curr {
                Nat::Zero => break,
                Nat::Succ(prev) => {
                    n += 1;
                    curr = prev;
                }
            }
        }
        n
    }
}
