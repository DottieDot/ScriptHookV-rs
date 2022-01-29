use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum VehicleClass {
  Compacts       = 0,
  Sedans         = 1,
  SUVs           = 2,
  Coupes         = 3,
  Muscle         = 4,
  SportsClassics = 5,
  Sports         = 6,
  Super          = 7,
  Motorcycles    = 8,
  OffRoad        = 9,
  Industrial     = 10,
  Utility        = 11,
  Vans           = 12,
  Cycles         = 13,
  Boats          = 14,
  Helicopters    = 15,
  Planes         = 16,
  Service        = 17,
  Emergency      = 18,
  Military       = 19,
  Commercial     = 20,
  Trains         = 21,
  OpenWheel      = 22
}
