//! The search algorithm decides what encryptions to do next
//! And also runs the decryption modules
//! Click here to find out more:
//! https://broadleaf-angora-7db.notion.site/Search-Nodes-Edges-What-should-they-look-like-b74c43ca7ac341a1a5cfdbeb84a7eef0



use crate::filtration_system::filter_and_get_decoders;


/*

use std::{collections::VecDeque};
pub struct Tree <'a> {
    // Wrap in a box because
    // https://doc.rust-lang.org/error-index.html#E0072
    parent: &'a Box<Option<Tree<'a>>>,
    value: String
}

impl Tree<'_> {
    pub fn new(value: String) -> Tree<'static> {
        self {
            // The root node does not have a parent
            // Therefore this becomes None.
            parent: &Box::new(None),
            value: value,
        }
    }
}

impl Tree<'_> {
    /// Performs the search algorithm.
    ///
    /// When we perform the decryptions, we will get a vector of Some<String>
    /// We need to loop through these and determine:
    /// 1. Did we reach our exit condition?
    /// 2. If not, create new nodes out of them and add them to the queue.
    /// We can return an Option? An Enum? And then match on that
    /// So if we return CrackSuccess we return
    /// Else if we return an array, we add it to the children and go again.
    pub fn bfs(self) -> Option::<String>{
        let mut q = VecDeque::new();
        /*
        TODO: I am not sure where self is coming from ?
        We need to put _something_ on the queue so we put self, which is a Tree
        */
        q.push_back(self);
        
        while let Some(t) = q.pop_front() {
            //TODO: why do we have Some(t) here? This should skip the empty nodes, hopefully?

            // This returns a CrackObject
            let value = Tree::perform_decoding(&t.value); // This gets us all the values from the decoders :pray:
            let parent = &Box::new(Some(t));

            for decrypted_object in value {
                if decrypted_object.is_some() {
                    /*if decrypted_object.unwrap().success {
                        // if we were successful, return idk what yet just a child fuck it
                        return vec![decrypted_object];
                    }*/
                    // TODO: we only return vec of strings not crack objects
                    // so we should change to crack objects in perform_decoding
                    // to allow us to do more :) <3

                    // else add to the queue our new child node and continue
                    let tree_node = Tree {parent: parent, value: decrypted_object?};
                    q.push_back(tree_node);
                }
            }

        }
        None
    }
    // Performs the decodings by getting all of the decoders
    // and calling `.run` which in turn loops through them and calls
    // `.crack()`.
    fn perform_decoding(text: &str) -> Vec<Option<String>>{
        let decoders = filter_and_get_decoders();
        decoders.run(text)
    }
}
*/

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

fn search() {

}
