#[derive(Debug, Clone)]
pub enum InfrastructureType {
    Region,
    Cluster,
    Host,
    Container,
    Service,
}

#[derive(Debug, Clone)]
pub struct InfrastructureNode {
    pub id: u32,
    pub node_type: InfrastructureType,
    pub name: String,
    pub parent: Option<String>, // Using Option because a "Region" might not have a parent
    pub children: Vec<String>,
    pub status: String,
    pub health: u32,
    pub metrics: Vec<String>,
}

impl InfrastructureNode {
    // Associated function to create a new node easily
    pub fn new(
        id: u32,
        node_type: InfrastructureType,
        name: &str,
        parent: Option<&str>,
        status: &str,
        health: u32,
    ) -> Self {
        Self {
            id,
            node_type,
            name: name.to_string(),
            parent: parent.map(|p| p.to_string()), // Converts Option<&str> to Option<String>
            children: Vec::new(),                  // Start with an empty list of children
            status: status.to_string(),
            health,
            metrics: Vec::new(),                   // Start with empty metrics
        }
    }

    // Method to add a child to this node
    pub fn add_child(&mut self, child_name: &str) {
        self.children.push(child_name.to_string());
    }

    // Method to add a metric
    pub fn add_metric(&mut self, metric: &str) {
        self.metrics.push(metric.to_string());
    }
}