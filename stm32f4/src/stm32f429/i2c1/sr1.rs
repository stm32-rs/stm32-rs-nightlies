///Register `SR1` reader
pub type R = crate::R<SR1rs>;
///Register `SR1` writer
pub type W = crate::W<SR1rs>;
/**Start bit (Master mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SB {
    ///0: No Start condition
    NoStart = 0,
    ///1: Start condition generated
    Start = 1,
}
impl From<SB> for bool {
    #[inline(always)]
    fn from(variant: SB) -> Self {
        variant as u8 != 0
    }
}
///Field `SB` reader - Start bit (Master mode)
pub type SB_R = crate::BitReader<SB>;
impl SB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SB {
        match self.bits {
            false => SB::NoStart,
            true => SB::Start,
        }
    }
    ///No Start condition
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == SB::NoStart
    }
    ///Start condition generated
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SB::Start
    }
}
/**Address sent (master mode)/matched (slave mode)

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
///Field `ADDR` reader - Address sent (master mode)/matched (slave mode)
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
/**Byte transfer finished

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BTF {
    ///0: Data byte transfer not done
    NotFinished = 0,
    ///1: Data byte transfer successful
    Finished = 1,
}
impl From<BTF> for bool {
    #[inline(always)]
    fn from(variant: BTF) -> Self {
        variant as u8 != 0
    }
}
///Field `BTF` reader - Byte transfer finished
pub type BTF_R = crate::BitReader<BTF>;
impl BTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BTF {
        match self.bits {
            false => BTF::NotFinished,
            true => BTF::Finished,
        }
    }
    ///Data byte transfer not done
    #[inline(always)]
    pub fn is_not_finished(&self) -> bool {
        *self == BTF::NotFinished
    }
    ///Data byte transfer successful
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == BTF::Finished
    }
}
///Field `ADD10` reader - 10-bit header sent (Master mode)
pub type ADD10_R = crate::BitReader;
/**Stop detection (slave mode)

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
///Field `STOPF` reader - Stop detection (slave mode)
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
/**Data register not empty (receivers)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_NE {
    ///0: Data register empty
    Empty = 0,
    ///1: Data register not empty
    NotEmpty = 1,
}
impl From<RX_NE> for bool {
    #[inline(always)]
    fn from(variant: RX_NE) -> Self {
        variant as u8 != 0
    }
}
///Field `RxNE` reader - Data register not empty (receivers)
pub type RX_NE_R = crate::BitReader<RX_NE>;
impl RX_NE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RX_NE {
        match self.bits {
            false => RX_NE::Empty,
            true => RX_NE::NotEmpty,
        }
    }
    ///Data register empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RX_NE::Empty
    }
    ///Data register not empty
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RX_NE::NotEmpty
    }
}
/**Data register empty (transmitters)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_E {
    ///0: Data register not empty
    NotEmpty = 0,
    ///1: Data register empty
    Empty = 1,
}
impl From<TX_E> for bool {
    #[inline(always)]
    fn from(variant: TX_E) -> Self {
        variant as u8 != 0
    }
}
///Field `TxE` reader - Data register empty (transmitters)
pub type TX_E_R = crate::BitReader<TX_E>;
impl TX_E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TX_E {
        match self.bits {
            false => TX_E::NotEmpty,
            true => TX_E::Empty,
        }
    }
    ///Data register not empty
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TX_E::NotEmpty
    }
    ///Data register empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TX_E::Empty
    }
}
/**Bus error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERRR {
    ///0: No misplaced Start or Stop condition
    NoError = 0,
    ///1: Misplaced Start or Stop condition
    Error = 1,
}
impl From<BERRR> for bool {
    #[inline(always)]
    fn from(variant: BERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `BERR` reader - Bus error
pub type BERR_R = crate::BitReader<BERRR>;
impl BERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BERRR {
        match self.bits {
            false => BERRR::NoError,
            true => BERRR::Error,
        }
    }
    ///No misplaced Start or Stop condition
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BERRR::NoError
    }
    ///Misplaced Start or Stop condition
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BERRR::Error
    }
}
/**Bus error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERRW {
    ///0: Clear flag
    Clear = 0,
}
impl From<BERRW> for bool {
    #[inline(always)]
    fn from(variant: BERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `BERR` writer - Bus error
pub type BERR_W<'a, REG> = crate::BitWriter0C<'a, REG, BERRW>;
impl<'a, REG> BERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BERRW::Clear)
    }
}
/**Arbitration lost (master mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLOR {
    ///0: No Arbitration Lost detected
    NoLost = 0,
    ///1: Arbitration Lost detected
    Lost = 1,
}
impl From<ARLOR> for bool {
    #[inline(always)]
    fn from(variant: ARLOR) -> Self {
        variant as u8 != 0
    }
}
///Field `ARLO` reader - Arbitration lost (master mode)
pub type ARLO_R = crate::BitReader<ARLOR>;
impl ARLO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARLOR {
        match self.bits {
            false => ARLOR::NoLost,
            true => ARLOR::Lost,
        }
    }
    ///No Arbitration Lost detected
    #[inline(always)]
    pub fn is_no_lost(&self) -> bool {
        *self == ARLOR::NoLost
    }
    ///Arbitration Lost detected
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == ARLOR::Lost
    }
}
/**Arbitration lost (master mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLOW {
    ///0: Clear flag
    Clear = 0,
}
impl From<ARLOW> for bool {
    #[inline(always)]
    fn from(variant: ARLOW) -> Self {
        variant as u8 != 0
    }
}
///Field `ARLO` writer - Arbitration lost (master mode)
pub type ARLO_W<'a, REG> = crate::BitWriter0C<'a, REG, ARLOW>;
impl<'a, REG> ARLO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ARLOW::Clear)
    }
}
/**Acknowledge failure

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFR {
    ///0: No acknowledge failure
    NoFailure = 0,
    ///1: Acknowledge failure
    Failure = 1,
}
impl From<AFR> for bool {
    #[inline(always)]
    fn from(variant: AFR) -> Self {
        variant as u8 != 0
    }
}
///Field `AF` reader - Acknowledge failure
pub type AF_R = crate::BitReader<AFR>;
impl AF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFR {
        match self.bits {
            false => AFR::NoFailure,
            true => AFR::Failure,
        }
    }
    ///No acknowledge failure
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == AFR::NoFailure
    }
    ///Acknowledge failure
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == AFR::Failure
    }
}
/**Acknowledge failure

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<AFW> for bool {
    #[inline(always)]
    fn from(variant: AFW) -> Self {
        variant as u8 != 0
    }
}
///Field `AF` writer - Acknowledge failure
pub type AF_W<'a, REG> = crate::BitWriter0C<'a, REG, AFW>;
impl<'a, REG> AF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AFW::Clear)
    }
}
/**Overrun/Underrun

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRR {
    ///0: No overrun/underrun occured
    NoOverrun = 0,
    ///1: Overrun/underrun occured
    Overrun = 1,
}
impl From<OVRR> for bool {
    #[inline(always)]
    fn from(variant: OVRR) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` reader - Overrun/Underrun
pub type OVR_R = crate::BitReader<OVRR>;
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRR {
        match self.bits {
            false => OVRR::NoOverrun,
            true => OVRR::Overrun,
        }
    }
    ///No overrun/underrun occured
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR::NoOverrun
    }
    ///Overrun/underrun occured
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR::Overrun
    }
}
/**Overrun/Underrun

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRW {
    ///0: Clear flag
    Clear = 0,
}
impl From<OVRW> for bool {
    #[inline(always)]
    fn from(variant: OVRW) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` writer - Overrun/Underrun
pub type OVR_W<'a, REG> = crate::BitWriter0C<'a, REG, OVRW>;
impl<'a, REG> OVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVRW::Clear)
    }
}
/**PEC Error in reception

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECERRR {
    ///0: no PEC error: receiver returns ACK after PEC reception (if ACK=1)
    NoError = 0,
    ///1: PEC error: receiver returns NACK after PEC reception (whatever ACK)
    Error = 1,
}
impl From<PECERRR> for bool {
    #[inline(always)]
    fn from(variant: PECERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `PECERR` reader - PEC Error in reception
pub type PECERR_R = crate::BitReader<PECERRR>;
impl PECERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PECERRR {
        match self.bits {
            false => PECERRR::NoError,
            true => PECERRR::Error,
        }
    }
    ///no PEC error: receiver returns ACK after PEC reception (if ACK=1)
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PECERRR::NoError
    }
    ///PEC error: receiver returns NACK after PEC reception (whatever ACK)
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PECERRR::Error
    }
}
/**PEC Error in reception

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECERRW {
    ///0: Clear flag
    Clear = 0,
}
impl From<PECERRW> for bool {
    #[inline(always)]
    fn from(variant: PECERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `PECERR` writer - PEC Error in reception
pub type PECERR_W<'a, REG> = crate::BitWriter0C<'a, REG, PECERRW>;
impl<'a, REG> PECERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PECERRW::Clear)
    }
}
/**Timeout or Tlow error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUTR {
    ///0: No Timeout error
    NoTimeout = 0,
    ///1: SCL remained LOW for 25 ms
    Timeout = 1,
}
impl From<TIMEOUTR> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUTR) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMEOUT` reader - Timeout or Tlow error
pub type TIMEOUT_R = crate::BitReader<TIMEOUTR>;
impl TIMEOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMEOUTR {
        match self.bits {
            false => TIMEOUTR::NoTimeout,
            true => TIMEOUTR::Timeout,
        }
    }
    ///No Timeout error
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == TIMEOUTR::NoTimeout
    }
    ///SCL remained LOW for 25 ms
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TIMEOUTR::Timeout
    }
}
/**Timeout or Tlow error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUTW {
    ///0: Clear flag
    Clear = 0,
}
impl From<TIMEOUTW> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUTW) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMEOUT` writer - Timeout or Tlow error
pub type TIMEOUT_W<'a, REG> = crate::BitWriter0C<'a, REG, TIMEOUTW>;
impl<'a, REG> TIMEOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUTW::Clear)
    }
}
/**SMBus alert

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBALERTR {
    ///0: No SMBALERT occured
    NoAlert = 0,
    ///1: SMBALERT occurred
    Alert = 1,
}
impl From<SMBALERTR> for bool {
    #[inline(always)]
    fn from(variant: SMBALERTR) -> Self {
        variant as u8 != 0
    }
}
///Field `SMBALERT` reader - SMBus alert
pub type SMBALERT_R = crate::BitReader<SMBALERTR>;
impl SMBALERT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMBALERTR {
        match self.bits {
            false => SMBALERTR::NoAlert,
            true => SMBALERTR::Alert,
        }
    }
    ///No SMBALERT occured
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == SMBALERTR::NoAlert
    }
    ///SMBALERT occurred
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == SMBALERTR::Alert
    }
}
/**SMBus alert

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBALERTW {
    ///0: Clear flag
    Clear = 0,
}
impl From<SMBALERTW> for bool {
    #[inline(always)]
    fn from(variant: SMBALERTW) -> Self {
        variant as u8 != 0
    }
}
///Field `SMBALERT` writer - SMBus alert
pub type SMBALERT_W<'a, REG> = crate::BitWriter0C<'a, REG, SMBALERTW>;
impl<'a, REG> SMBALERT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SMBALERTW::Clear)
    }
}
impl R {
    ///Bit 0 - Start bit (Master mode)
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Address sent (master mode)/matched (slave mode)
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Byte transfer finished
    #[inline(always)]
    pub fn btf(&self) -> BTF_R {
        BTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 10-bit header sent (Master mode)
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Stop detection (slave mode)
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Data register not empty (receivers)
    #[inline(always)]
    pub fn rx_ne(&self) -> RX_NE_R {
        RX_NE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Data register empty (transmitters)
    #[inline(always)]
    pub fn tx_e(&self) -> TX_E_R {
        TX_E_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Bus error
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Arbitration lost (master mode)
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Acknowledge failure
    #[inline(always)]
    pub fn af(&self) -> AF_R {
        AF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Overrun/Underrun
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PEC Error in reception
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Timeout or Tlow error
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SMBus alert
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR1")
            .field("smbalert", &self.smbalert())
            .field("timeout", &self.timeout())
            .field("pecerr", &self.pecerr())
            .field("ovr", &self.ovr())
            .field("af", &self.af())
            .field("arlo", &self.arlo())
            .field("berr", &self.berr())
            .field("tx_e", &self.tx_e())
            .field("rx_ne", &self.rx_ne())
            .field("stopf", &self.stopf())
            .field("add10", &self.add10())
            .field("btf", &self.btf())
            .field("addr", &self.addr())
            .field("sb", &self.sb())
            .finish()
    }
}
impl W {
    ///Bit 8 - Bus error
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W<'_, SR1rs> {
        BERR_W::new(self, 8)
    }
    ///Bit 9 - Arbitration lost (master mode)
    #[inline(always)]
    pub fn arlo(&mut self) -> ARLO_W<'_, SR1rs> {
        ARLO_W::new(self, 9)
    }
    ///Bit 10 - Acknowledge failure
    #[inline(always)]
    pub fn af(&mut self) -> AF_W<'_, SR1rs> {
        AF_W::new(self, 10)
    }
    ///Bit 11 - Overrun/Underrun
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<'_, SR1rs> {
        OVR_W::new(self, 11)
    }
    ///Bit 12 - PEC Error in reception
    #[inline(always)]
    pub fn pecerr(&mut self) -> PECERR_W<'_, SR1rs> {
        PECERR_W::new(self, 12)
    }
    ///Bit 14 - Timeout or Tlow error
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<'_, SR1rs> {
        TIMEOUT_W::new(self, 14)
    }
    ///Bit 15 - SMBus alert
    #[inline(always)]
    pub fn smbalert(&mut self) -> SMBALERT_W<'_, SR1rs> {
        SMBALERT_W::new(self, 15)
    }
}
/**Status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F429.html#I2C1:SR1)*/
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u16;
}
///`read()` method returns [`sr1::R`](R) reader structure
impl crate::Readable for SR1rs {}
///`write(|w| ..)` method takes [`sr1::W`](W) writer structure
impl crate::Writable for SR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0xdf00;
}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1rs {}
