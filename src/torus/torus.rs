use rand::Rng; // bring the trait into scope


#[derive(Debug, Clone, Copy)]
pub struct TorusElement {
    pub value: f32,
}

impl TorusElement {
    fn new(torus_element: f32) -> TorusElement {

        let check = (torus_element > 0.0) && (torus_element < 1.0);
        if check {
            TorusElement { value: torus_element}
        } else {
            //perform implicit conversion mod 1 so that its in the format R/Z mod 1
            TorusElement { value: ((torus_element).rem_euclid(1.0)) }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Torus {
    pub set: Vec<TorusElement>,
    element_position: usize,
}

impl Torus {
    //create a new torus group of size n, default elements zero
    pub fn new(n: usize) -> Self {
        let set: Vec<TorusElement> = (0..n)
            .map(|_| TorusElement { value: 0 as f32})
            .collect();
        Torus { set, element_position: 0 }
    }

    //create a new torus group of size n of random elements 
    pub fn gen_random(n: usize) -> Self {
        let mut rng = rand::rng();
        let set: Vec<TorusElement> = (0..n)
            .map(|_| TorusElement { value: rng.random_range(0.0..1.0) as f32})
            .collect();
        Torus { set, element_position: 0 }
    }

    //Expand torus object with a new torus element, perform implicit conversion if supplied value is not in the required torus range (0,1] 
    pub fn push(&mut self, new_torus_element: f32) {

        //perform implicit conversion if supplied torus value is not in the range 0,1
        let new_element = TorusElement::new(new_torus_element);
        self.set.push(new_element);
        self.element_position = self.set.len() -1;
    }
}

impl Iterator for Torus {
    type Item = TorusElement;

    fn next(&mut self) -> Option<Self::Item> {

        if self.element_position < self.set.len() {
            let item = self.set[self.element_position];
            self.element_position += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Torus {
    type Item = &'a TorusElement;
    type IntoIter = std::slice::Iter<'a, TorusElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.iter()
    }
}
