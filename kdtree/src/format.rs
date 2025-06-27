use crate::KdNode;
use geometry::{axis::Axis, shape::Shape};
use std::io::{self};

fn write_triangle_bracketed<W>(write: &mut W, geometries: &[Shape]) -> Result<(), io::Error>
where
    W: io::Write,
{
    write!(
        write,
        "{:?}",
        geometries
            .iter()
            .map(|t| match t {
                Shape::Triangle(t) => t.as_arrays(),
                Shape::AxiallyAlignedTriangle(t) => t.as_arrays(),
                Shape::Sphere(_) => todo!(),
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

pub fn write_tree_rust<W>(
    write: &mut W,
    geometries: &[Shape],
    tree: &KdNode,
) -> Result<(), io::Error>
where
    W: io::Write,
{
    write!(write, "let geometries = ")?;
    write_triangle_bracketed(write, geometries)?;
    writeln!(write, ".map(Triangle::from);")?;
    write!(write, "let node = ")?;
    write_node_rust(write, tree)?;
    writeln!(write, ";")?;
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
    }

    Ok(())
}

pub fn write_tree_json<W>(
    write: &mut W,
    geometries: &[Shape],
    tree: &KdNode,
) -> Result<(), io::Error>
where
    W: io::Write,
{
    write!(write, "{{\"triangles\": ")?;
    write_triangle_bracketed(write, geometries)?;
    write!(write, ", \"root\": ")?;
    write_node_json(write, tree)?;
    writeln!(write, "}}")?;
    Ok(())
}
