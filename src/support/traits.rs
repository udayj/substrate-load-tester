use super::converter::*;
use super::market_helper::*;
use super::types::general_conversion_error::*;
use super::types::*;
use primitive_types::U256;
use sp_arithmetic::{fixed_point::FixedI128, traits::CheckedDiv, FixedPointNumber};
use starknet_core::crypto::compute_hash_on_elements;
use starknet_crypto::poseidon_hash_many;
use starknet_crypto::{sign, FieldElement};
use starknet_ff::FromByteSliceError;
pub trait StringExt {
    fn to_felt_rep(&self) -> u128;
}

pub trait U256Ext {
    fn try_to_felt(&self) -> Result<FieldElement, FromByteSliceError>;
}

pub trait FixedI128Ext {
    fn round_to_precision(&self, precision: u32) -> Self;
    fn floor_with_precision(&self, precision: u32) -> Self;
    fn to_u256(&self) -> U256;
}
// This trait needs to be implemented by every type that can be hashed (pedersen or poseidon) and
// returns a FieldElement
pub trait Hashable {
    type ConversionError;
    fn hash(&self, hash_type: &HashType) -> Result<FieldElement, Self::ConversionError>;
}

// Impls
impl From<Direction> for u8 {
    fn from(value: Direction) -> u8 {
        match value {
            Direction::Long => 0_u8,
            Direction::Short => 1_u8,
        }
    }
}

impl From<Side> for u8 {
    fn from(value: Side) -> u8 {
        match value {
            Side::Buy => 0_u8,
            Side::Sell => 1_u8,
        }
    }
}

impl From<OrderType> for u8 {
    fn from(value: OrderType) -> u8 {
        match value {
            OrderType::Limit => 0_u8,
            OrderType::Market => 1_u8,
            OrderType::Forced => 2_u8,
        }
    }
}

impl From<TimeInForce> for u8 {
    fn from(value: TimeInForce) -> u8 {
        match value {
            TimeInForce::GTC => 0_u8,
            TimeInForce::IOC => 1_u8,
            TimeInForce::FOK => 2_u8,
        }
    }
}

impl From<Direction> for &str {
    fn from(value: Direction) -> &'static str {
        match value {
            Direction::Long => "LONG",
            Direction::Short => "SHORT",
        }
    }
}

impl From<Side> for &str {
    fn from(value: Side) -> &'static str {
        match value {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        }
    }
}

impl From<OrderType> for &str {
    fn from(value: OrderType) -> &'static str {
        match value {
            OrderType::Market => "MARKET",
            OrderType::Limit => "LIMIT",
            OrderType::Forced => "FORCED",
        }
    }
}

impl From<TimeInForce> for &str {
    fn from(value: TimeInForce) -> &'static str {
        match value {
            TimeInForce::GTC => "GTC",
            TimeInForce::FOK => "FOK",
            TimeInForce::IOC => "IOC",
        }
    }
}

impl From<ForceClosureFlag> for u8 {
    fn from(value: ForceClosureFlag) -> u8 {
        match value {
            ForceClosureFlag::Deleverage => 0_u8,
            ForceClosureFlag::Liquidate => 1_u8,
        }
    }
}

impl Hashable for Order {
    type ConversionError = GeneralConversionError;

    fn hash(&self, hash_type: &HashType) -> Result<FieldElement, Self::ConversionError> {
        let mut elements: Vec<FieldElement> = Vec::new();

        let (account_id_low, account_id_high) = convert_to_u128_pair(self.account_id)
            .map_err(|_err| GeneralConversionError::U256ToFieldElementError)?;
        elements.push(account_id_low);
        elements.push(account_id_high);

        let (order_id_low, order_id_high) = convert_to_u128_pair(self.order_id)
            .map_err(|_| GeneralConversionError::U256ToFieldElementError)?;
        elements.push(order_id_low);
        elements.push(order_id_high);

        elements.push(FieldElement::from(self.market_id));

        let order_type: &str = self.order_type.into();
        elements.push(
            FieldElement::from_hex_be(hex::encode(order_type).as_str())
                .map_err(|_err| GeneralConversionError::EnumToFieldElementError)?,
        );

        let direction: &str = self.direction.into();
        elements.push(
            FieldElement::from_hex_be(hex::encode(direction).as_str())
                .map_err(|_err| GeneralConversionError::EnumToFieldElementError)?,
        );

        let side: &str = self.side.into();
        elements.push(
            FieldElement::from_hex_be(hex::encode(side).as_str())
                .map_err(|_err| GeneralConversionError::EnumToFieldElementError)?,
        );

        let u256_representation = &self.price.to_u256();
        elements.push(
            u256_representation
                .try_to_felt()
                .map_err(|_err| GeneralConversionError::U256ToFieldElementError)?,
        );

        let u256_representation = &self.size.to_u256();
        elements.push(
            u256_representation
                .try_to_felt()
                .map_err(|_err| GeneralConversionError::U256ToFieldElementError)?,
        );

        let u256_representation = &self.leverage.to_u256();
        elements.push(
            u256_representation
                .try_to_felt()
                .map_err(|_err| GeneralConversionError::U256ToFieldElementError)?,
        );

        let u256_representation = &self.slippage.to_u256();
        elements.push(
            u256_representation
                .try_to_felt()
                .map_err(|_err| GeneralConversionError::U256ToFieldElementError)?,
        );

        match self.post_only {
            true => elements.push(FieldElement::from(1_u8)),
            false => elements.push(FieldElement::from(0_u8)),
        }

        let time_in_force: &str = self.time_in_force.into();
        elements.push(
            FieldElement::from_hex_be(hex::encode(time_in_force).as_str())
                .map_err(|_err| GeneralConversionError::EnumToFieldElementError)?,
        );

        elements.push(FieldElement::from(self.timestamp));

        match &hash_type {
            HashType::Pedersen => Ok(compute_hash_on_elements(&elements)),
            HashType::Poseidon => Ok(poseidon_hash_many(&elements)),
        }
    }
}

