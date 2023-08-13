use crate::ray::{hit_scan, shadow_hit_scan, HitRecord, Hittable, HittableList, Ray};
use super::{AABB, Plane, Dimension};
use super::candidate::{Candidates, Candidate, Side};

// Values taken from the paper "On building fast kd-Trees for Ray
// Tracing, and on doing that in O(N log N)"
static K_T: f32 = 15.; // Cost of tree traversal
static K_I: f32 = 20.; // Cost of intersection

// 1. -> Cutting an empty space is always better than cutting full one
// 0. -> Cutting an empty space is never better than cutting full one
static EMPTY_CUT_BONUS: f32 = 0.2;

pub struct KDTree {
    hittables: HittableList,
    tree: Vec<KDTreeNode>,
    depth: usize,
}

pub enum KDTreeNode {
    Leaf {
        shapes: Vec<usize>,
    },
    Node {
        l_child: usize,
        l_space: AABB,
        r_child: usize,
        r_space: AABB,
    },
}

impl KDTree {
    pub fn build(shapes: HittableList) -> Self {
        // Can only build tree if we have hittables
        assert!(!shapes.objects.is_empty());
        let nb_shapes = shapes.objects.len();

        let mut space: AABB = Default::default();
        let mut candidates = Candidates::with_capacity(nb_shapes * 6);

        for (index, shape) in shapes.objects.iter().enumerate() {
            let bb = shape.bound();
            candidates.extend(Candidate::gen_candidates(index, &bb));
            space.merge(&bb);
        }

        // Sorting, as the order is important for the algorithm
        candidates.sort();

        let mut sides = vec![Side::Both; nb_shapes];
        let mut tree = vec![];
        let depth = build_tree(
            &space, candidates, nb_shapes, &mut sides, &mut tree,
        );

        Self {
            hittables: shapes,
            tree,
            depth,
        }
    }
}

impl Hittable for KDTree {
    fn hit(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord> {
        let mut result: Vec<usize> = vec![];
        let mut stack = vec![0];

        stack.reserve_exact(self.depth);
        while !stack.is_empty() {
            let node = &self.tree[stack.pop().unwrap()];
            match node {
                KDTreeNode::Leaf {
                    shapes,
                } => result.extend(shapes),
                KDTreeNode::Node {
                    l_child,
                    l_space,
                    r_child,
                    r_space,
                } => {
                    if r.intersect_aabb(r_space) {
                        stack.push(*r_child);
                    }
                    if r.intersect_aabb(l_space) {
                        stack.push(*l_child);
                    }
                }
            }
        }

        result.sort();
        result.dedup();

        let objects = result
            .into_iter()
            .map(|index| self.hittables.objects[index].as_ref());
        hit_scan(objects, r, t_min, t_max)
    }

    fn shadow_hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut result: Vec<usize> = vec![];
        let mut stack = vec![0];

        stack.reserve_exact(self.depth);
        while !stack.is_empty() {
            let node = &self.tree[stack.pop().unwrap()];
            match node {
                KDTreeNode::Leaf {
                    shapes,
                } => result.extend(shapes),
                KDTreeNode::Node {
                    l_child,
                    l_space,
                    r_child,
                    r_space,
                } => {
                    if r.intersect_aabb(r_space) {
                        stack.push(*r_child);
                    }
                    if r.intersect_aabb(l_space) {
                        stack.push(*l_child);
                    }
                }
            }
        }

        result.sort();
        result.dedup();

        let objects = result
            .into_iter()
            .map(|index| self.hittables.objects[index].as_ref());
        shadow_hit_scan(objects, r, t_min, t_max)
    }

    /// Dummy impl, shouldn't be used
    fn bound(&self) -> AABB {
        Default::default()
    }
}

pub fn build_tree(
    space: &AABB,
    candidates: Candidates,
    nb_shapes: usize,
    sides: &mut [Side],
    tree: &mut Vec<KDTreeNode>,
) -> usize {
    let (cost, best_index, n_l, n_r) =
        partition(nb_shapes, space, &candidates);

    if cost > K_I * nb_shapes as f32 {
        let shapes = candidates
            .iter()
            .filter(|e| e.is_left() && e.dimension() == Dimension::X)
            .map(|e| e.shape)
            .collect();
        tree.push(KDTreeNode::Leaf {
            shapes,
        });
        return 1;
    }

    let (left_space, right_space) =
        split_space(space, &candidates[best_index].plane);
    let (left_candidates, right_candidates) =
        classify(candidates, best_index, sides);

    let node_index = tree.len();
    tree.push(KDTreeNode::Node {
        l_child: node_index + 1,
        l_space: left_space.clone(),
        r_child: 0,
        r_space: right_space.clone(),
    });

    let depth_left =
        build_tree(&left_space, left_candidates, n_l, sides, tree);

    let r_child_index = tree.len();
    if let KDTreeNode::Node {
        ref mut r_child,
        ..
    } = tree[node_index]
    {
        *r_child = r_child_index;
    }

    let depth_right =
        build_tree(&right_space, right_candidates, n_r, sides, tree);

    1 + depth_left.max(depth_right)
}

