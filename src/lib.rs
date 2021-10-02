pub mod nodes;
pub mod errors;

#[cfg(test)]
mod tests {

    use crate::nodes::node_constructor;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn basicnodetest() {
        let node = node_constructor(&[3.0, 5.0, 7.0], 4.0);
        assert_eq!(node.get_result(&[1.0, 1.0, 1.0]).expect("if you can see this my day has been ruined."), 19.0)
    }
}

