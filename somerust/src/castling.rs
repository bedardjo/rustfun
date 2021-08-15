#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Castling {
  WhiteShort, WhiteLong,
  BlackShort, BlackLong
}