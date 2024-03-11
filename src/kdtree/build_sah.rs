use nalgebra::vector;

use crate::geometry::{
    aabb::Aabb,
    aap::{Aap, Axis},
    algorithms::triangles_bounding_box,
    triangle::Triangle,
};

use super::{
    build::{KdBox, KdTreeInputs},
    KdNode, KdTree,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventKind {
    START,
    PLANAR,
    END,
}

#[derive(Debug)]
pub struct Event {
    pub kind: EventKind,
    pub distance: f32,
}

impl Event {
    fn total_cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (
            f32::total_cmp(&self.distance, &other.distance),
            self.kind.cmp(&other.kind),
        ) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => std::cmp::Ordering::Less,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => std::cmp::Ordering::Less,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => std::cmp::Ordering::Less,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => std::cmp::Ordering::Less,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => std::cmp::Ordering::Equal,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => std::cmp::Ordering::Greater,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => std::cmp::Ordering::Greater,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => std::cmp::Ordering::Greater,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
                std::cmp::Ordering::Greater
            }
        }
    }
}

fn split_events(input: &KdTreeInputs, triangle_index: u32, axis: Axis) -> Vec<Event> {
    let triangle = &input.triangles[triangle_index as usize];
    let a = triangle.min()[axis];
    let b = triangle.max()[axis];
    if a == b {
        vec![Event {
            kind: EventKind::PLANAR,
            distance: a,
        }]
    } else {
        vec![
            Event {
                kind: EventKind::START,
                distance: a,
            },
            Event {
                kind: EventKind::END,
                distance: b,
            },
        ]
    }
}

fn potential_split_events(input: &KdTreeInputs, parent: &KdBox, axis: Axis) -> Vec<Event> {
    let min = parent.boundary.min()[axis];
    let max = parent.boundary.max()[axis];
    let mut points = parent
        .triangle_indices
        .iter()
        .flat_map(|i| split_events(input, *i, axis))
        .filter(|p| p.distance >= min && p.distance <= max)
        .collect::<Vec<_>>();
    points.sort_by(Event::total_cmp);
    points
}

fn count_events(events: &[Event]) -> (usize, usize, usize) {
    let minus = events
        .iter()
        .take_while(|x| x.kind == EventKind::END && x.distance == events[0].distance)
        .count();
    let plane = events
        .iter()
        .skip(minus)
        .take_while(|x| x.kind == EventKind::PLANAR && x.distance == events[0].distance)
        .count();
    let plus = events
        .iter()
        .skip(minus + plane)
        .take_while(|x| x.kind == EventKind::START && x.distance == events[0].distance)
        .count();
    return (minus, plane, plus);
}

const COST_EMPTY_FACTOR: f32 = 0.8;
const COST_TRAVERSE: f32 = 0.1;
const COST_INTERSECT: f32 = 1.0;

fn calculate_sah_cost_helper(probability: (f32, f32), counts: (usize, usize)) -> f32 {
    assert!(probability.0 >= 0.0 && probability.1 >= 0.0);
    assert!(probability.0 > 0.0 || probability.1 > 0.0);
    let empty_factor = if counts.0 == 0 || counts.1 == 0 {
        COST_EMPTY_FACTOR
    } else {
        1.0
    };
    let intersect_cost =
        COST_INTERSECT * (probability.0 * counts.0 as f32 + probability.1 * counts.1 as f32);
    empty_factor * (COST_TRAVERSE + intersect_cost)
}

enum Side {
    LEFT,
    RIGHT,
}

struct Cost {
    cost: f32,
    side: Side,
}

fn calculate_sah_cost(parent: &Aabb, plane: &Aap, counts: (usize, usize, usize)) -> Cost {
    assert!(parent.surface_area() > 0.0);
    let (left, right) = parent.split(plane);
    if left.volume() <= 0.0 {
        return Cost {
            cost: f32::MAX,
            side: Side::LEFT,
        };
    }
    if right.volume() <= 0.0 {
        return Cost {
            cost: f32::MAX,
            side: Side::RIGHT,
        };
    }

    let probability_left = left.surface_area() / parent.surface_area();
    let probability_right = right.surface_area() / parent.surface_area();
    let probability = (probability_left, probability_right);

    let cost_plane_left = calculate_sah_cost_helper(probability, (counts.0 + counts.1, counts.2));
    let cost_plane_right = calculate_sah_cost_helper(probability, (counts.0, counts.1 + counts.2));

    if cost_plane_left <= cost_plane_right {
        Cost {
            cost: cost_plane_left,
            side: Side::LEFT,
        }
    } else {
        Cost {
            cost: cost_plane_right,
            side: Side::RIGHT,
        }
    }
}

struct CostSplit {
    plane: Aap,
    cost: Cost,
}

impl CostSplit {
    fn cheapest(self, other: Self) -> Self {
        if self.cost.cost <= other.cost.cost {
            self
        } else {
            other
        }
    }
}

fn find_best_split(inputs: &KdTreeInputs, parent: &KdBox) -> CostSplit {
    assert!(parent.boundary.volume() > 0.0);
    assert!(!parent.triangle_indices.is_empty());

    let mut best = CostSplit {
        plane: Aap {
            axis: Axis::X,
            distance: 0.0,
        },
        cost: Cost {
            cost: f32::MAX,
            side: Side::LEFT,
        },
    };
    for axis in [Axis::X, Axis::Y, Axis::Z].iter() {
        let events = potential_split_events(inputs, parent, *axis);
        println!("{:?}", &events);
        let mut number_left = 0;
        let mut number_right = parent.triangle_indices.len();
        for (i, event) in events.iter().enumerate() {
            let (pminus, pplane, pplus) = count_events(&events[i..]);
            println!(
                "number_left={} number_right={} pminus={} pplane={} pplus={}",
                number_left, number_right, pminus, pplane, pplus
            );
            number_right = number_right - pminus - pplane;
            let plane = Aap {
                axis: *axis,
                distance: event.distance,
            };
            let cost = calculate_sah_cost(
                &parent.boundary,
                &plane,
                (number_left, pplane, number_right),
            );
            number_left = number_left + pplus + pplane;
            best = best.cheapest(CostSplit { plane, cost });
        }
    }

    todo!()
}

fn build(inputs: &KdTreeInputs, depth: u32, parent: KdBox) -> Box<KdNode> {
    if depth >= inputs.max_depth || parent.triangle_indices.is_empty() {
        return Box::new(KdNode::Leaf(parent.triangle_indices));
    }

    let best_split = find_best_split(inputs, &parent);
    if best_split.cost.cost >= f32::MAX {
        return Box::new(KdNode::Leaf(parent.triangle_indices));
    }
    let split = inputs.split_box(parent, &best_split.plane);
    let left = build(inputs, depth + 1, split.left);
    let right = build(inputs, depth + 1, split.right);
    Box::new(KdNode::Node {
        plane: best_split.plane,
        left,
        right,
    })
}

pub fn build_kdtree_sah(max_depth: u32, triangles: Vec<Triangle>) -> KdTree {
    let kdbox: KdBox = KdBox {
        boundary: triangles_bounding_box(&triangles).enlarge(&vector![0.1, 0.1, 0.1]),
        triangle_indices: (0u32..triangles.len() as u32).collect(),
    };
    let inputs = KdTreeInputs {
        max_depth,
        triangles,
    };
    KdTree {
        root: build(&inputs, 0, kdbox),
        triangles: inputs.triangles,
    }
}
