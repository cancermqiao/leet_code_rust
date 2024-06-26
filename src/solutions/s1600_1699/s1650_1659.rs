use std::cmp::Ordering;

struct JumpInfo {
    position: i32,
    direction: i32,
    step: i32,
}

pub struct Solution;

impl Solution {
    /// 1654. 到家的最小跳跃次数
    #[allow(dead_code)]
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let mut visited_position = std::collections::HashSet::new();
        visited_position.insert(0);
        let mut queue = vec![JumpInfo {
            position: 0,
            direction: 1,
            step: 0,
        }];
        let lower_bound = 0;
        let upper_bound = match a.cmp(&b) {
            Ordering::Equal => x,
            Ordering::Greater => x + b,
            Ordering::Less => x.max(forbidden.iter().max().unwrap() + a) + b,
        };

        while let Some(jump_info) = queue.pop() {
            if jump_info.position == x {
                return jump_info.step;
            }
            let next_position = jump_info.position + a;
            if next_position <= upper_bound
                && !visited_position.contains(&next_position)
                && !forbidden.contains(&next_position)
            {
                visited_position.insert(next_position);
                queue.insert(
                    0,
                    JumpInfo {
                        position: next_position,
                        direction: 1,
                        step: jump_info.step + 1,
                    },
                );
            }
            if jump_info.direction > 0 {
                let next_position = jump_info.position - b;
                if next_position > lower_bound
                    && !visited_position.contains(&(-next_position))
                    && !forbidden.contains(&next_position)
                {
                    visited_position.insert(-next_position);
                    queue.insert(
                        0,
                        JumpInfo {
                            position: next_position,
                            direction: -1,
                            step: jump_info.step + 1,
                        },
                    );
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1654. 到家的最小跳跃次数
    #[test]
    fn minimum_jumps() {
        let forbidden = vec![
            1906, 1988, 1693, 483, 900, 1173, 805, 1593, 1208, 1084, 300, 614, 1325, 783, 1104,
            1450, 311, 1506, 1388, 1567, 1497, 47, 102, 338, 1937, 888, 111, 195, 1041, 1570, 686,
            1707, 1521, 1566, 74, 1264, 667, 1486, 960, 389, 442, 329, 1577, 1557, 1494, 1382,
            1688, 779, 484, 410, 227, 1025, 1417, 1475, 1042, 1903, 1920, 1712, 870, 1813, 1137,
            1732, 18, 1065, 1653, 1289, 1636, 147, 1833, 1168, 1087, 1408, 881, 1129, 71, 924,
            1718, 1458, 371, 597, 1790, 889, 414, 784, 1883, 6, 1650, 1549, 552, 1233, 1467, 1514,
            1568, 211, 1301, 772, 377, 1751, 1699, 1701, 1214, 1874, 324, 1991, 1006, 1413, 41,
            289, 1274, 802, 1892, 1908, 1960, 1635, 69, 423, 1795, 96, 1024, 1596, 1044, 1513,
            1390, 711, 1806, 1298, 968, 1160, 1232, 1315, 1646, 1178, 169, 1295, 466, 44, 10, 1250,
            1283, 927, 49, 267, 1773, 342, 1828, 1949, 1291, 244, 707, 408, 798, 938, 1542, 690,
            639, 1148, 1081, 431, 752, 120, 1125, 339, 480, 247, 733, 266, 596, 987, 777, 214,
            1005, 1687, 160, 785, 1010, 1282, 1135, 922, 671, 1221, 250, 1982, 398, 1959, 179, 325,
            1313, 577, 1053, 1436, 185, 1014, 1851, 1685, 1143, 1510, 1972, 830, 681, 390, 972,
            1003, 844, 229, 1246, 1257, 668, 1765, 619, 276, 1355, 1544, 1842, 1340, 1375, 1944,
            790, 606, 345, 1487, 796, 1985, 1673, 1503, 180, 1642, 498, 1805, 201, 104, 1658, 1633,
            1507, 1142, 541, 865, 1193, 485, 216, 1849, 359, 1422, 391, 856, 1864, 470, 1888, 1698,
            760, 1778, 572, 1057, 48, 189, 1086, 1704, 1258, 192, 825, 585, 152, 1865, 1645, 807,
            225, 402, 1198, 1476, 600, 1914, 975, 1378, 1190, 24, 1550, 723, 696, 1131, 1831, 1880,
            1029, 713, 486, 126, 876, 1270, 1891, 544, 61, 1356, 1676, 1239, 36, 1177, 620, 1723,
            1651, 1136, 141, 1889, 1123, 624, 1519, 725, 241, 1253, 1119, 269, 763, 1120, 1620,
            642, 1713, 966, 1204, 558, 1344, 550, 316, 412, 886, 1309, 1648, 599, 1893, 265, 258,
            1561, 477, 1967, 66, 1296, 75, 1628, 715, 826, 1942, 1966, 1407, 159, 646, 1438, 1730,
            768, 411, 287, 499, 467, 46, 302, 661, 526, 848, 1327, 1097, 166, 413, 1578, 574, 1304,
            925, 504, 914, 978, 1352, 1103, 1859, 1167, 1318, 1454, 1990, 739, 1252, 132, 529,
            1622, 422, 1744, 1819, 425, 945, 1767, 1791, 976, 1226, 1092, 305, 479, 174, 626, 1063,
            662, 1948, 1978, 524, 512, 1255, 651, 1678, 1059,
        ];
        let a = 806;
        let b = 1994;
        let x = 326;
        let res = Solution::minimum_jumps(forbidden, a, b, x);
        assert_eq!(res, -1);
    }
}
