///Register `ISTR` reader
pub type R = crate::R<ISTRrs>;
///Register `ISTR` writer
pub type W = crate::W<ISTRrs>;
///Field `EP_ID` reader - Endpoint Identifier
pub type EP_ID_R = crate::FieldReader;
/**Direction of transaction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR {
    ///0: Data transmitted by the USB peripheral to the host PC
    To = 0,
    ///1: Data received by the USB peripheral from the host PC
    From = 1,
}
impl From<DIR> for bool {
    #[inline(always)]
    fn from(variant: DIR) -> Self {
        variant as u8 != 0
    }
}
///Field `DIR` reader - Direction of transaction
pub type DIR_R = crate::BitReader<DIR>;
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIR {
        match self.bits {
            false => DIR::To,
            true => DIR::From,
        }
    }
    ///Data transmitted by the USB peripheral to the host PC
    #[inline(always)]
    pub fn is_to(&self) -> bool {
        *self == DIR::To
    }
    ///Data received by the USB peripheral from the host PC
    #[inline(always)]
    pub fn is_from(&self) -> bool {
        *self == DIR::From
    }
}
/**LPM L1 state request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQR {
    ///0: LPM command to enter the L1 state is not received
    NotReceived = 0,
    ///1: LPM command to enter the L1 state is successfully received and acknowledged
    Received = 1,
}
impl From<L1REQR> for bool {
    #[inline(always)]
    fn from(variant: L1REQR) -> Self {
        variant as u8 != 0
    }
}
///Field `L1REQ` reader - LPM L1 state request
pub type L1REQ_R = crate::BitReader<L1REQR>;
impl L1REQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1REQR {
        match self.bits {
            false => L1REQR::NotReceived,
            true => L1REQR::Received,
        }
    }
    ///LPM command to enter the L1 state is not received
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == L1REQR::NotReceived
    }
    ///LPM command to enter the L1 state is successfully received and acknowledged
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == L1REQR::Received
    }
}
/**LPM L1 state request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQW {
    ///0: Clear flag
    Clear = 0,
}
impl From<L1REQW> for bool {
    #[inline(always)]
    fn from(variant: L1REQW) -> Self {
        variant as u8 != 0
    }
}
///Field `L1REQ` writer - LPM L1 state request
pub type L1REQ_W<'a, REG> = crate::BitWriter0C<'a, REG, L1REQW>;
impl<'a, REG> L1REQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(L1REQW::Clear)
    }
}
/**Expected start frame

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESOFR {
    ///0: NotExpectedStartOfFrame
    NotExpectedStartOfFrame = 0,
    ///1: An SOF packet is expected but not received
    ExpectedStartOfFrame = 1,
}
impl From<ESOFR> for bool {
    #[inline(always)]
    fn from(variant: ESOFR) -> Self {
        variant as u8 != 0
    }
}
///Field `ESOF` reader - Expected start frame
pub type ESOF_R = crate::BitReader<ESOFR>;
impl ESOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ESOFR {
        match self.bits {
            false => ESOFR::NotExpectedStartOfFrame,
            true => ESOFR::ExpectedStartOfFrame,
        }
    }
    ///NotExpectedStartOfFrame
    #[inline(always)]
    pub fn is_not_expected_start_of_frame(&self) -> bool {
        *self == ESOFR::NotExpectedStartOfFrame
    }
    ///An SOF packet is expected but not received
    #[inline(always)]
    pub fn is_expected_start_of_frame(&self) -> bool {
        *self == ESOFR::ExpectedStartOfFrame
    }
}
/**Expected start frame

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESOFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<ESOFW> for bool {
    #[inline(always)]
    fn from(variant: ESOFW) -> Self {
        variant as u8 != 0
    }
}
///Field `ESOF` writer - Expected start frame
pub type ESOF_W<'a, REG> = crate::BitWriter0C<'a, REG, ESOFW>;
impl<'a, REG> ESOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ESOFW::Clear)
    }
}
/**start of frame

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFR {
    ///0: NotStartOfFrame
    NotStartOfFrame = 0,
    ///1: Beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus
    StartOfFrame = 1,
}
impl From<SOFR> for bool {
    #[inline(always)]
    fn from(variant: SOFR) -> Self {
        variant as u8 != 0
    }
}
///Field `SOF` reader - start of frame
pub type SOF_R = crate::BitReader<SOFR>;
impl SOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOFR {
        match self.bits {
            false => SOFR::NotStartOfFrame,
            true => SOFR::StartOfFrame,
        }
    }
    ///NotStartOfFrame
    #[inline(always)]
    pub fn is_not_start_of_frame(&self) -> bool {
        *self == SOFR::NotStartOfFrame
    }
    ///Beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus
    #[inline(always)]
    pub fn is_start_of_frame(&self) -> bool {
        *self == SOFR::StartOfFrame
    }
}
/**start of frame

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<SOFW> for bool {
    #[inline(always)]
    fn from(variant: SOFW) -> Self {
        variant as u8 != 0
    }
}
///Field `SOF` writer - start of frame
pub type SOF_W<'a, REG> = crate::BitWriter0C<'a, REG, SOFW>;
impl<'a, REG> SOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SOFW::Clear)
    }
}
/**reset request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETR {
    ///0: NotReset
    NotReset = 0,
    ///1: Peripheral detects an active USB RESET signal at its inputs
    Reset = 1,
}
impl From<RESETR> for bool {
    #[inline(always)]
    fn from(variant: RESETR) -> Self {
        variant as u8 != 0
    }
}
///Field `RESET` reader - reset request
pub type RESET_R = crate::BitReader<RESETR>;
impl RESET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RESETR {
        match self.bits {
            false => RESETR::NotReset,
            true => RESETR::Reset,
        }
    }
    ///NotReset
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == RESETR::NotReset
    }
    ///Peripheral detects an active USB RESET signal at its inputs
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESETR::Reset
    }
}
/**reset request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETW {
    ///0: Clear flag
    Clear = 0,
}
impl From<RESETW> for bool {
    #[inline(always)]
    fn from(variant: RESETW) -> Self {
        variant as u8 != 0
    }
}
///Field `RESET` writer - reset request
pub type RESET_W<'a, REG> = crate::BitWriter0C<'a, REG, RESETW>;
impl<'a, REG> RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RESETW::Clear)
    }
}
/**Suspend mode request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPR {
    ///0: NotSuspend
    NotSuspend = 0,
    ///1: No traffic has been received for 3 ms, indicating a suspend mode request from the USB bus
    Suspend = 1,
}
impl From<SUSPR> for bool {
    #[inline(always)]
    fn from(variant: SUSPR) -> Self {
        variant as u8 != 0
    }
}
///Field `SUSP` reader - Suspend mode request
pub type SUSP_R = crate::BitReader<SUSPR>;
impl SUSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SUSPR {
        match self.bits {
            false => SUSPR::NotSuspend,
            true => SUSPR::Suspend,
        }
    }
    ///NotSuspend
    #[inline(always)]
    pub fn is_not_suspend(&self) -> bool {
        *self == SUSPR::NotSuspend
    }
    ///No traffic has been received for 3 ms, indicating a suspend mode request from the USB bus
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == SUSPR::Suspend
    }
}
/**Suspend mode request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPW {
    ///0: Clear flag
    Clear = 0,
}
impl From<SUSPW> for bool {
    #[inline(always)]
    fn from(variant: SUSPW) -> Self {
        variant as u8 != 0
    }
}
///Field `SUSP` writer - Suspend mode request
pub type SUSP_W<'a, REG> = crate::BitWriter0C<'a, REG, SUSPW>;
impl<'a, REG> SUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPW::Clear)
    }
}
/**Wakeup

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPR {
    ///0: NotWakeup
    NotWakeup = 0,
    ///1: Activity is detected that wakes up the USB peripheral
    Wakeup = 1,
}
impl From<WKUPR> for bool {
    #[inline(always)]
    fn from(variant: WKUPR) -> Self {
        variant as u8 != 0
    }
}
///Field `WKUP` reader - Wakeup
pub type WKUP_R = crate::BitReader<WKUPR>;
impl WKUP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WKUPR {
        match self.bits {
            false => WKUPR::NotWakeup,
            true => WKUPR::Wakeup,
        }
    }
    ///NotWakeup
    #[inline(always)]
    pub fn is_not_wakeup(&self) -> bool {
        *self == WKUPR::NotWakeup
    }
    ///Activity is detected that wakes up the USB peripheral
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WKUPR::Wakeup
    }
}
/**Wakeup

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPW {
    ///0: Clear flag
    Clear = 0,
}
impl From<WKUPW> for bool {
    #[inline(always)]
    fn from(variant: WKUPW) -> Self {
        variant as u8 != 0
    }
}
///Field `WKUP` writer - Wakeup
pub type WKUP_W<'a, REG> = crate::BitWriter0C<'a, REG, WKUPW>;
impl<'a, REG> WKUP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPW::Clear)
    }
}
/**Error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRR {
    ///0: Errors are not occurred
    NotOverrun = 0,
    ///1: One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred
    Error = 1,
}
impl From<ERRR> for bool {
    #[inline(always)]
    fn from(variant: ERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `ERR` reader - Error
pub type ERR_R = crate::BitReader<ERRR>;
impl ERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERRR {
        match self.bits {
            false => ERRR::NotOverrun,
            true => ERRR::Error,
        }
    }
    ///Errors are not occurred
    #[inline(always)]
    pub fn is_not_overrun(&self) -> bool {
        *self == ERRR::NotOverrun
    }
    ///One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERRR::Error
    }
}
/**Error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRW {
    ///0: Clear flag
    Clear = 0,
}
impl From<ERRW> for bool {
    #[inline(always)]
    fn from(variant: ERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `ERR` writer - Error
pub type ERR_W<'a, REG> = crate::BitWriter0C<'a, REG, ERRW>;
impl<'a, REG> ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERRW::Clear)
    }
}
/**Packet memory area over / underrun

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMAOVRR {
    ///0: Overrun is not occurred
    NotOverrun = 0,
    ///1: Microcontroller has not been able to respond in time to an USB memory request
    Overrun = 1,
}
impl From<PMAOVRR> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRR) -> Self {
        variant as u8 != 0
    }
}
///Field `PMAOVR` reader - Packet memory area over / underrun
pub type PMAOVR_R = crate::BitReader<PMAOVRR>;
impl PMAOVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PMAOVRR {
        match self.bits {
            false => PMAOVRR::NotOverrun,
            true => PMAOVRR::Overrun,
        }
    }
    ///Overrun is not occurred
    #[inline(always)]
    pub fn is_not_overrun(&self) -> bool {
        *self == PMAOVRR::NotOverrun
    }
    ///Microcontroller has not been able to respond in time to an USB memory request
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == PMAOVRR::Overrun
    }
}
/**Packet memory area over / underrun

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMAOVRW {
    ///0: Clear flag
    Clear = 0,
}
impl From<PMAOVRW> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRW) -> Self {
        variant as u8 != 0
    }
}
///Field `PMAOVR` writer - Packet memory area over / underrun
pub type PMAOVR_W<'a, REG> = crate::BitWriter0C<'a, REG, PMAOVRW>;
impl<'a, REG> PMAOVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PMAOVRW::Clear)
    }
}
/**Correct transfer

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTR {
    ///1: Endpoint has successfully completed a transaction
    Completed = 1,
}
impl From<CTR> for bool {
    #[inline(always)]
    fn from(variant: CTR) -> Self {
        variant as u8 != 0
    }
}
///Field `CTR` reader - Correct transfer
pub type CTR_R = crate::BitReader<CTR>;
impl CTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTR> {
        match self.bits {
            true => Some(CTR::Completed),
            _ => None,
        }
    }
    ///Endpoint has successfully completed a transaction
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == CTR::Completed
    }
}
impl R {
    ///Bits 0:3 - Endpoint Identifier
    #[inline(always)]
    pub fn ep_id(&self) -> EP_ID_R {
        EP_ID_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Direction of transaction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - LPM L1 state request
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Expected start frame
    #[inline(always)]
    pub fn esof(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - start of frame
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - reset request
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Suspend mode request
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Wakeup
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Error
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Packet memory area over / underrun
    #[inline(always)]
    pub fn pmaovr(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Correct transfer
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISTR")
            .field("ep_id", &self.ep_id())
            .field("dir", &self.dir())
            .field("l1req", &self.l1req())
            .field("esof", &self.esof())
            .field("sof", &self.sof())
            .field("reset", &self.reset())
            .field("susp", &self.susp())
            .field("wkup", &self.wkup())
            .field("err", &self.err())
            .field("pmaovr", &self.pmaovr())
            .field("ctr", &self.ctr())
            .finish()
    }
}
impl W {
    ///Bit 7 - LPM L1 state request
    #[inline(always)]
    pub fn l1req(&mut self) -> L1REQ_W<'_, ISTRrs> {
        L1REQ_W::new(self, 7)
    }
    ///Bit 8 - Expected start frame
    #[inline(always)]
    pub fn esof(&mut self) -> ESOF_W<'_, ISTRrs> {
        ESOF_W::new(self, 8)
    }
    ///Bit 9 - start of frame
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<'_, ISTRrs> {
        SOF_W::new(self, 9)
    }
    ///Bit 10 - reset request
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<'_, ISTRrs> {
        RESET_W::new(self, 10)
    }
    ///Bit 11 - Suspend mode request
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<'_, ISTRrs> {
        SUSP_W::new(self, 11)
    }
    ///Bit 12 - Wakeup
    #[inline(always)]
    pub fn wkup(&mut self) -> WKUP_W<'_, ISTRrs> {
        WKUP_W::new(self, 12)
    }
    ///Bit 13 - Error
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W<'_, ISTRrs> {
        ERR_W::new(self, 13)
    }
    ///Bit 14 - Packet memory area over / underrun
    #[inline(always)]
    pub fn pmaovr(&mut self) -> PMAOVR_W<'_, ISTRrs> {
        PMAOVR_W::new(self, 14)
    }
}
/**interrupt status register

You can [`read`](crate::Reg::read) this register and get [`istr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#USB:ISTR)*/
pub struct ISTRrs;
impl crate::RegisterSpec for ISTRrs {
    type Ux = u16;
}
///`read()` method returns [`istr::R`](R) reader structure
impl crate::Readable for ISTRrs {}
///`write(|w| ..)` method takes [`istr::W`](W) writer structure
impl crate::Writable for ISTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x7f80;
}
///`reset()` method sets ISTR to value 0
impl crate::Resettable for ISTRrs {}
