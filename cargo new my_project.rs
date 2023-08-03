cargo new my_project
struct FilterCondition<T> {
    desired_value: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        item == &self.desired_value
    }
}

fn custom_filter<T>(collection: &[T], condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq,
    T: Clone,
{
    collection
        .iter()
        .cloned()
        .filter(|item| condition.is_match(item))
        .collect()
}

fn main() {
    let original_collection = vec![1, 2, 3, 4, 5, 6];

    let condition = FilterCondition { desired_value: 3 };

    let filtered_collection = custom_filter(&original_collection, &condition);

    println!("Original Collection: {:?}", original_collection);
    println!("Filtered Collection: {:?}", filtered_collection);
}