impl Order {
    pub fn new(order_id: U256, account_id: U256) -> Order {
        Order {
            account_id,
            order_id,
            market_id: btc_usdc().market.id,
            order_type: OrderType::Limit,
            direction: Direction::Long,
            side: Side::Buy,
            price: 1.into(),
            size: 1.into(),
            leverage: 1.into(),
            slippage: FixedI128::from_inner(100000000000000000),
            post_only: false,
            time_in_force: TimeInForce::GTC,
            signature_info: SignatureInfo {
                liquidator_pub_key: U256::zero(),
                hash_type: HashType::Pedersen,
                sig_r: U256::zero(),
                sig_s: U256::zero(),
            },
            //timestamp: 1699940278000,
            timestamp: 1711878950000,
        }
    }

    pub fn set_account_id(self: Order, account_id: U256) -> Order {
        Order { account_id, ..self }
    }

    pub fn set_order_id(self: Order, order_id: U256) -> Order {
        Order { order_id, ..self }
    }

    pub fn set_market_id(self: Order, market_id: u128) -> Order {
        Order { market_id, ..self }
    }

    pub fn set_order_type(self: Order, order_type: OrderType) -> Order {
        Order { order_type, ..self }
    }

    pub fn set_direction(self: Order, direction: Direction) -> Order {
        Order { direction, ..self }
    }

    pub fn set_side(self: Order, side: Side) -> Order {
        Order { side, ..self }
    }

    pub fn set_price(self: Order, price: FixedI128) -> Order {
        Order { price, ..self }
    }

    pub fn set_size(self: Order, size: FixedI128) -> Order {
        Order { size, ..self }
    }

    pub fn set_leverage(self: Order, leverage: FixedI128) -> Order {
        Order { leverage, ..self }
    }

    pub fn set_slippage(self: Order, slippage: FixedI128) -> Order {
        Order { slippage, ..self }
    }

    pub fn set_post_only(self: Order, post_only: bool) -> Order {
        Order { post_only, ..self }
    }

    pub fn set_time_in_force(self: Order, time_in_force: TimeInForce) -> Order {
        Order {
            time_in_force,
            ..self
        }
    }

    pub fn set_timestamp(self: Order, timestamp: u64) -> Order {
        Order { timestamp, ..self }
    }

    pub fn sign_order(self: Order, private_key: FieldElement) -> Order {
        let order_hash = self.hash(&self.signature_info.hash_type).unwrap();
        let signature = sign(&private_key, &order_hash, &FieldElement::ONE).unwrap();

        let sig_r = signature.r.to_u256();
        let sig_s = signature.s.to_u256();
        let signature_info = SignatureInfo {
            sig_r,
            sig_s,
            ..self.signature_info
        };
        Order {
            signature_info,
            ..self
        }
    }

    pub fn sign_order_liquidator(
        self: Order,
        private_key: FieldElement,
        liquidator_pub_key: U256,
    ) -> Order {
        let order_hash = self.hash(&self.signature_info.hash_type).unwrap();
        let signature = sign(&private_key, &order_hash, &FieldElement::ONE).unwrap();

        let sig_r = signature.r.to_u256();
        let sig_s = signature.s.to_u256();
        let signature_info = SignatureInfo {
            sig_r,
            sig_s,
            liquidator_pub_key,
            ..self.signature_info
        };
        Order {
            signature_info,
            ..self
        }
    }
}

