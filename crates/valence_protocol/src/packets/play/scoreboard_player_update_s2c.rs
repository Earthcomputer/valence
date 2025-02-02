use super::*;

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(id = packet_id::SCOREBOARD_PLAYER_UPDATE_S2C)]
pub struct ScoreboardPlayerUpdateS2c<'a> {
    pub entity_name: &'a str,
    pub action: ScoreboardPlayerUpdateAction<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum ScoreboardPlayerUpdateAction<'a> {
    Update {
        objective_name: &'a str,
        objective_score: VarInt,
    },
    Remove {
        objective_name: &'a str,
    },
}
