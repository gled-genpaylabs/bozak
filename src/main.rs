#[derive( Debug)]
struct Bozak<T>{

    button: Option<Vec<T>>,
    connected: bool,
    device: String,
}

impl<T>Bozak<T>{


    fn new(button:Option<Vec<T>>, connected: bool, device: String)-> Self {

       Self{
            button: button,
            connected: connected,
            device: device
      }
       

    }

    fn add_cue(&mut self, newValue: bool) -> bool {

         self.connected = newValue;
         self.connected

    }



}

impl<T> Default for Bozak<T> {

    fn default() ->Bozak<T>{
   
      Self{
            button: None,
            connected: false,
            device: "".to_string()
      }

    }

}

impl<T>Drop for Bozak<T> {

    fn drop(&mut self) {
    
        
    }

}

fn main(){

    let mut bozak1: Bozak<Vec<String>> = Bozak::new(None, false, "".to_string());

     bozak1.add_cue(true);

     println!("{:?}", bozak1);
  

}
