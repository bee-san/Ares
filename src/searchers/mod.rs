//! The search algorithm decides what encryptions to do next
//! And also runs the decryption modules
//! Click here to find out more:
//! https://broadleaf-angora-7db.notion.site/Search-Nodes-Edges-What-should-they-look-like-b74c43ca7ac341a1a5cfdbeb84a7eef0
use std::collections::VecDeque;
use crate::searchers;
use crate::filtration_system::filter_and_get_decoders;

pub struct Tree {
    // Wrap in a box because
    // https://doc.rust-lang.org/error-index.html#E0072
    parent: Box<Option<Tree>>,
    value: String
}

impl Tree {
    pub fn new(value: String) -> Tree {
        return Tree {
            // The root node does not have a parent
            // Therefore this becomes None.
            parent: Box::new(None),
            value: value,
        }
    }
}

impl Tree {
    /// When we perform the decryptions, we will get a vector of Some<String>
    /// We need to loop through these and determine:
    /// 1. Did we reach our exit condition?
    /// 2. If not, create new nodes out of them and add them to the queue.
    /// We can return an Option? An Enum? And then match on that
    /// So if we return CrackSuccess we return
    /// Else if we return an array, we add it to the children and go again.
    pub fn bfs(&self) -> Option<String>{
        let mut q = VecDeque::new();
        // TODO I am not sure where self is coming from ?
        q.push_back(self);
        
        while let Some(t) = q.pop_front() {
            // TODO why do we have Some(t) here? This should skip the empty nodes, hopefully?

            // This returns a CrackObject
            let value = self.perform_decoding(t.unencrypted_text); // This gets us all the values from the decoders :pray:


            for decrypted_object in value {
                if decrypted_object.is_some() {
                    if decrypted_object.success {
                        // if we were successful, return idk what yet just a child fuck it
                        return decrypted_object;
                    }
                    // else add to the queue our new child node and continue
                    q.push_back(&Tree {parent: Some(t), value: decrypted_object.unencrypted_text});
                }
            }

        }
    }
    // Performs the decodings by getting all of the decoders
    // and calling `.run` which in turn loops through them and calls
    // `.crack()`.
    fn perform_decoding(&self, text: &str) -> Vec<Option<String>>{
        let decoders = filter_and_get_decoders();
        decoders.run(text)
    }
}