pub struct ContainerInfo {
    pub id :  u32;
    pub name : String; 
    pub image : String //url 
    pub status : String; //active, inactive, maintenance 
    pub state : String; //running, stopped, paused
    pub created : String; //timestamp
    pub ports : Vec<String>; //list of ports
    pub network : String; //network name
    pub ip_address : Option<String>; //optional for now
    pub cpu_usage : f32; //future
    pub memory_usage : f32; //future

} 
impl ContainerInfo {
    pub fn new(id: u32, name: &str, image: &str, status: &str, state: &str, created: &str, ports: Vec<String>, network: &str, ip_address: Option<String>, cpu_usage: f32, memory_usage: f32) -> Self {
        Self {
            id,
            name: name.to_string(),
            image: image.to_string(),
            status: status.to_string(),
            state: state.to_string(),
            created: created.to_string(),
            ports,
            network: network.to_string(),
            ip_address,
            cpu_usage,
            memory_usage,
        }
    }
}
