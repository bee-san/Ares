use petgraph::Graph;
use petgraph::visit::Bfs;

pub struct Node {
    value: String,
    parent: Box<Option<T>>,
}

impl Node {
    pub fn new(value: String) -> Tree{
        self {
            value: value,
            parent: None,
        }
    }
}

impl Node {
    pub fn bfs(text: String){
        let mut graph = Graph::<_,()>::new();
        let start_node = Node::new();
        let a = graph.add_node(start_node);
        
        let mut bfs = Bfs::new(&graph, a);
        while let Some(nx) = bfs.next(&graph) {
            // we can access `graph` mutably here still
            let values = Node::perform_decoding(&nx.value);
            for decrypted_object in value{
                if decrypted_object.is_some(){
                    let child_node = Node {
                        value: decrypted_object.unwrap(),
                        parent: Some(nx.value),
                    };
                    graph.add_node(child_node);
                }
            }
        }
    }

    // Performs the decodings by getting all of the decoders
    // and calling `.run` which in turn loops through them and calls
    // `.crack()`.
    fn perform_decoding(text: &str) -> Vec<Option<String>>{
        let decoders = filter_and_get_decoders();
        decoders.run(text)
    }
}