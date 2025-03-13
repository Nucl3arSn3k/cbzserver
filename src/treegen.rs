use crate::cbztools::dbHold;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};
use rusqlite::Connection;
use std::collections::HashMap;
#[derive(Debug)]
pub struct TreeNode {
    contents: Vec<dbHold>, //each node holds the respective files/folders at its level. We can cram the dbHold objects in there
    nodelevel: i32,
}




pub fn dump_graph(graph: Graph<TreeNode, String>) { //adding graphviz support for debug,take this out in final build
    let dot_string = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    std::fs::write("graph.dot", dot_string).expect("Error writing file");

    println!("Graph dumped");

}



/*Generate a graph. Each node is a folder in the tree. Store files in the internal vec and connect all the nodes via edges. Depth in filesystem is tracked with stack */



pub fn create_graph(con: Connection) -> Graph<TreeNode, String> {
    let mut graph = Graph::<TreeNode, String>::new();
    let mut graphtrack: HashMap<String, NodeIndex> = HashMap::new();

    match con.prepare("SELECT name, filepath, coverpath, dirornot FROM files") {
        Ok(mut indiv) => {
            if let Ok(entry_iter) = indiv.query_map([], |row| {
                Ok(dbHold {
                    name: row.get(0).unwrap_or_default(),
                    filepath: row.get(1).unwrap_or_default(),
                    cover_path: row.get(2).unwrap_or_default(),
                    dirornot: row.get(3).unwrap_or_default(),
                })
            }) {
                let mut objects = Vec::new();
                for entry in entry_iter {
                    if let Ok(item) = entry {
                        objects.push(item);
                    }
                }
                let baseline: dbHold;
                match objects.get(0) {
                    //grab first item,clone instead of destroy,because need it for later nodes. Later nodes just destroy,because unneeded.
                    Some(x) => {
                        let mut owned_x = x.clone();
                        let new_path: Vec<&str> = owned_x.filepath.split('\\').collect();
                        let nu_path = format!("{}\\{}", new_path[0], new_path[1]);
                        println!("New path: {}", nu_path);
                        let root_name = new_path[1].to_string();
                        let passed_name = root_name.clone();
                        let passed_path = nu_path.clone();
                        owned_x.filepath = nu_path;
                        owned_x.name = root_name;
                        let mut vals: Vec<dbHold> = Vec::new();
                        let path = owned_x.filepath.clone(); //ugh,cloning again
                        vals.push(owned_x); //here's the move

                        let root_node = TreeNode {
                            contents: vals,
                            nodelevel: 2,
                        };
                        let node_index = graph.add_node(root_node);
                        graphtrack.insert(path, node_index);

                        baseline = dbHold {
                            name: passed_name,
                            filepath: passed_path,
                            cover_path: None,
                            dirornot: 1,
                        }
                    }
                    None => {
                        baseline = dbHold {
                            name: todo!(),
                            filepath: todo!(),
                            cover_path: todo!(),
                            dirornot: todo!(),
                        };
                        println!("Error")
                    }
                };
                let mut pathstack: Vec<dbHold> = Vec::new();

                pathstack.push(baseline); //Shove the baseline object to the statestack

                for x in objects {
                    //Now for a real man's node generation
                    //let pathcheck = x.filepath;

                    //level decided by subing 1

                    // Store the root level when you initialize your stack
                    let root_level = 1; // Or whatever the initial level is

                    // Then in your processing loop:
                    if x.dirornot == 1 {
                        // Is dir
                        let val = pathstack.last().unwrap(); // peek the stack
                        let val_path = &val.filepath;
                        let sval: Vec<&str> = val_path.split('\\').collect();
                        let stack_top_level = sval.len() - 1; // level of the last element

                        // Calculate the current item's level
                        let loclevel = &x.filepath;
                        let gval: Vec<&str> = loclevel.split('\\').collect();
                        let current_level = gval.len() - 1;

                        // Get both differences
                        let diff_from_stack_top: isize =
                            current_level as isize - stack_top_level as isize;
                        let absolute_level: isize = current_level as isize - root_level as isize;

                        println!("Level diff from stack top: {:?}", diff_from_stack_top);
                        println!("Absolute level from root: {:?}", absolute_level);

                        // Now you can use both pieces of information
                        if diff_from_stack_top == 1 { // Just add as child to current, and push onto stack
                            if graphtrack.contains_key(&val.filepath) {
                                let parent_index = graphtrack.get(&val.filepath).unwrap();
                                let mut new_contents = Vec::new();
                                new_contents.push(x.clone());
                                
                                let new_node = TreeNode {
                                    contents: new_contents,
                                    nodelevel: current_level as i32,
                                };
                                
                                let new_index = graph.add_node(new_node);
                                graph.add_edge(*parent_index, new_index, "child".to_string());
                                graphtrack.insert(x.filepath.clone(), new_index);
                                pathstack.push(x); // Add to stack
                            }
                        } else if diff_from_stack_top == 0 { // sibling, pop once, add as child to parent, push to stack
                            pathstack.pop(); // Remove current
                            if let Some(parent) = pathstack.last() {
                                if graphtrack.contains_key(&parent.filepath) {
                                    let parent_index = graphtrack.get(&parent.filepath).unwrap();
                                    let mut new_contents = Vec::new();
                                    new_contents.push(x.clone());
                                    
                                    let new_node = TreeNode {
                                        contents: new_contents,
                                        nodelevel: current_level as i32,
                                    };
                                    
                                    let new_index = graph.add_node(new_node);
                                    graph.add_edge(*parent_index, new_index, "child".to_string());
                                    graphtrack.insert(x.filepath.clone(), new_index);
                                    pathstack.push(x); // Add to stack
                                }
                            }
                        }
                        else if diff_from_stack_top < 0 {
                            // Pop multiple times to get to the right level
                            let pops_needed = diff_from_stack_top.abs() as usize;
                            
                            // Pop the stack to get to the right level
                            for _ in 0..pops_needed {
                                pathstack.pop();
                            }
                            
                            // Now add as a child of the current top of stack
                            if let Some(ancestor) = pathstack.last() {
                                if graphtrack.contains_key(&ancestor.filepath) {
                                    let ancestor_index = graphtrack.get(&ancestor.filepath).unwrap();
                                    let mut new_contents = Vec::new();
                                    new_contents.push(x.clone());
                                    
                                    let new_node = TreeNode {
                                        contents: new_contents,
                                        nodelevel: current_level as i32,
                                    };
                                    
                                    let new_index = graph.add_node(new_node);
                                    graph.add_edge(*ancestor_index, new_index, "child".to_string());
                                    graphtrack.insert(x.filepath.clone(), new_index);
                                    pathstack.push(x); // Add to stack
                                }
                            }
                        }

                        // You can also use absolute_level if needed
                    } else {
                        // This is a file, add it to the current directory node
                        if let Some(current_dir) = pathstack.last() {
                            if graphtrack.contains_key(&current_dir.filepath) {
                                let dir_index = graphtrack.get(&current_dir.filepath).unwrap();
                                
                                // Get a mutable reference to the node and update its contents
                                if let Some(node_weight) = graph.node_weight_mut(*dir_index) {
                                    node_weight.contents.push(x);
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
        }
    }

    graph
}
