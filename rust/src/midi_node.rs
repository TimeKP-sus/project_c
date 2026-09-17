

use godot::classes::Node2D;
use godot::{
    classes::{INode2D}, obj::{Base}, prelude::{GodotClass, godot_api}
};


#[derive(GodotClass)]
#[class(init, base=Node2D)] 
pub struct MidiNode {
    #[base]
    base: Base<Node2D>,
    
}

impl MidiNode {

}

#[godot_api]
impl INode2D for MidiNode {
    fn ready(&mut self) {
        
    }
    
}
