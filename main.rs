use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;

type List = Vec<(usize,usize)>;
type Pair = (usize, usize);
type Edges = Vec<Vec<usize>>;
type Float = Vec<(usize, f64)>;


fn file_to_vec(filename:&str) -> List {
    let mut file = File::open(filename).expect("Can't open the file");
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read the file");
    
    let mut data:List = Vec::new();
    let lines:Vec<&str> = contents.lines().collect();
    
     for i in 4..lines.len() {
        let nums: Vec<&str> = lines[i].split_whitespace().collect();
            
        let pair:Pair = (nums[0].parse().unwrap(), nums[1].parse().unwrap());
                
        data.push(pair); 
    };

    return data;
}


fn outgoing_list(vec:List) -> Edges {
    let n = vec[vec.len() - 1].0 + 1;
    let mut outgoing_edges:Edges = vec![vec![];n];
    for (v,w) in vec.iter() {
        outgoing_edges[*v].push(*w);
    };
    return outgoing_edges;    
}    
    
fn incoming_list(vec:List) -> Edges {
    let n = vec[vec.len() - 1].0 + 1;
    let mut incoming_edges:Edges = vec![vec![];n];
    for (v,w) in vec.iter() {
        incoming_edges[*w].push(*v);
    };
    return incoming_edges;
} 

fn node_degree(vec:Edges) -> Float {
    let mut nodedegrees:Vec<(usize,f64)> = Vec::new();
    for i in 0..vec.len() {
        nodedegrees.push((i, vec[i].len() as f64));
    };

    return nodedegrees;
}

fn pagerank(edges:Edges, nodedegrees:Float, iter:usize) -> Float {
    let mut pageranks:Float = Vec::new();
    let total = edges.len() as f64;
    let initial_val = 1.0/total;

    for i in 0..edges.len() {
        let mut pr_val = 0.0;
        if edges[i].len() == 0 {
            pageranks.push((i,0.0));
        } else {
            for j in 0..edges[i].len() {
                let neighbor = edges[i][j];
                if nodedegrees[neighbor].1 != 0.0 {
                    pr_val += initial_val/nodedegrees[neighbor].1 as f64;
                };
            };
            pageranks.push((i,pr_val));
        };
    };

    for _n in 0..iter {
        for i in 0..edges.len() {
            if edges[i].len() != 0{
                let mut update = 0.0;
                for j in 0..edges[i].len() {
                    let neighbor = edges[i][j];
                    if nodedegrees[neighbor].1 != 0.0 {
                        update += pageranks[neighbor].1/nodedegrees[neighbor].1 as f64;
                    };
                };
                pageranks[i].1 = update;
            };
        };
    };

    pageranks.sort_by(|a, b| {
        if a.1 < b.1 {
            Ordering::Greater
        } else if a.1 == b.1 {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    });

    return pageranks;
}



fn main() {
    println!("PageRank for Outgoing Edges");
    for n in 0..3{
        print!("iteration {}: ", n+1);
        println!("{:?}",pagerank(outgoing_list(file_to_vec("amazon0302.txt"))
        , node_degree(outgoing_list(file_to_vec("amazon0302.txt"))), n)[0]);
    };

    println!();
    println!("PageRank for Incoming Edges");

    for n in 0..3{
        print!("iteration {}: ", n+1);
        println!("{:?}",pagerank(incoming_list(file_to_vec("amazon0302.txt"))
        , node_degree(incoming_list(file_to_vec("amazon0302.txt"))), n)[0]);
    };
}