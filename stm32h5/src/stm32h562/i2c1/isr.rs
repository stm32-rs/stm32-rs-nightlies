#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISRrs>;
#[doc = "Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to ‘1’ by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE=0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXER {
    #[doc = "0: TXDR register not empty"]
    NotEmpty = 0,
    #[doc = "1: TXDR register empty"]
    Empty = 1,
}
impl From<TXER> for bool {
    #[inline(always)]
    fn from(variant: TXER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXE` reader - Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to ‘1’ by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE=0."]
pub type TXE_R = crate::BitReader<TXER>;
impl TXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXER {
        match self.bits {
            false => TXER::NotEmpty,
            true => TXER::Empty,
        }
    }
    #[doc = "TXDR register not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXER::NotEmpty
    }
    #[doc = "TXDR register empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXER::Empty
    }
}
#[doc = "Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to ‘1’ by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE=0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEW {
    #[doc = "1: Flush the transmit data register"]
    Flush = 1,
}
impl From<TXEW> for bool {
    #[inline(always)]
    fn from(variant: TXEW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXE` writer - Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to ‘1’ by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE=0."]
pub type TXE_W<'a, REG> = crate::BitWriter1S<'a, REG, TXEW>;
impl<'a, REG> TXE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flush the transmit data register"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(TXEW::Flush)
    }
}
#[doc = "Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to ‘1’ by software when NOSTRETCH=1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN=1). Note: This bit is cleared by hardware when PE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXISR {
    #[doc = "0: The TXDR register is not empty"]
    NotEmpty = 0,
    #[doc = "1: The TXDR register is empty and the data to be transmitted must be written in the TXDR register"]
    Empty = 1,
}
impl From<TXISR> for bool {
    #[inline(always)]
    fn from(variant: TXISR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIS` reader - Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to ‘1’ by software when NOSTRETCH=1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN=1). Note: This bit is cleared by hardware when PE=0."]
pub type TXIS_R = crate::BitReader<TXISR>;
impl TXIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXISR {
        match self.bits {
            false => TXISR::NotEmpty,
            true => TXISR::Empty,
        }
    }
    #[doc = "The TXDR register is not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXISR::NotEmpty
    }
    #[doc = "The TXDR register is empty and the data to be transmitted must be written in the TXDR register"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXISR::Empty
    }
}
#[doc = "Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to ‘1’ by software when NOSTRETCH=1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN=1). Note: This bit is cleared by hardware when PE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXISW {
    #[doc = "1: Generate a TXIS event"]
    Trigger = 1,
}
impl From<TXISW> for bool {
    #[inline(always)]
    fn from(variant: TXISW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIS` writer - Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to ‘1’ by software when NOSTRETCH=1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN=1). Note: This bit is cleared by hardware when PE=0."]
pub type TXIS_W<'a, REG> = crate::BitWriter1S<'a, REG, TXISW>;
impl<'a, REG> TXIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a TXIS event"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TXISW::Trigger)
    }
}
#[doc = "Receive data register not empty (receivers) This bit is set by hardware when the received data is copied into the I2C_RXDR register, and is ready to be read. It is cleared when I2C_RXDR is read. Note: This bit is cleared by hardware when PE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE {
    #[doc = "0: The RXDR register is empty"]
    Empty = 0,
    #[doc = "1: Received data is copied into the RXDR register, and is ready to be read"]
    NotEmpty = 1,
}
impl From<RXNE> for bool {
    #[inline(always)]
    fn from(variant: RXNE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNE` reader - Receive data register not empty (receivers) This bit is set by hardware when the received data is copied into the I2C_RXDR register, and is ready to be read. It is cleared when I2C_RXDR is read. Note: This bit is cleared by hardware when PE=0."]
pub type RXNE_R = crate::BitReader<RXNE>;
impl RXNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXNE {
        match self.bits {
            false => RXNE::Empty,
            true => RXNE::NotEmpty,
        }
    }
    #[doc = "The RXDR register is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNE::Empty
    }
    #[doc = "Received data is copied into the RXDR register, and is ready to be read"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNE::NotEmpty
    }
}
#[doc = "Address matched (slave mode) This bit is set by hardware as soon as the received slave address matched with one of the enabled slave addresses. It is cleared by software by setting ADDRCF bit. Note: This bit is cleared by hardware when PE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR {
    #[doc = "0: Adress mismatched or not received"]
    NotMatch = 0,
    #[doc = "1: Received slave address matched with one of the enabled slave addresses"]
    Match = 1,
}
impl From<ADDR> for bool {
    #[inline(always)]
    fn from(variant: ADDR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR` reader - Address matched (slave mode) This bit is set by hardware as soon as the received slave address matched with one of the enabled slave addresses. It is cleared by software by setting ADDRCF bit. Note: This bit is cleared by hardware when PE=0."]
pub type ADDR_R = crate::BitReader<ADDR>;
impl ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDR {
        match self.bits {
            false => ADDR::NotMatch,
            true => ADDR::Match,
        }
    }
    #[doc = "Adress mismatched or not received"]
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        *self == ADDR::NotMatch
    }
    #[doc = "Received slave address matched with one of the enabled slave addresses"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ADDR::Match
    }
}
#[doc = "Not Acknowledge received flag This flag is set by hardware when a NACK is received after a byte transmission. It is cleared by software by setting the NACKCF bit. Note: This bit is cleared by hardware when PE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKF {
    #[doc = "0: No NACK has been received"]
    NoNack = 0,
    #[doc = "1: NACK has been received"]
    Nack = 1,
}
impl From<NACKF> for bool {
    #[inline(always)]
    fn from(variant: NACKF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKF` reader - Not Acknowledge received flag This flag is set by hardware when a NACK is received after a byte transmission. It is cleared by software by setting the NACKCF bit. Note: This bit is cleared by hardware when PE=0."]
pub type NACKF_R = crate::BitReader<NACKF>;
impl NACKF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NACKF {
        match self.bits {
            false => NACKF::NoNack,
            true => NACKF::Nack,
        }
    }
    #[doc = "No NACK has been received"]
    #[inline(always)]
    pub fn is_no_nack(&self) -> bool {
        *self == NACKF::NoNack
    }
    #[doc = "NACK has been received"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == NACKF::Nack
    }
}
#[doc = "Stop detection flag This flag is set by hardware when a STOP condition is detected on the bus and the peripheral is involved in this transfer: either as a master, provided that the STOP condition is generated by the peripheral. or as a slave, provided that the peripheral has been addressed previously during this transfer. It is cleared by software by setting the STOPCF bit. Note: This bit is cleared by hardware when PE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPF {
    #[doc = "0: No Stop condition detected"]
    NoStop = 0,
    #[doc = "1: Stop condition detected"]
    Stop = 1,
}
impl From<STOPF> for bool {
    #[inline(always)]
    fn from(variant: STOPF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPF` reader - Stop detection flag This flag is set by hardware when a STOP condition is detected on the bus and the peripheral is involved in this transfer: either as a master, provided that the STOP condition is generated by the peripheral. or as a slave, provided that the peripheral has been addressed previously during this transfer. It is cleared by software by setting the STOPCF bit. Note: This bit is cleared by hardware when PE=0."]
pub type STOPF_R = crate::BitReader<STOPF>;
impl STOPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPF {
        match self.bits {
            false => STOPF::NoStop,
            true => STOPF::Stop,
        }
    }
    #[doc = "No Stop condition detected"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPF::NoStop
    }
    #[doc = "Stop condition detected"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPF::Stop
    }
}
#[doc = "Transfer Complete (master mode) This flag is set by hardware when RELOAD=0, AUTOEND=0 and NBYTES data have been transferred. It is cleared by software when START bit or STOP bit is set. Note: This bit is cleared by hardware when PE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC {
    #[doc = "0: Transfer is not complete"]
    NotComplete = 0,
    #[doc = "1: NBYTES has been transfered"]
    Complete = 1,
}
impl From<TC> for bool {
    #[inline(always)]
    fn from(variant: TC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC` reader - Transfer Complete (master mode) This flag is set by hardware when RELOAD=0, AUTOEND=0 and NBYTES data have been transferred. It is cleared by software when START bit or STOP bit is set. Note: This bit is cleared by hardware when PE=0."]
pub type TC_R = crate::BitReader<TC>;
impl TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC {
        match self.bits {
            false => TC::NotComplete,
            true => TC::Complete,
        }
    }
    #[doc = "Transfer is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TC::NotComplete
    }
    #[doc = "NBYTES has been transfered"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TC::Complete
    }
}
#[doc = "Transfer Complete Reload This flag is set by hardware when RELOAD=1 and NBYTES data have been transferred. It is cleared by software when NBYTES is written to a non-zero value. Note: This bit is cleared by hardware when PE=0. This flag is only for master mode, or for slave mode when the SBC bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCR {
    #[doc = "0: Transfer is not complete"]
    NotComplete = 0,
    #[doc = "1: NBYTES has been transfered"]
    Complete = 1,
}
impl From<TCR> for bool {
    #[inline(always)]
    fn from(variant: TCR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCR` reader - Transfer Complete Reload This flag is set by hardware when RELOAD=1 and NBYTES data have been transferred. It is cleared by software when NBYTES is written to a non-zero value. Note: This bit is cleared by hardware when PE=0. This flag is only for master mode, or for slave mode when the SBC bit is set."]
pub type TCR_R = crate::BitReader<TCR>;
impl TCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCR {
        match self.bits {
            false => TCR::NotComplete,
            true => TCR::Complete,
        }
    }
    #[doc = "Transfer is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCR::NotComplete
    }
    #[doc = "NBYTES has been transfered"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCR::Complete
    }
}
#[doc = "Bus error This flag is set by hardware when a misplaced Start or STOP condition is detected whereas the peripheral is involved in the transfer. The flag is not set during the address phase in slave mode. It is cleared by software by setting BERRCF bit. Note: This bit is cleared by hardware when PE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERR {
    #[doc = "0: No bus error"]
    NoError = 0,
    #[doc = "1: Misplaced Start and Stop condition is detected"]
    Error = 1,
}
impl From<BERR> for bool {
    #[inline(always)]
    fn from(variant: BERR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BERR` reader - Bus error This flag is set by hardware when a misplaced Start or STOP condition is detected whereas the peripheral is involved in the transfer. The flag is not set during the address phase in slave mode. It is cleared by software by setting BERRCF bit. Note: This bit is cleared by hardware when PE=0."]
pub type BERR_R = crate::BitReader<BERR>;
impl BERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BERR {
        match self.bits {
            false => BERR::NoError,
            true => BERR::Error,
        }
    }
    #[doc = "No bus error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BERR::NoError
    }
    #[doc = "Misplaced Start and Stop condition is detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BERR::Error
    }
}
#[doc = "Arbitration lost This flag is set by hardware in case of arbitration loss. It is cleared by software by setting the ARLOCF bit. Note: This bit is cleared by hardware when PE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLO {
    #[doc = "0: No arbitration lost"]
    NotLost = 0,
    #[doc = "1: Arbitration lost"]
    Lost = 1,
}
impl From<ARLO> for bool {
    #[inline(always)]
    fn from(variant: ARLO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARLO` reader - Arbitration lost This flag is set by hardware in case of arbitration loss. It is cleared by software by setting the ARLOCF bit. Note: This bit is cleared by hardware when PE=0."]
pub type ARLO_R = crate::BitReader<ARLO>;
impl ARLO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARLO {
        match self.bits {
            false => ARLO::NotLost,
            true => ARLO::Lost,
        }
    }
    #[doc = "No arbitration lost"]
    #[inline(always)]
    pub fn is_not_lost(&self) -> bool {
        *self == ARLO::NotLost
    }
    #[doc = "Arbitration lost"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == ARLO::Lost
    }
}
#[doc = "Overrun/Underrun (slave mode) This flag is set by hardware in slave mode with NOSTRETCH=1, when an overrun/underrun error occurs. It is cleared by software by setting the OVRCF bit. Note: This bit is cleared by hardware when PE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR {
    #[doc = "0: No overrun/underrun error occurs"]
    NoOverrun = 0,
    #[doc = "1: slave mode with NOSTRETCH=1, when an overrun/underrun error occurs"]
    Overrun = 1,
}
impl From<OVR> for bool {
    #[inline(always)]
    fn from(variant: OVR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` reader - Overrun/Underrun (slave mode) This flag is set by hardware in slave mode with NOSTRETCH=1, when an overrun/underrun error occurs. It is cleared by software by setting the OVRCF bit. Note: This bit is cleared by hardware when PE=0."]
pub type OVR_R = crate::BitReader<OVR>;
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR {
        match self.bits {
            false => OVR::NoOverrun,
            true => OVR::Overrun,
        }
    }
    #[doc = "No overrun/underrun error occurs"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR::NoOverrun
    }
    #[doc = "slave mode with NOSTRETCH=1, when an overrun/underrun error occurs"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR::Overrun
    }
}
#[doc = "PEC Error in reception This flag is set by hardware when the received PEC does not match with the PEC register content. A NACK is automatically sent after the wrong PEC reception. It is cleared by software by setting the PECCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECERR {
    #[doc = "0: Received PEC does match with PEC register"]
    Match = 0,
    #[doc = "1: Received PEC does not match with PEC register"]
    NoMatch = 1,
}
impl From<PECERR> for bool {
    #[inline(always)]
    fn from(variant: PECERR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECERR` reader - PEC Error in reception This flag is set by hardware when the received PEC does not match with the PEC register content. A NACK is automatically sent after the wrong PEC reception. It is cleared by software by setting the PECCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type PECERR_R = crate::BitReader<PECERR>;
impl PECERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PECERR {
        match self.bits {
            false => PECERR::Match,
            true => PECERR::NoMatch,
        }
    }
    #[doc = "Received PEC does match with PEC register"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == PECERR::Match
    }
    #[doc = "Received PEC does not match with PEC register"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == PECERR::NoMatch
    }
}
#[doc = "Timeout or tLOW detection flag This flag is set by hardware when a timeout or extended clock timeout occurred. It is cleared by software by setting the TIMEOUTCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUT {
    #[doc = "0: No timeout occured"]
    NoTimeout = 0,
    #[doc = "1: Timeout occured"]
    Timeout = 1,
}
impl From<TIMEOUT> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` reader - Timeout or tLOW detection flag This flag is set by hardware when a timeout or extended clock timeout occurred. It is cleared by software by setting the TIMEOUTCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type TIMEOUT_R = crate::BitReader<TIMEOUT>;
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMEOUT {
        match self.bits {
            false => TIMEOUT::NoTimeout,
            true => TIMEOUT::Timeout,
        }
    }
    #[doc = "No timeout occured"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == TIMEOUT::NoTimeout
    }
    #[doc = "Timeout occured"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TIMEOUT::Timeout
    }
}
#[doc = "SMBus alert This flag is set by hardware when SMBHEN=1 (SMBus host configuration), ALERTEN=1 and a SMBALERT event (falling edge) is detected on SMBA pin. It is cleared by software by setting the ALERTCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERT {
    #[doc = "0: SMBA alert is not detected"]
    NoAlert = 0,
    #[doc = "1: SMBA alert event is detected on SMBA pin"]
    Alert = 1,
}
impl From<ALERT> for bool {
    #[inline(always)]
    fn from(variant: ALERT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERT` reader - SMBus alert This flag is set by hardware when SMBHEN=1 (SMBus host configuration), ALERTEN=1 and a SMBALERT event (falling edge) is detected on SMBA pin. It is cleared by software by setting the ALERTCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type ALERT_R = crate::BitReader<ALERT>;
