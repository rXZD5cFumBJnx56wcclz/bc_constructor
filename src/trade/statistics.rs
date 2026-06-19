use crate::trade::structs::TradeCell;

pub struct StatCollector {
    pub cells: Vec<TradeCell>,
}
impl StatCollector {
    pub fn push(
        &mut self,
        cell: TradeCell,
    ) {
        self.cells.push(cell);
    }
}