fn split_space(space: &AABB, plane: &Plane) -> (AABB, AABB) {
    let mut left = space.clone();
    let mut right = space.clone();
    let pos = plane.pos;

    match plane.dimension {
        Dimension::X => {
            right.min.x = pos.clamp(space.min.x, space.max.x);
            left.max.x = pos.clamp(space.min.x, space.max.x);
        }
        Dimension::Y => {
            right.min.y = pos.clamp(space.min.y, space.max.y);
            left.max.y = pos.clamp(space.min.y, space.max.y);
        }
        Dimension::Z => {
            right.min.z = pos.clamp(space.min.z, space.max.z);
            left.max.z = pos.clamp(space.min.z, space.max.z);
        }
    };

    (left, right)
}

fn partition(
    n: usize,
    space: &AABB,
    candidates: &Candidates,
) -> (f32, usize, usize, usize) {
    let mut best_cost = f32::INFINITY;
    let mut best_candidate_index = 0;

    // Number of items in both subspace for each dimension
    let mut n_l = [0usize; 3];
    let mut n_r = [n; 3];

    let mut best_n_l = 0;
    let mut best_n_r = n;

    for (i, candidate) in candidates.iter().enumerate() {
        let dim = match candidate.dimension() {
            Dimension::X => 0usize,
            Dimension::Y => 1usize,
            Dimension::Z => 2usize,
        };

        if candidate.is_right() {
            n_r[dim] -= 1;
        }

        let cost = cost(&candidate.plane, space, n_l[dim], n_r[dim]);
        if cost < best_cost {
            best_cost = cost;
            best_candidate_index = i;
            best_n_l = n_l[dim];
            best_n_r = n_r[dim];
        }

        if candidate.is_left() {
            n_l[dim] += 1;
        }
    }

    (best_cost, best_candidate_index, best_n_l, best_n_r)
}

fn classify(
    candidates: Candidates,
    best_index: usize,
    sides: &mut [Side],
) -> (Candidates, Candidates) {
    classify_items(&candidates, best_index, sides);
    splicing_candidates(candidates, sides)
}

fn classify_items(
    candidates: &Candidates,
    best_index: usize,
    sides: &mut [Side],
) {
    let best_dimension = candidates[best_index].dimension();
    for i in 0..(best_index + 1) {
        if candidates[i].dimension() != best_dimension {
            continue;
        }

        if candidates[i].is_right() {
            sides[candidates[i].shape] = Side::Left;
        } else {
            sides[candidates[i].shape] = Side::Both;
        }
    }

    for i in best_index..candidates.len() {
        if candidates[i].dimension() != best_dimension {
            continue;
        }

        if candidates[i].is_left() {
            sides[candidates[i].shape] = Side::Right;
        }
    }
}

fn splicing_candidates(
    candidates: Candidates,
    sides: &[Side],
) -> (Candidates, Candidates) {
    let mut left_candidates =
        Candidates::with_capacity(candidates.len() / 2);
    let mut right_candidates =
        Candidates::with_capacity(candidates.len() / 2);

    for e in candidates {
        match sides[e.shape] {
            Side::Left => left_candidates.push(e),
            Side::Right => right_candidates.push(e),
            Side::Both => {
                right_candidates.push(e.clone());
                left_candidates.push(e);
            }
        }
    }

    (left_candidates, right_candidates)
}

// Surface Area Heuristic (SAH)
fn cost(
    plane: &Plane,
    space: &AABB,
    n_left: usize,
    n_right: usize,
) -> f32 {
    // If we're not cutting the space, we want to make the costs so
    // high that it will never be chosen
    if !plane.is_cutting(space) {
        return f32::INFINITY;
    }

    let space_surface = space.surface();
    let (space_left, space_right) = split_space(space, plane);
    let left_surface = space_left.surface();
    let right_surface = space_right.surface();
    let cost = K_T
        + K_I
            * (n_left as f32 * left_surface / space_surface
                + n_right as f32 * right_surface / space_surface);

    if n_left == 0 || n_right == 0 {
        cost * (1. - EMPTY_CUT_BONUS)
    } else {
        cost
    }
}