impl ALERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALERT {
        match self.bits {
            false => ALERT::NoAlert,
            true => ALERT::Alert,
        }
    }
    #[doc = "SMBA alert is not detected"]
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == ALERT::NoAlert
    }
    #[doc = "SMBA alert event is detected on SMBA pin"]
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == ALERT::Alert
    }
}
#[doc = "Bus busy This flag indicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected, or when PE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    #[doc = "0: No communication is in progress on the bus"]
    NotBusy = 0,
    #[doc = "1: A communication is in progress on the bus"]
    Busy = 1,
}
impl From<BUSY> for bool {
    #[inline(always)]
    fn from(variant: BUSY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Bus busy This flag indicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected, or when PE=0."]
pub type BUSY_R = crate::BitReader<BUSY>;
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY {
        match self.bits {
            false => BUSY::NotBusy,
            true => BUSY::Busy,
        }
    }
    #[doc = "No communication is in progress on the bus"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY::NotBusy
    }
    #[doc = "A communication is in progress on the bus"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY::Busy
    }
}
#[doc = "Transfer direction (Slave mode) This flag is updated when an address match event occurs (ADDR=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR {
    #[doc = "0: Write transfer, slave enters receiver mode"]
    Write = 0,
    #[doc = "1: Read transfer, slave enters transmitter mode"]
    Read = 1,
}
impl From<DIR> for bool {
    #[inline(always)]
    fn from(variant: DIR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Transfer direction (Slave mode) This flag is updated when an address match event occurs (ADDR=1)."]
pub type DIR_R = crate::BitReader<DIR>;
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIR {
        match self.bits {
            false => DIR::Write,
            true => DIR::Read,
        }
    }
    #[doc = "Write transfer, slave enters receiver mode"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == DIR::Write
    }
    #[doc = "Read transfer, slave enters transmitter mode"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == DIR::Read
    }
}
#[doc = "Field `ADDCODE` reader - Address match code (Slave mode) These bits are updated with the received address when an address match event occurs (ADDR = 1). In the case of a 10-bit address, ADDCODE provides the 10-bit header followed by the 2 MSBs of the address."]
pub type ADDCODE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to ‘1’ by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE=0."]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to ‘1’ by software when NOSTRETCH=1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN=1). Note: This bit is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive data register not empty (receivers) This bit is set by hardware when the received data is copied into the I2C_RXDR register, and is ready to be read. It is cleared when I2C_RXDR is read. Note: This bit is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address matched (slave mode) This bit is set by hardware as soon as the received slave address matched with one of the enabled slave addresses. It is cleared by software by setting ADDRCF bit. Note: This bit is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not Acknowledge received flag This flag is set by hardware when a NACK is received after a byte transmission. It is cleared by software by setting the NACKCF bit. Note: This bit is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn nackf(&self) -> NACKF_R {
        NACKF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop detection flag This flag is set by hardware when a STOP condition is detected on the bus and the peripheral is involved in this transfer: either as a master, provided that the STOP condition is generated by the peripheral. or as a slave, provided that the peripheral has been addressed previously during this transfer. It is cleared by software by setting the STOPCF bit. Note: This bit is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete (master mode) This flag is set by hardware when RELOAD=0, AUTOEND=0 and NBYTES data have been transferred. It is cleared by software when START bit or STOP bit is set. Note: This bit is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer Complete Reload This flag is set by hardware when RELOAD=1 and NBYTES data have been transferred. It is cleared by software when NBYTES is written to a non-zero value. Note: This bit is cleared by hardware when PE=0. This flag is only for master mode, or for slave mode when the SBC bit is set."]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error This flag is set by hardware when a misplaced Start or STOP condition is detected whereas the peripheral is involved in the transfer. The flag is not set during the address phase in slave mode. It is cleared by software by setting BERRCF bit. Note: This bit is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost This flag is set by hardware in case of arbitration loss. It is cleared by software by setting the ARLOCF bit. Note: This bit is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun/Underrun (slave mode) This flag is set by hardware in slave mode with NOSTRETCH=1, when an overrun/underrun error occurs. It is cleared by software by setting the OVRCF bit. Note: This bit is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PEC Error in reception This flag is set by hardware when the received PEC does not match with the PEC register content. A NACK is automatically sent after the wrong PEC reception. It is cleared by software by setting the PECCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timeout or tLOW detection flag This flag is set by hardware when a timeout or extended clock timeout occurred. It is cleared by software by setting the TIMEOUTCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert This flag is set by hardware when SMBHEN=1 (SMBus host configuration), ALERTEN=1 and a SMBALERT event (falling edge) is detected on SMBA pin. It is cleared by software by setting the ALERTCF bit. Note: This bit is cleared by hardware when PE=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Bus busy This flag indicates that a communication is in progress on the bus. It is set by hardware when a START condition is detected. It is cleared by hardware when a STOP condition is detected, or when PE=0."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transfer direction (Slave mode) This flag is updated when an address match event occurs (ADDR=1)."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Address match code (Slave mode) These bits are updated with the received address when an address match event occurs (ADDR = 1). In the case of a 10-bit address, ADDCODE provides the 10-bit header followed by the 2 MSBs of the address."]
    #[inline(always)]
    pub fn addcode(&self) -> ADDCODE_R {
        ADDCODE_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit data register empty (transmitters) This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to ‘1’ by software in order to flush the transmit data register I2C_TXDR. Note: This bit is set by hardware when PE=0."]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<ISRrs> {
        TXE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit interrupt status (transmitters) This bit is set by hardware when the I2C_TXDR register is empty and the data to be transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be sent is written in the I2C_TXDR register. This bit can be written to ‘1’ by software when NOSTRETCH=1 only, in order to generate a TXIS event (interrupt if TXIE=1 or DMA request if TXDMAEN=1). Note: This bit is cleared by hardware when PE=0."]
    #[inline(always)]
    #[must_use]
    pub fn txis(&mut self) -> TXIS_W<ISRrs> {
        TXIS_W::new(self, 1)
    }
}
#[doc = "I2C interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets ISR to value 0x01"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0x01;
}
