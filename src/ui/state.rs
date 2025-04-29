#[derive(Clone)]
pub enum State {
    NotStarted,
    ArmySelection,
    DetachmentSelection {
        p1_army: usize,
        p2_army: usize,
    },
    Details {
        player_1: (usize, usize),
        player_2: (usize, usize),
    },
}
