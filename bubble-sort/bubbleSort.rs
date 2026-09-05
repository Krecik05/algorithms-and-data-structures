struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        SimpleRng { state: seed }
    }

    // Generuje losowy u32
    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.state >> 32) as u32
    }

    // Losuje liczbę z przedziału [min, max]
    fn gen_range(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min + 1) as u32;
        min + (self.next_u32() % range) as i32
    }
}

fn fill_with_random(array: &mut Vec<i32>, min: i32, max: i32) {
    // Żeby otrzymać "nowe" liczby losowe trzeba zmienić seed na dowolnie inną liczbe.
    let mut rng = SimpleRng::new(34); 

    let size = rng.gen_range(10, 20);

    for _ in 0..size {
        let losowa = rng.gen_range(min, max); 
        array.push(losowa);
    }
}

fn bubble_sort_vec(vec: &mut Vec<i32>) {
    let n = vec.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}


fn main() {
    let mut tablica: Vec<i32> = Vec::new();

    fill_with_random(&mut tablica, 1, 100);
    println!("Przed sortowaniem: {:?}", tablica);

    bubble_sort_vec(&mut tablica);

    println!("Po sortowaniu: {:?}", tablica);
}