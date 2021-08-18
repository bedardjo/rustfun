use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Castling {
  WhiteShort, WhiteLong,
  BlackShort, BlackLong
}

pub fn all_castling() -> HashSet<Castling> {
  let mut all_castling = HashSet::new();
  all_castling.insert(Castling::WhiteShort);
  all_castling.insert(Castling::WhiteLong);
  all_castling.insert(Castling::BlackShort);
  all_castling.insert(Castling::BlackLong);
  return all_castling;
}