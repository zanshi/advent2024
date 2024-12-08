use std::collections::{HashMap, HashSet};

use glam::IVec2;

pub fn part_one() -> i64 {
    let input = include_str!("../input.txt");
    let width = input.find('\n').unwrap();
    let input = input.split('\n').collect::<String>();
    let input = input.as_bytes();

    let index_to_coord = |index: usize| {
        let x = index % width;
        let y = index / width;

        IVec2::new(x as i32, y as i32)
    };

    let coord_to_index = |p: IVec2| (p.y * width as i32 + p.x) as usize;

    let mut antenna_map = HashMap::new();
    for (index, c) in input.iter().enumerate().filter(|(_, c)| **c != b'.') {
        antenna_map
            .entry(*c)
            .and_modify(|e: &mut Vec<usize>| {
                e.push(index);
            })
            .or_insert(vec![index]);
    }

    let mut unique_anti_nodes: HashSet<usize> = HashSet::new();

    for antenna_indices in antenna_map.values() {
        for i in 0..antenna_indices.len() {
            for j in (i + 1)..antenna_indices.len() {
                let index_a = antenna_indices[i];
                let index_b = antenna_indices[j];

                let point_a = index_to_coord(index_a);
                let point_b = index_to_coord(index_b);

                let v = point_b - point_a;

                let anti_node_a = point_a - v;
                let anti_node_b = point_b + v;

                if anti_node_a.x >= 0
                    && anti_node_a.x < width as i32
                    && anti_node_a.y >= 0
                    && anti_node_a.y < width as i32
                {
                    unique_anti_nodes.insert(coord_to_index(anti_node_a));
                }

                if anti_node_b.x >= 0
                    && anti_node_b.x < width as i32
                    && anti_node_b.y >= 0
                    && anti_node_b.y < width as i32
                {
                    unique_anti_nodes.insert(coord_to_index(anti_node_b));
                }
            }
        }
    }

    unique_anti_nodes.len() as i64
}

pub fn part_two() -> i64 {
    let input = include_str!("../input.txt");
    let width = input.find('\n').unwrap();
    let input = input.split('\n').collect::<String>();
    let input = input.as_bytes();

    let index_to_coord = |index: usize| {
        let x = index % width;
        let y = index / width;

        IVec2::new(x as i32, y as i32)
    };

    let coord_to_index = |p: IVec2| (p.y * width as i32 + p.x) as usize;

    let mut antenna_map = HashMap::new();
    for (index, c) in input.iter().enumerate().filter(|(_, c)| **c != b'.') {
        antenna_map
            .entry(*c)
            .and_modify(|e: &mut Vec<usize>| {
                e.push(index);
            })
            .or_insert(vec![index]);
    }

    let mut unique_anti_nodes: HashSet<usize> = HashSet::new();

    for antenna_indices in antenna_map.values() {
        for i in 0..antenna_indices.len() {
            unique_anti_nodes.insert(antenna_indices[i]);

            for j in (i + 1)..antenna_indices.len() {
                let index_a = antenna_indices[i];
                let index_b = antenna_indices[j];

                let point_a = index_to_coord(index_a);
                let point_b = index_to_coord(index_b);

                let v = point_b - point_a;

                let mut anti_node_a = point_a - v;
                let mut anti_node_b = point_b + v;

                while anti_node_a.x >= 0
                    && anti_node_a.x < width as i32
                    && anti_node_a.y >= 0
                    && anti_node_a.y < width as i32
                {
                    unique_anti_nodes.insert(coord_to_index(anti_node_a));

                    anti_node_a -= v;
                }

                while anti_node_b.x >= 0
                    && anti_node_b.x < width as i32
                    && anti_node_b.y >= 0
                    && anti_node_b.y < width as i32
                {
                    unique_anti_nodes.insert(coord_to_index(anti_node_b));

                    anti_node_b += v;
                }
            }
        }
    }

    unique_anti_nodes.len() as i64
}
