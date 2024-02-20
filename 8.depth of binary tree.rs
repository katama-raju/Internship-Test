 pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
     let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode::new(4))),
            right: Some(Box::new(TreeNode::new(5))),
        })),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode::new(6))),
            right: None,
        })),
    }));

    let depth = max_depth(root);
    println!("Maximum Depth of the Binary Tree: {}", depth);
}
