pub struct Solution;

impl Solution {
    /// 1333. 餐厅过滤器
    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let mut restaurants: Vec<&Vec<i32>> = restaurants
            .iter()
            .filter(|x| x[2] >= vegan_friendly && x[3] <= max_price && x[4] <= max_distance)
            .collect();
        restaurants.sort_by(|&a, &b| {
            if a[1].eq(&b[1]) {
                b[0].cmp(&a[0])
            } else {
                b[1].cmp(&a[1])
            }
        });
        restaurants.iter().map(|x| x[0]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1333. 餐厅过滤器
    #[test]
    fn filter_restaurants() {
        let restaurants = vec![
            vec![1, 4, 1, 40, 10],
            vec![2, 8, 0, 50, 5],
            vec![3, 8, 1, 30, 4],
            vec![4, 10, 0, 10, 3],
            vec![5, 1, 1, 15, 1],
        ];
        let vegan_friendly = 1;
        let max_price = 50;
        let max_distance = 10;
        let res =
            Solution::filter_restaurants(restaurants, vegan_friendly, max_price, max_distance);
        assert_eq!(res, vec![3, 1, 5]);
    }
}
