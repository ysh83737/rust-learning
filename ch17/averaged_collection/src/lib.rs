pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0 as f64,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut collection = AveragedCollection::new();

        collection.add(1);
        assert_eq!(collection.list.len(), 1);
        assert_eq!(collection.list.get(0), Some(1 as i32).as_ref());

        collection.add(2);
        assert_eq!(collection.list.len(), 2);
        assert_eq!(collection.list.get(1), Some(2 as i32).as_ref());
    }

    #[test]
    fn test_average() {
        let mut collection = AveragedCollection::new();

        collection.add(1);

        assert_eq!(collection.average, 1 as f64);

        collection.add(2);
        assert_eq!(collection.average, 1.5);
    }

    #[test]
    fn test_remove() {
        let mut collection = AveragedCollection::new();

        collection.add(1);
        assert_eq!(collection.list.len(), 1);

        let result = collection.remove();
        assert_eq!(collection.list.len(), 0);
        assert_eq!(result, Some(1));

        let result = collection.remove();
        assert_eq!(collection.list.len(), 0);
        assert_eq!(result, None);
    }
}
