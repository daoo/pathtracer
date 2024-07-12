use crate::{KdNode, KdTree};
use geometry::{axis::Axis, geometry::Geometry};
use std::io::{self};

fn write_triangle_bracketed<W>(write: &mut W, geometries: &[Geometry]) -> Result<(), io::Error>
where
    W: io::Write,
{
    write!(
        write,
        "{:?}",
        geometries
            .iter()
            .map(|t| match t {
                Geometry::Triangle(t) => t.as_arrays(),
                Geometry::AxiallyAlignedTriangle(t) => t.as_arrays(),
            })
            .collect::<Vec<_>>()
    )
}

pub fn write_node_pretty<W>(write: &mut W, root: &KdNode) -> Result<(), io::Error>
where
    W: io::Write,
{
    let mut stack: Vec<(usize, &KdNode)> = Vec::new();
    stack.push((0, root));

    while let Some((depth, node)) = stack.pop() {
        let indent = "  ".repeat(depth);
        match node {
            KdNode::Leaf(indices) => writeln!(write, "{indent}Leaf {indices:?}")?,
            KdNode::Node { plane, left, right } => {
                stack.push((depth + 1, left));
                stack.push((depth + 1, right));
                writeln!(
                    write,
                    "{}Split {:?} {}",
                    "  ".repeat(depth),
                    plane.axis,
                    plane.distance
                )?;
            }
        }
    }
    Ok(())
}

pub fn write_tree_pretty<W>(write: &mut W, tree: &KdTree) -> Result<(), io::Error>
where
    W: io::Write,
{
    write_node_pretty(write, &tree.root)
}

pub fn write_node_rust<W>(write: &mut W, node: &KdNode) -> Result<(), io::Error>
where
    W: io::Write,
{
    match node {
        KdNode::Leaf(indices) if indices.is_empty() => write!(write, "KdNode::empty()")?,
        KdNode::Leaf(indices) => write!(write, "KdNode::new_leaf(vec!{indices:?})")?,
        KdNode::Node { plane, left, right } => {
            let aap_new = match plane.axis {
                Axis::X => "Aap::new_x",
                Axis::Y => "Aap::new_y",
                Axis::Z => "Aap::new_z",
            };

            write!(
                write,
                "KdNode::new_node({}({:?}), ",
                aap_new, plane.distance
            )?;
            write_node_rust(write, left)?;
            write!(write, ", ")?;
            write_node_rust(write, right)?;
            write!(write, ")")?;
        }
    }
    Ok(())
}

pub fn write_tree_rust<W>(write: &mut W, tree: &KdTree) -> Result<(), io::Error>
where
    W: io::Write,
{
    write!(write, "let geometries = ")?;
    write_triangle_bracketed(write, &tree.geometries)?;
    writeln!(write, ";")?;
    write!(write, "let root = ")?;
    write_node_rust(write, &tree.root)?;
    writeln!(write, ";")?;
    writeln!(write, "let tree = KdTree {{ root, geometries }};")?;
    Ok(())
}

pub fn write_node_json<W>(write: &mut W, node: &KdNode) -> Result<(), io::Error>
where
    W: io::Write,
{
    match node {
        KdNode::Leaf(indices) => write!(write, "{indices:?}")?,
        KdNode::Node { plane, left, right } => {
            write!(
                write,
                "{{\"axis\": \"{:?}\", \"distance\": {}, \"left\": ",
                plane.axis, plane.distance
            )?;
            write_node_json(write, left)?;
            write!(write, ", \"right\": ")?;
            write_node_json(write, right)?;
            write!(write, "}}")?;
        }
    };

    Ok(())
}

pub fn write_tree_json<W>(write: &mut W, tree: &KdTree) -> Result<(), io::Error>
where
    W: io::Write,
{
    write!(write, "{{\"triangles\": ")?;
    write_triangle_bracketed(write, &tree.geometries)?;
    write!(write, ", \"root\": ")?;
    write_node_json(write, &tree.root)?;
    writeln!(write, "}}")?;
    Ok(())
}

pub fn write_node_dot<W>(write: &mut W, path: String, node: &KdNode) -> Result<(), io::Error>
where
    W: io::Write,
{
    match node {
        KdNode::Leaf(indices) => {
            let formatted = format!("{:?}", indices);
            let wrapped = textwrap::fill(formatted.as_str(), 60);
            writeln!(write, "  {} [label={:?}];", path, wrapped)?;
        }
        KdNode::Node { plane, left, right } => {
            writeln!(
                write,
                "  {} [label=\"{:?} {}\"];",
                path, plane.axis, plane.distance
            )?;
            let left_path = path.clone() + "l";
            let right_path = path.clone() + "r";
            writeln!(write, "  {} -> {};", &path, left_path)?;
            writeln!(write, "  {} -> {};", &path, right_path)?;
            write_node_dot(write, left_path, left)?;
            write_node_dot(write, right_path, right)?;
        }
    }
    Ok(())
}

pub fn write_tree_dot<W>(write: &mut W, tree: &KdTree) -> Result<(), io::Error>
where
    W: io::Write,
{
    writeln!(write, "digraph {{")?;
    writeln!(write, "  rankdir=\"LR\";")?;
    writeln!(write, "  node [shape=\"box\"];")?;
    write_node_dot(write, "t".to_string(), &tree.root)?;
    writeln!(write, "}}")?;
    Ok(())
}
