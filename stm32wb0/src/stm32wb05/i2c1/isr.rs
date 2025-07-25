///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
/**Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to '1' by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE = 0.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXER {
    ///0: TXDR register not empty
    NotEmpty = 0,
    ///1: TXDR register empty
    Empty = 1,
}
impl From<TXER> for bool {
    #[inline(always)]
    fn from(variant: TXER) -> Self {
        variant as u8 != 0
    }
}
///Field `TXE` reader - Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to '1' by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE = 0.
pub type TXE_R = crate::BitReader<TXER>;
impl TXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXER {
        match self.bits {
            false => TXER::NotEmpty,
            true => TXER::Empty,
        }
    }
    ///TXDR register not empty
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXER::NotEmpty
    }
    ///TXDR register empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXER::Empty
    }
}
/**Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to '1' by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE = 0.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEW {
    ///1: Flush the transmit data register
    Flush = 1,
}
impl From<TXEW> for bool {
    #[inline(always)]
    fn from(variant: TXEW) -> Self {
        variant as u8 != 0
    }
}
///Field `TXE` writer - Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to '1' by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE = 0.
pub type TXE_W<'a, REG> = crate::BitWriter1S<'a, REG, TXEW>;
impl<'a, REG> TXE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flush the transmit data register
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(TXEW::Flush)
    }
}
/**Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to '1' by software when NOSTRETCH = 1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN = 1). Note: This bit is cleared by hardware when PE = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXISR {
    ///0: The TXDR register is not empty
    NotEmpty = 0,
    ///1: The TXDR register is empty and the data to be transmitted must be written in the TXDR register
    Empty = 1,
}
impl From<TXISR> for bool {
    #[inline(always)]
    fn from(variant: TXISR) -> Self {
        variant as u8 != 0
    }
}
///Field `TXIS` reader - Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to '1' by software when NOSTRETCH = 1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN = 1). Note: This bit is cleared by hardware when PE = 0.
pub type TXIS_R = crate::BitReader<TXISR>;
impl TXIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXISR {
        match self.bits {
            false => TXISR::NotEmpty,
            true => TXISR::Empty,
        }
    }
    ///The TXDR register is not empty
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXISR::NotEmpty
    }
    ///The TXDR register is empty and the data to be transmitted must be written in the TXDR register
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXISR::Empty
    }
}
/**Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to '1' by software when NOSTRETCH = 1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN = 1). Note: This bit is cleared by hardware when PE = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXISW {
    ///1: Generate a TXIS event
    Trigger = 1,
}
impl From<TXISW> for bool {
    #[inline(always)]
    fn from(variant: TXISW) -> Self {
        variant as u8 != 0
    }
}
///Field `TXIS` writer - Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to '1' by software when NOSTRETCH = 1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN = 1). Note: This bit is cleared by hardware when PE = 0.
pub type TXIS_W<'a, REG> = crate::BitWriter1S<'a, REG, TXISW>;
impl<'a, REG> TXIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generate a TXIS event
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TXISW::Trigger)
    }
}
/**Receive data register not empty (receivers) This bit is set by hardware when the received data is copied into the I2C_RXDR register, and is ready to be read. It is cleared when I2C_RXDR is read. Note: This bit is cleared by hardware when PE = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE {
    ///0: The RXDR register is empty
    Empty = 0,
    ///1: Received data is copied into the RXDR register, and is ready to be read
    NotEmpty = 1,
}
impl From<RXNE> for bool {
    #[inline(always)]
    fn from(variant: RXNE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNE` reader - Receive data register not empty (receivers) This bit is set by hardware when the received data is copied into the I2C_RXDR register, and is ready to be read. It is cleared when I2C_RXDR is read. Note: This bit is cleared by hardware when PE = 0.
pub type RXNE_R = crate::BitReader<RXNE>;
impl RXNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXNE {
        match self.bits {
            false => RXNE::Empty,
            true => RXNE::NotEmpty,
        }
    }
    ///The RXDR register is empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNE::Empty
    }
    ///Received data is copied into the RXDR register, and is ready to be read
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNE::NotEmpty
    }
}
/**Address matched (slave mode) This bit is set by hardware as soon as the received slave address matched with one of the enabled slave addresses. It is cleared by software by setting ADDRCF bit. Note: This bit is cleared by hardware when PE = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR {
    ///0: Adress mismatched or not received
    NotMatch = 0,
    ///1: Received slave address matched with one of the enabled slave addresses
    Match = 1,
}
impl From<ADDR> for bool {
    #[inline(always)]
    fn from(variant: ADDR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDR` reader - Address matched (slave mode) This bit is set by hardware as soon as the received slave address matched with one of the enabled slave addresses. It is cleared by software by setting ADDRCF bit. Note: This bit is cleared by hardware when PE = 0.
pub type ADDR_R = crate::BitReader<ADDR>;
impl ADDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADDR {
        match self.bits {
            false => ADDR::NotMatch,
            true => ADDR::Match,
        }
    }
    ///Adress mismatched or not received
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        *self == ADDR::NotMatch
    }
    ///Received slave address matched with one of the enabled slave addresses
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ADDR::Match
    }
}
/**Not Acknowledge received flag This flag is set by hardware when a NACK is received after a byte transmission. It is cleared by software by setting the NACKCF bit. Note: This bit is cleared by hardware when PE = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKF {
    ///0: No NACK has been received
    NoNack = 0,
    ///1: NACK has been received
    Nack = 1,
}
impl From<NACKF> for bool {
    #[inline(always)]
    fn from(variant: NACKF) -> Self {
        variant as u8 != 0
    }
}
///Field `NACKF` reader - Not Acknowledge received flag This flag is set by hardware when a NACK is received after a byte transmission. It is cleared by software by setting the NACKCF bit. Note: This bit is cleared by hardware when PE = 0.
pub type NACKF_R = crate::BitReader<NACKF>;
impl NACKF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NACKF {
        match self.bits {
            false => NACKF::NoNack,
            true => NACKF::Nack,
        }
    }
    ///No NACK has been received
    #[inline(always)]
    pub fn is_no_nack(&self) -> bool {
        *self == NACKF::NoNack
    }
    ///NACK has been received
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == NACKF::Nack
    }
}
/**Stop detection flag This flag is set by hardware when a STOP condition is detected on the bus and the peripheral is involved in this transfer: either as a master, provided that the STOP condition is generated by the peripheral. or as a slave, provided that the peripheral has been addressed previously during this transfer. It is cleared by software by setting the STOPCF bit. Note: This bit is cleared by hardware when PE = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPF {
    ///0: No Stop condition detected
    NoStop = 0,
    ///1: Stop condition detected
    Stop = 1,
}
impl From<STOPF> for bool {
    #[inline(always)]
    fn from(variant: STOPF) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPF` reader - Stop detection flag This flag is set by hardware when a STOP condition is detected on the bus and the peripheral is involved in this transfer: either as a master, provided that the STOP condition is generated by the peripheral. or as a slave, provided that the peripheral has been addressed previously during this transfer. It is cleared by software by setting the STOPCF bit. Note: This bit is cleared by hardware when PE = 0.
pub type STOPF_R = crate::BitReader<STOPF>;
impl STOPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOPF {
        match self.bits {
            false => STOPF::NoStop,
            true => STOPF::Stop,
        }
    }
    ///No Stop condition detected
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPF::NoStop
    }
    ///Stop condition detected
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPF::Stop
    }
}
/**Transfer Complete (master mode) This flag is set by hardware when RELOAD=0, AUTOEND=0 and NBYTES data have been transferred. It is cleared by software when START bit or STOP bit is set. Note: This bit is cleared by hardware when PE = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC {
    ///0: Transfer is not complete
    NotComplete = 0,
    ///1: NBYTES has been transfered
    Complete = 1,
}
impl From<TC> for bool {
    #[inline(always)]
    fn from(variant: TC) -> Self {
        variant as u8 != 0
    }
}
///Field `TC` reader - Transfer Complete (master mode) This flag is set by hardware when RELOAD=0, AUTOEND=0 and NBYTES data have been transferred. It is cleared by software when START bit or STOP bit is set. Note: This bit is cleared by hardware when PE = 0.
pub type TC_R = crate::BitReader<TC>;
impl TC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TC {
        match self.bits {
            false => TC::NotComplete,
            true => TC::Complete,
        }
    }
    ///Transfer is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TC::NotComplete
    }
    ///NBYTES has been transfered
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TC::Complete
    }
}
/**Transfer Complete Reload This flag is set by hardware when RELOAD=1 and NBYTES data have been transferred. It is cleared by software when NBYTES is written to a non-zero value. Note: This bit is cleared by hardware when PE = 0. Note: This flag is only for master mode, or for slave mode when the SBC bit is set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCR {
    ///0: Transfer is not complete
    NotComplete = 0,
    ///1: NBYTES has been transfered
    Complete = 1,
}
impl From<TCR> for bool {
    #[inline(always)]
    fn from(variant: TCR) -> Self {
        variant as u8 != 0
    }
}
///Field `TCR` reader - Transfer Complete Reload This flag is set by hardware when RELOAD=1 and NBYTES data have been transferred. It is cleared by software when NBYTES is written to a non-zero value. Note: This bit is cleared by hardware when PE = 0. Note: This flag is only for master mode, or for slave mode when the SBC bit is set.
pub type TCR_R = crate::BitReader<TCR>;
impl TCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCR {
        match self.bits {
            false => TCR::NotComplete,
            true => TCR::Complete,
        }
    }
    ///Transfer is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCR::NotComplete
    }
    ///NBYTES has been transfered
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCR::Complete
    }
}
/**Bus error This flag is set by hardware when a misplaced Start or STOP condition is detected whereas the peripheral is involved in the transfer. The flag is not set during the address phase in slave mode. It is cleared by software by setting BERRCF bit. Note: This bit is cleared by hardware when PE = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERR {
    ///0: No bus error
    NoError = 0,
    ///1: Misplaced Start and Stop condition is detected
    Error = 1,
}
impl From<BERR> for bool {
    #[inline(always)]
    fn from(variant: BERR) -> Self {
        variant as u8 != 0
    }
}
///Field `BERR` reader - Bus error This flag is set by hardware when a misplaced Start or STOP condition is detected whereas the peripheral is involved in the transfer. The flag is not set during the address phase in slave mode. It is cleared by software by setting BERRCF bit. Note: This bit is cleared by hardware when PE = 0.
pub type BERR_R = crate::BitReader<BERR>;
impl BERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BERR {
        match self.bits {
            false => BERR::NoError,
            true => BERR::Error,
        }
    }
    ///No bus error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BERR::NoError
    }
    ///Misplaced Start and Stop condition is detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BERR::Error
    }
}
/**Arbitration lost This flag is set by hardware in case of arbitration loss. It is cleared by software by setting the ARLOCF bit. Note: This bit is cleared by hardware when PE = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLO {
    ///0: No arbitration lost
    NotLost = 0,
    ///1: Arbitration lost
    Lost = 1,
}
impl From<ARLO> for bool {
    #[inline(always)]
    fn from(variant: ARLO) -> Self {
        variant as u8 != 0
    }
}
///Field `ARLO` reader - Arbitration lost This flag is set by hardware in case of arbitration loss. It is cleared by software by setting the ARLOCF bit. Note: This bit is cleared by hardware when PE = 0.
pub type ARLO_R = crate::BitReader<ARLO>;
impl ARLO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARLO {
        match self.bits {
            false => ARLO::NotLost,
            true => ARLO::Lost,
        }
    }
    ///No arbitration lost
    #[inline(always)]
    pub fn is_not_lost(&self) -> bool {
        *self == ARLO::NotLost
    }
    ///Arbitration lost
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == ARLO::Lost
    }
}
/**Overrun/Underrun (slave mode) This flag is set by hardware in slave mode with NOSTRETCH = 1, when an overrun/underrun error occurs. It is cleared by software by setting the OVRCF bit. Note: This bit is cleared by hardware when PE = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR {
    ///0: No overrun/underrun error occurs
    NoOverrun = 0,
    ///1: slave mode with NOSTRETCH=1, when an overrun/underrun error occurs
    Overrun = 1,
}
impl From<OVR> for bool {
    #[inline(always)]
    fn from(variant: OVR) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` reader - Overrun/Underrun (slave mode) This flag is set by hardware in slave mode with NOSTRETCH = 1, when an overrun/underrun error occurs. It is cleared by software by setting the OVRCF bit. Note: This bit is cleared by hardware when PE = 0.
pub type OVR_R = crate::BitReader<OVR>;
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVR {
        match self.bits {
            false => OVR::NoOverrun,
            true => OVR::Overrun,
        }
    }
    ///No overrun/underrun error occurs
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR::NoOverrun
    }
    ///slave mode with NOSTRETCH=1, when an overrun/underrun error occurs
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR::Overrun
    }
}
/**PEC Error in reception This flag is set by hardware when the received PEC does not match with the PEC register content. A NACK is automatically sent after the wrong PEC reception. It is cleared by software by setting the PECCF bit. Note: This bit is cleared by hardware when PE = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to Section 52.3: I2C implementation.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECERR {
    ///0: Received PEC does match with PEC register
    Match = 0,
    ///1: Received PEC does not match with PEC register
    NoMatch = 1,
}
impl From<PECERR> for bool {
    #[inline(always)]
    fn from(variant: PECERR) -> Self {
        variant as u8 != 0
    }
}
///Field `PECERR` reader - PEC Error in reception This flag is set by hardware when the received PEC does not match with the PEC register content. A NACK is automatically sent after the wrong PEC reception. It is cleared by software by setting the PECCF bit. Note: This bit is cleared by hardware when PE = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to Section 52.3: I2C implementation.
pub type PECERR_R = crate::BitReader<PECERR>;
impl PECERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PECERR {
        match self.bits {
            false => PECERR::Match,
            true => PECERR::NoMatch,
        }
    }
    ///Received PEC does match with PEC register
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == PECERR::Match
    }
    ///Received PEC does not match with PEC register
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == PECERR::NoMatch
    }
}
/**Timeout or t<sub>LOW</sub> detection flag This flag is set by hardware when a timeout or extended clock timeout occurred. It is cleared by software by setting the TIMEOUTCF bit. Note: This bit is cleared by hardware when PE = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to Section 52.3: I2C implementation.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUT {
    ///0: No timeout occured
    NoTimeout = 0,
    ///1: Timeout occured
    Timeout = 1,
}
impl From<TIMEOUT> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMEOUT` reader - Timeout or t<sub>LOW</sub> detection flag This flag is set by hardware when a timeout or extended clock timeout occurred. It is cleared by software by setting the TIMEOUTCF bit. Note: This bit is cleared by hardware when PE = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to Section 52.3: I2C implementation.
pub type TIMEOUT_R = crate::BitReader<TIMEOUT>;
impl TIMEOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMEOUT {
        match self.bits {
            false => TIMEOUT::NoTimeout,
            true => TIMEOUT::Timeout,
        }
    }
    ///No timeout occured
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == TIMEOUT::NoTimeout
    }
    ///Timeout occured
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TIMEOUT::Timeout
    }
}
/**SMBus alert This flag is set by hardware when SMBHEN=1 (SMBus host configuration), ALERTEN=1 and a SMBALERT event (falling edge) is detected on SMBA pin. It is cleared by software by setting the ALERTCF bit. Note: This bit is cleared by hardware when PE = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to Section 52.3: I2C implementation.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERT {
    ///0: SMBA alert is not detected
    NoAlert = 0,
    ///1: SMBA alert event is detected on SMBA pin
    Alert = 1,
}
impl From<ALERT> for bool {
    #[inline(always)]
    fn from(variant: ALERT) -> Self {
        variant as u8 != 0
    }
}
///Field `ALERT` reader - SMBus alert This flag is set by hardware when SMBHEN=1 (SMBus host configuration), ALERTEN=1 and a SMBALERT event (falling edge) is detected on SMBA pin. It is cleared by software by setting the ALERTCF bit. Note: This bit is cleared by hardware when PE = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to Section 52.3: I2C implementation.
pub type ALERT_R = crate::BitReader<ALERT>;
impl ALERT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALERT {
        match self.bits {
            false => ALERT::NoAlert,
            true => ALERT::Alert,
        }
    }
    ///SMBA alert is not detected
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == ALERT::NoAlert
    }
    ///SMBA alert event is detected on SMBA pin
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == ALERT::Alert
    }
}
/**Bus busy This flag indicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected, or when PE = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    ///0: No communication is in progress on the bus
    NotBusy = 0,
    ///1: A communication is in progress on the bus
    Busy = 1,
}
impl From<BUSY> for bool {
    #[inline(always)]
    fn from(variant: BUSY) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSY` reader - Bus busy This flag indicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected, or when PE = 0.
pub type BUSY_R = crate::BitReader<BUSY>;
impl BUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSY {
        match self.bits {
            false => BUSY::NotBusy,
            true => BUSY::Busy,
        }
    }
    ///No communication is in progress on the bus
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY::NotBusy
    }
    ///A communication is in progress on the bus
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY::Busy
    }
}
/**Transfer direction (Slave mode) This flag is updated when an address match event occurs (ADDR = 1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR {
    ///0: Write transfer, slave enters receiver mode
    Write = 0,
    ///1: Read transfer, slave enters transmitter mode
    Read = 1,
}
impl From<DIR> for bool {
    #[inline(always)]
    fn from(variant: DIR) -> Self {
        variant as u8 != 0
    }
}
///Field `DIR` reader - Transfer direction (Slave mode) This flag is updated when an address match event occurs (ADDR = 1).
pub type DIR_R = crate::BitReader<DIR>;
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIR {
        match self.bits {
            false => DIR::Write,
            true => DIR::Read,
        }
    }
    ///Write transfer, slave enters receiver mode
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == DIR::Write
    }
    ///Read transfer, slave enters transmitter mode
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == DIR::Read
    }
}
///Field `ADDCODE` reader - Address match code (Slave mode) These bits are updated with the received address when an address match event occurs (ADDR = 1). In the case of a 10-bit address, ADDCODE provides the 10-bit header followed by the 2 MSBs of the address.
pub type ADDCODE_R = crate::FieldReader;
impl R {
    ///Bit 0 - Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to '1' by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE = 0.
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to '1' by software when NOSTRETCH = 1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN = 1). Note: This bit is cleared by hardware when PE = 0.
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive data register not empty (receivers) This bit is set by hardware when the received data is copied into the I2C_RXDR register, and is ready to be read. It is cleared when I2C_RXDR is read. Note: This bit is cleared by hardware when PE = 0.
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Address matched (slave mode) This bit is set by hardware as soon as the received slave address matched with one of the enabled slave addresses. It is cleared by software by setting ADDRCF bit. Note: This bit is cleared by hardware when PE = 0.
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Not Acknowledge received flag This flag is set by hardware when a NACK is received after a byte transmission. It is cleared by software by setting the NACKCF bit. Note: This bit is cleared by hardware when PE = 0.
    #[inline(always)]
    pub fn nackf(&self) -> NACKF_R {
        NACKF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Stop detection flag This flag is set by hardware when a STOP condition is detected on the bus and the peripheral is involved in this transfer: either as a master, provided that the STOP condition is generated by the peripheral. or as a slave, provided that the peripheral has been addressed previously during this transfer. It is cleared by software by setting the STOPCF bit. Note: This bit is cleared by hardware when PE = 0.
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transfer Complete (master mode) This flag is set by hardware when RELOAD=0, AUTOEND=0 and NBYTES data have been transferred. It is cleared by software when START bit or STOP bit is set. Note: This bit is cleared by hardware when PE = 0.
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transfer Complete Reload This flag is set by hardware when RELOAD=1 and NBYTES data have been transferred. It is cleared by software when NBYTES is written to a non-zero value. Note: This bit is cleared by hardware when PE = 0. Note: This flag is only for master mode, or for slave mode when the SBC bit is set.
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Bus error This flag is set by hardware when a misplaced Start or STOP condition is detected whereas the peripheral is involved in the transfer. The flag is not set during the address phase in slave mode. It is cleared by software by setting BERRCF bit. Note: This bit is cleared by hardware when PE = 0.
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Arbitration lost This flag is set by hardware in case of arbitration loss. It is cleared by software by setting the ARLOCF bit. Note: This bit is cleared by hardware when PE = 0.
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Overrun/Underrun (slave mode) This flag is set by hardware in slave mode with NOSTRETCH = 1, when an overrun/underrun error occurs. It is cleared by software by setting the OVRCF bit. Note: This bit is cleared by hardware when PE = 0.
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PEC Error in reception This flag is set by hardware when the received PEC does not match with the PEC register content. A NACK is automatically sent after the wrong PEC reception. It is cleared by software by setting the PECCF bit. Note: This bit is cleared by hardware when PE = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to Section 52.3: I2C implementation.
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Timeout or t<sub>LOW</sub> detection flag This flag is set by hardware when a timeout or extended clock timeout occurred. It is cleared by software by setting the TIMEOUTCF bit. Note: This bit is cleared by hardware when PE = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to Section 52.3: I2C implementation.
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SMBus alert This flag is set by hardware when SMBHEN=1 (SMBus host configuration), ALERTEN=1 and a SMBALERT event (falling edge) is detected on SMBA pin. It is cleared by software by setting the ALERTCF bit. Note: This bit is cleared by hardware when PE = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to Section 52.3: I2C implementation.
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Bus busy This flag indicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected, or when PE = 0.
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Transfer direction (Slave mode) This flag is updated when an address match event occurs (ADDR = 1).
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:23 - Address match code (Slave mode) These bits are updated with the received address when an address match event occurs (ADDR = 1). In the case of a 10-bit address, ADDCODE provides the 10-bit header followed by the 2 MSBs of the address.
    #[inline(always)]
    pub fn addcode(&self) -> ADDCODE_R {
        ADDCODE_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("txe", &self.txe())
            .field("txis", &self.txis())
            .field("rxne", &self.rxne())
            .field("addr", &self.addr())
            .field("nackf", &self.nackf())
            .field("stopf", &self.stopf())
            .field("tc", &self.tc())
            .field("tcr", &self.tcr())
            .field("berr", &self.berr())
            .field("arlo", &self.arlo())
            .field("ovr", &self.ovr())
            .field("pecerr", &self.pecerr())
            .field("timeout", &self.timeout())
            .field("alert", &self.alert())
            .field("busy", &self.busy())
            .field("dir", &self.dir())
            .field("addcode", &self.addcode())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to '1' by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE = 0.
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W<ISRrs> {
        TXE_W::new(self, 0)
    }
    ///Bit 1 - Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to '1' by software when NOSTRETCH = 1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN = 1). Note: This bit is cleared by hardware when PE = 0.
    #[inline(always)]
    pub fn txis(&mut self) -> TXIS_W<ISRrs> {
        TXIS_W::new(self, 1)
    }
}
/**I2C interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#I2C1:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
///`reset()` method sets ISR to value 0x01
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0x01;
}