impl StringExt for &str {
    fn to_felt_rep(&self) -> u128 {
        let a = FieldElement::from_byte_slice_be(self.as_bytes());
        u128::try_from(a.unwrap()).unwrap()
    }
}

impl U256Ext for U256 {
    fn try_to_felt(&self) -> Result<FieldElement, FromByteSliceError> {
        let mut buffer: [u8; 32] = [0; 32];
        self.to_big_endian(&mut buffer);
        FieldElement::from_byte_slice_be(&buffer)
    }
}

impl FieldElementExt for FieldElement {
    fn to_u256(&self) -> U256 {
        let buffer = self.to_bytes_be();
        U256::from_big_endian(&buffer)
    }
}

impl FixedI128Ext for FixedI128 {
    fn round_to_precision(&self, precision: u32) -> FixedI128 {
        // Get the inner value (number * 10^18) from FixedI128 representation
        let inner_value: i128 = FixedI128::into_inner(*self);
        // Get the integer part and decimal part separately
        let divisor: i128 = 10_u64.pow(18).into();
        let integer_part: i128 = inner_value / divisor;
        let decimal_part: i128 = inner_value % divisor;
        // Multiply decimal part with (10 ^ precision) and round it
        // so that now we have the required precision
        let ten_power_precision: i128 = 10_u64.pow(precision).into();
        let decimal_part: FixedI128 = FixedI128::from_inner(decimal_part * ten_power_precision);
        let decimal_part_rounded: FixedI128 = decimal_part.round();

        // Divide the decimal part with (10 ^ precision)
        // so that we get it to required decimal representaion
        let ten_power_precision: FixedI128 =
            FixedI128::checked_from_integer(ten_power_precision).unwrap();
        let decimal_part: FixedI128 = decimal_part_rounded
            .checked_div(&ten_power_precision)
            .unwrap();
        let integer_part: FixedI128 = FixedPointNumber::checked_from_integer(integer_part).unwrap();
        // Add both the parts together to get the final result
        let res: FixedI128 = FixedI128::add(integer_part, decimal_part);
        res
    }

    fn floor_with_precision(&self, precision: u32) -> FixedI128 {
        // Get the inner value (number * 10^18) from FixedI128 representation
        let inner_value: i128 = FixedI128::into_inner(*self);
        // Get the integer part and decimal part separately
        let divisor: i128 = 10_i128.pow(18);
        let integer_part: i128 = inner_value / divisor;
        let decimal_part: i128 = inner_value % divisor;
        // Multiply decimal part with (10 ^ precision) and round it
        // so that now we have the required precision
        let ten_power_precision: i128 = 10_i128.pow(precision);
        let decimal_part: FixedI128 = FixedI128::from_inner(decimal_part * ten_power_precision);
        let decimal_part_rounded: FixedI128 = decimal_part.floor();

        // Divide the decimal part with (10 ^ precision)
        // so that we get it to required decimal representaion
        let ten_power_precision: FixedI128 =
            FixedI128::checked_from_integer(ten_power_precision).unwrap();
        let decimal_part: FixedI128 = decimal_part_rounded
            .checked_div(&ten_power_precision)
            .unwrap();
        let integer_part: FixedI128 = FixedPointNumber::checked_from_integer(integer_part).unwrap();
        // Add both the parts together to get the final result
        let res: FixedI128 = FixedI128::add(integer_part, decimal_part);
        res
    }

    // Function to convert from fixed point number to U256 inside the PRIME field
    // This function does the appropriate mod arithmetic to ensure the returned value is actually
    // less than PRIME
    fn to_u256(&self) -> U256 {
        let inner_val: U256;
        // Max prime 2^251 + 17*2^192 + 1
        let prime: U256 = U256::from_dec_str(
            "3618502788666131213697322783095070105623107215331596699973092056135872020481",
        )
        .unwrap();
        // If the fixed point number is positive, we directly convert the inner val to U256
        if !self.is_negative() {
            inner_val = U256::from(self.into_inner());
        } else {
            // If the fixed point number is negative then we need to wrap the value
            // i.e. -x is equivalent to PRIME - x (or -x % PRIME) where x is a positive number
            inner_val = prime - (U256::from(-self.into_inner()));
        }
        inner_val
    }
}

pub trait ChainConstants {
    fn starknet_chain() -> u128;
    fn zkx_sync_chain() -> u128;
}

impl ChainConstants for Chains {
    fn starknet_chain() -> u128 {
        0x535441524b4e4554_u128
    }

    fn zkx_sync_chain() -> u128 {
        0x5a4b53594e43_u128
    }
}
