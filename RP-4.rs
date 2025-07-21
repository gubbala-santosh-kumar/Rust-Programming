// Scope
fn main() {
    let mut globalValue = 100;
    {
        println!("{}",globalValue);
        let localValue = 200;      //----> Local variable creation
    }
    // println!("{}",localValue);     //----> This causes Error because local variable is using outside of the scope   
}

//Shadowing
fn main() {
    let global = 100;
    {
        let local = 200;
        let global = 300; // creating a variable with the same name which global variable has called shadowing
        println!("{}\n{}", local, global);
    }
    println!("{}",global);
}
