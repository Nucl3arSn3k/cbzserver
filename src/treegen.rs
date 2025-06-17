use crate::cbztools::dbHold;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{Graph, NodeIndex};
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashMap;
#[derive(Debug)]
pub struct TreeNode {
    contents: Vec<dbHold>, //each node holds the respective files/folders at its level. We can cram the dbHold objects in there
    nodelevel: i32,
}

pub struct Holder {
    //Holds both Hash and graph. May be unnecessary
    pub map: HashMap<String, NodeIndex>,
    pub tree: Graph<TreeNode, String>,
}

#[derive(Debug, Serialize)]
pub struct FrontendNode {
    pub id: String,            // Using filepath as unique ID
    pub contents: Vec<dbHold>, // Files/folders in this node
    pub level: i32,            // Depth in filesystem
    pub children: Vec<String>, // Filepaths of child nodes
}

impl FrontendNode {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            contents: Vec::new(),
            level: 0,
            children: Vec::new(),
        }
    }

    pub fn from_graph(holder: &Holder, node_path: &str) -> Option<Self> {
        let node_index = holder.map.get(node_path)?;
        let node = holder.tree.node_weight(*node_index)?;

        // Get paths of children
        let children = holder
            .tree
            .neighbors(*node_index)
            .filter_map(|child_idx| {
                holder
                    .map
                    .iter()
                    .find(|(_, &idx)| idx == child_idx)
                    .map(|(path, _)| path.clone())
            })
            .collect();

        Some(Self {
            //Return node instance to be serialized
            id: node_path.to_string(),
            contents: node.contents.clone(),
            level: node.nodelevel,
            children,
        })
    }
}

pub fn dump_graph(graph: Graph<TreeNode, String>) {
    //adding graphviz support for debug,take this out in final build
    let dot_string = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    std::fs::write("graph.dot", dot_string).expect("Error writing file");

    println!("Graph dumped");
}

/*Generate a graph. Each node is a folder in the tree. Store files in the internal vec and connect all the nodes via edges. Depth in filesystem is tracked with stack */

pub fn create_graph(con: Connection) -> Holder {
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
                        let nu_path = format!("{}\\{}\\{}", new_path[0], new_path[1],new_path[2]); //Always goes to I\\Comics. Should reconfig (temporarily for test path)
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
                println!("objects length is{:?}", objects.len());
                for x in objects.into_iter() {
                    //Set params for graphgen here,was only going 20 deep and thought I had broken graphgen,but forgot I set this flag. This comment is to remind myself
                    //Now for a real man's node generation

                    // Then in your processing loop:
                    if x.dirornot == 1 {
                        // Is dir

                        // Calculate the current item's level
                        let loclevel = &x.filepath;
                        let mut gval: Vec<&str> = loclevel.split('\\').collect();
                        gval.pop();
                        let current_level = gval.len() - 1;
                        let recombined = gval.join("\\");
                        println!("Actual level is : {:?}", loclevel);
                        println!("Parent level is :{:?}", recombined); //parent node gen
                        if graphtrack.contains_key(&recombined) {
                            //look up parent nodes
                            println!("recombined key found");
                            let parent_index = graphtrack.get(&recombined).unwrap();
                            let mut new_contents = Vec::new();
                            new_contents.push(x.clone());
                            let new_node = TreeNode {
                                contents: new_contents,
                                nodelevel: current_level as i32,
                            };
                            let new_index = graph.add_node(new_node);
                            graph.add_edge(*parent_index, new_index, "child".to_string());
                            graphtrack.insert(x.filepath, new_index);
                        }

                        // You can also use absolute_level if needed
                    } else {
                        // This is a file, add it to the current directory node
                        /*
                        if let Some(current_dir) = pathstack.last() {

                        }*/
                        //pop final element off,should show parent folder
                        let loclevel = &x.filepath;
                        let mut gval: Vec<&str> = loclevel.split('\\').collect();
                        gval.pop();
                        let current_level = gval.len() - 1;
                        let recombined = gval.join("\\");

                        println!("Actual filepath is {:?}", x.filepath);
                        println!("Parent filepath is {:?}", recombined);

                        if graphtrack.contains_key(&recombined) {
                            let dir_index = graphtrack.get(&recombined).unwrap();

                            // Get a mutable reference to the node and update its contents
                            if let Some(node_weight) = graph.node_weight_mut(*dir_index) {
                                node_weight.contents.push(x);
                            }
                        }
                        continue;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
        }
    }

    let hold_val = Holder {
        map: graphtrack,
        tree: graph,
    };

    hold_val
}
