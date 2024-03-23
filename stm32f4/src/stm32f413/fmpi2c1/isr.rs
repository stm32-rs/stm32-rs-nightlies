#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISRrs>;
#[doc = "TXE\n\nValue on reset: 1"]
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
#[doc = "Field `TXE` reader - TXE"]
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
#[doc = "TXE\n\nValue on reset: 1"]
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
#[doc = "Field `TXE` writer - TXE"]
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
#[doc = "TXIS\n\nValue on reset: 0"]
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
#[doc = "Field `TXIS` reader - TXIS"]
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
#[doc = "TXIS\n\nValue on reset: 0"]
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
#[doc = "Field `TXIS` writer - TXIS"]
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
#[doc = "RXNE\n\nValue on reset: 0"]
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
#[doc = "Field `RXNE` reader - RXNE"]
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
#[doc = "ADDR\n\nValue on reset: 0"]
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
#[doc = "Field `ADDR` reader - ADDR"]
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
#[doc = "NACKF\n\nValue on reset: 0"]
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
#[doc = "Field `NACKF` reader - NACKF"]
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
#[doc = "STOPF\n\nValue on reset: 0"]
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
#[doc = "Field `STOPF` reader - STOPF"]
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
#[doc = "TC\n\nValue on reset: 0"]
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
#[doc = "Field `TC` reader - TC"]
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
#[doc = "TCR\n\nValue on reset: 0"]
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
#[doc = "Field `TCR` reader - TCR"]
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
#[doc = "BERR\n\nValue on reset: 0"]
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
#[doc = "Field `BERR` reader - BERR"]
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
#[doc = "ARLO\n\nValue on reset: 0"]
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
#[doc = "Field `ARLO` reader - ARLO"]
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
#[doc = "OVR\n\nValue on reset: 0"]
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
#[doc = "Field `OVR` reader - OVR"]
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
#[doc = "PECERR\n\nValue on reset: 0"]
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
#[doc = "Field `PECERR` reader - PECERR"]
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
#[doc = "TIMEOUT\n\nValue on reset: 0"]
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
#[doc = "Field `TIMEOUT` reader - TIMEOUT"]
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
#[doc = "ALERT\n\nValue on reset: 0"]
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
#[doc = "Field `ALERT` reader - ALERT"]
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
#[doc = "BUSY\n\nValue on reset: 0"]
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
#[doc = "Field `BUSY` reader - BUSY"]
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
#[doc = "DIR\n\nValue on reset: 0"]
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
#[doc = "Field `DIR` reader - DIR"]
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
#[doc = "Field `ADDCODE` reader - ADDCODE"]
pub type ADDCODE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXIS"]
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NACKF"]
    #[inline(always)]
    pub fn nackf(&self) -> NACKF_R {
        NACKF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - STOPF"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TCR"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BERR"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ARLO"]
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PECERR"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIMEOUT"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ALERT"]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - ADDCODE"]
    #[inline(always)]
    pub fn addcode(&self) -> ADDCODE_R {
        ADDCODE_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TXE"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<ISRrs> {
        TXE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TXIS"]
    #[inline(always)]
    #[must_use]
    pub fn txis(&mut self) -> TXIS_W<ISRrs> {
        TXIS_W::new(self, 1)
    }
}
#[doc = "Interrupt and Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
