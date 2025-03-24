
#[derive(Debug)]
enum Investiment{
    INTRADAY,
    SCALPING,
    SWING,
}

fn main(){
    let investiment = Investiment::INTRADAY;
    println!("{:?}", investiment);
}
