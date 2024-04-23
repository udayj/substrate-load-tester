use primitive_types::U256;
use scale_info::TypeInfo;
use sp_arithmetic::{fixed_point::FixedI128, FixedPointNumber};

pub trait FieldElementExt {
    fn to_u256(&self) -> U256;
}

#[derive(Clone, Default, PartialEq, TypeInfo, Debug)]
pub struct Order {
    pub account_id: U256,
    pub order_id: U256,
    pub market_id: u128,
    pub order_type: OrderType,
    pub direction: Direction,
    pub side: Side,
    pub price: FixedI128,
    pub size: FixedI128,
    pub leverage: FixedI128,
    pub slippage: FixedI128,
    pub post_only: bool,
    pub time_in_force: TimeInForce,
    pub signature_info: SignatureInfo,
    pub timestamp: u64,
}

impl Order {

    pub fn to_bytes(&self) -> Vec<u8> {

        let mut order = vec![];
        let mut buffer:[u8;32] = [0;32];
        self.account_id.to_little_endian(&mut buffer); 
        order.extend_from_slice(&buffer); //32
        self.order_id.to_little_endian(&mut buffer);
        order.extend_from_slice(&buffer); //32
        order.extend_from_slice(&self.market_id.to_le_bytes()); //16
        order.extend_from_slice(&self.order_type.to_bytes()); //1
        order.extend_from_slice(&self.direction.to_bytes()); //1
        order.extend_from_slice(&self.side.to_bytes()); //1
        order.extend_from_slice(&self.price.into_inner().to_le_bytes()); //16
        order.extend_from_slice(&self.size.into_inner().to_le_bytes()); //16
        order.extend_from_slice(&self.leverage.into_inner().to_le_bytes()); //16
        order.extend_from_slice(&self.slippage.into_inner().to_le_bytes()); //16
        match self.post_only { //1
            true => order.extend_from_slice(&[1]),
            false => order.extend_from_slice(&[0])
        }
        order.extend_from_slice(&self.time_in_force.to_bytes()); //1
        order.extend_from_slice(&self.timestamp.to_le_bytes()); //8
        order
    }
}
#[derive(Clone, Copy, Default, PartialEq, TypeInfo, Debug)]
pub enum OrderSide {
    #[default]
    Maker,
    Taker,
}

#[derive(Clone, Copy, Default, PartialEq, TypeInfo, Debug)]
pub enum Side {
    #[default]
    Buy,
    Sell,
}

#[derive(Clone, Copy, Default, PartialEq, TypeInfo, Debug)]
pub enum Direction {
    #[default]
    Long,
    Short,
}

#[derive(Clone, Copy, Default, PartialEq, TypeInfo, Debug)]
pub enum OrderType {
    #[default]
    Limit,
    Market,
    Forced,
}

impl OrderType {

    pub fn to_bytes(&self) -> [u8;1] {

        match self {
            OrderType::Limit => [0],
            OrderType::Market => [1],
            OrderType::Forced => [2]
        }
    }
}

impl Direction {

    pub fn to_bytes(&self) -> [u8;1] {

        match self {
            Direction::Long => [0],
            Direction::Short => [1]
        }
    }
}

impl Side {

    pub fn to_bytes(&self) -> [u8;1] {

        match self {
            Side::Buy => [0],
            Side::Sell => [1]
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, TypeInfo, Debug)]
pub enum TimeInForce {
    #[default]
    GTC,
    IOC,
    FOK,
}

impl TimeInForce {

    pub fn to_bytes(&self) -> [u8;1] {

        match self {
            TimeInForce::GTC => [0],
            TimeInForce::IOC => [1],
            TimeInForce::FOK => [2]
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, TypeInfo)]
pub enum ForceClosureFlag {
    #[default]
    Deleverage,
    Liquidate,
}

#[derive(Clone, Default, PartialEq, TypeInfo, Debug)]
pub struct SignatureInfo {
    pub liquidator_pub_key: U256,
    pub hash_type: HashType,
    pub sig_r: U256,
    pub sig_s: U256,
}

#[derive(Clone, Default, PartialEq, TypeInfo, Eq, Debug)]
pub enum HashType {
    #[default]
    Pedersen,
    Poseidon,
}

#[derive(Clone, Copy, Default, PartialEq, TypeInfo)]
pub struct BaseFee {
    pub volume: FixedI128,
    pub fee: FixedI128,
}

pub mod general_conversion_error {
    #[derive(Debug)]
    pub enum GeneralConversionError {
        U256ToFieldElementError,
        EnumToFieldElementError,
    }
}

pub struct Chains;
