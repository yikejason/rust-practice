use std::collections::HashMap;
pub(crate) fn two() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // 使用两种方法实现 team_map2
    // 提示:其中一种方法是使用 `collect` 方法
    // let teams_map2 = HashMap::from(teams);
    let teams_map2 = teams.into_iter().collect();

    assert_eq!(teams_map1, teams_map2);

    println!("Success!")
}
