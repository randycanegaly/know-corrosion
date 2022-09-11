fn main() {
    println!("Starting enums play.");
    println!("A dog breed is: {:?}", DogBreed::Aussie);

    let dog = DogBreed::Aussie;
    //let color = dog.get_color();
    println!("Dog's color is {:?}", dog_color(dog));



    


}

fn dog_color(dog: DogBreed) -> Color {
    match dog {
        DogBreed::Newfoundland => Color::Unknown,
        DogBreed::Dachshund => Color::Unknown,
        DogBreed::Labrador => Color::Unknown,
        DogBreed::Aussie(coat) => coat
        }
}


//define some enumerations
#[derive(Debug)]//an annotation to allow new types to work in format strings
enum DogBreed {
    Newfoundland,
    Dachshund,
    Labrador,
    Aussie(Color)
}

//impl DogBreed {
    //fn get_Color(&self) {
        //println!("Called color");
        //self.Color
    //}
//}

#[derive(Debug)]//an annotation to allow new types to work in format strings
enum Color {
    Tri,
    BlueMerl,
    RedMerl,
    Unknown
}
