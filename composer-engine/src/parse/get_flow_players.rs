use crate::score::flows::Flow;
use crate::score::instruments::Instrument;
use crate::score::players::Player;
use crate::score::stave::Stave;
use crate::Engine;

impl Engine {
    pub fn get_flow_players(
        &self,
        flow_key: &str,
    ) -> (&Flow, Vec<&Player>, Vec<&Instrument>, Vec<&Stave>) {
        let mut players: Vec<&Player> = Vec::new();
        let mut instruments: Vec<&Instrument> = Vec::new();
        let mut staves: Vec<&Stave> = Vec::new();

        let flow = self.score.flows.by_key.get(flow_key).unwrap();
        for player_key in &self.score.players.order {
            if flow.players.contains(player_key) {
                let player = self.score.players.by_key.get(player_key).unwrap();
                players.push(player);
                for instrument_key in &player.instruments {
                    let instrument = self.score.instruments.get(instrument_key).unwrap();
                    instruments.push(instrument);

                    for stave_key in &instrument.staves {
                        let stave = flow.staves.get(stave_key).unwrap();
                        staves.push(stave);
                    }
                }
            }
        }
        (flow, players, instruments, staves)
    }
}
