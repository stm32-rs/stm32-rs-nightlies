#[doc = "Register `ISTR` reader"]
pub type R = crate::R<ISTRrs>;
#[doc = "Register `ISTR` writer"]
pub type W = crate::W<ISTRrs>;
#[doc = "Field `EP_ID` reader - EP_ID"]
pub type EP_ID_R = crate::FieldReader;
#[doc = "Field `EP_ID` writer - EP_ID"]
pub type EP_ID_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "DIR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR {
    #[doc = "0: data transmitted by the USB peripheral to the host PC"]
    To = 0,
    #[doc = "1: data received by the USB peripheral from the host PC"]
    From = 1,
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
            false => DIR::To,
            true => DIR::From,
        }
    }
    #[doc = "data transmitted by the USB peripheral to the host PC"]
    #[inline(always)]
    pub fn is_to(&self) -> bool {
        *self == DIR::To
    }
    #[doc = "data received by the USB peripheral from the host PC"]
    #[inline(always)]
    pub fn is_from(&self) -> bool {
        *self == DIR::From
    }
}
#[doc = "Field `DIR` writer - DIR"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG, DIR>;
impl<'a, REG> DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "data transmitted by the USB peripheral to the host PC"]
    #[inline(always)]
    pub fn to(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::To)
    }
    #[doc = "data received by the USB peripheral from the host PC"]
    #[inline(always)]
    pub fn from(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::From)
    }
}
#[doc = "L1REQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQR {
    #[doc = "0: LPM command to enter the L1 state is not received"]
    NotReceived = 0,
    #[doc = "1: LPM command to enter the L1 state is successfully received and acknowledged"]
    Received = 1,
}
impl From<L1REQR> for bool {
    #[inline(always)]
    fn from(variant: L1REQR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1REQ` reader - L1REQ"]
pub type L1REQ_R = crate::BitReader<L1REQR>;
impl L1REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L1REQR {
        match self.bits {
            false => L1REQR::NotReceived,
            true => L1REQR::Received,
        }
    }
    #[doc = "LPM command to enter the L1 state is not received"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == L1REQR::NotReceived
    }
    #[doc = "LPM command to enter the L1 state is successfully received and acknowledged"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == L1REQR::Received
    }
}
#[doc = "L1REQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<L1REQW> for bool {
    #[inline(always)]
    fn from(variant: L1REQW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1REQ` writer - L1REQ"]
pub type L1REQ_W<'a, REG> = crate::BitWriter0C<'a, REG, L1REQW>;
impl<'a, REG> L1REQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(L1REQW::Clear)
    }
}
#[doc = "ESOF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESOFR {
    #[doc = "0: NotExpectedStartOfFrame"]
    NotExpectedStartOfFrame = 0,
    #[doc = "1: an SOF packet is expected but not received"]
    ExpectedStartOfFrame = 1,
}
impl From<ESOFR> for bool {
    #[inline(always)]
    fn from(variant: ESOFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESOF` reader - ESOF"]
pub type ESOF_R = crate::BitReader<ESOFR>;
impl ESOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ESOFR {
        match self.bits {
            false => ESOFR::NotExpectedStartOfFrame,
            true => ESOFR::ExpectedStartOfFrame,
        }
    }
    #[doc = "NotExpectedStartOfFrame"]
    #[inline(always)]
    pub fn is_not_expected_start_of_frame(&self) -> bool {
        *self == ESOFR::NotExpectedStartOfFrame
    }
    #[doc = "an SOF packet is expected but not received"]
    #[inline(always)]
    pub fn is_expected_start_of_frame(&self) -> bool {
        *self == ESOFR::ExpectedStartOfFrame
    }
}
#[doc = "ESOF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESOFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<ESOFW> for bool {
    #[inline(always)]
    fn from(variant: ESOFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESOF` writer - ESOF"]
pub type ESOF_W<'a, REG> = crate::BitWriter0C<'a, REG, ESOFW>;
impl<'a, REG> ESOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ESOFW::Clear)
    }
}
#[doc = "SOF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFR {
    #[doc = "0: NotStartOfFrame"]
    NotStartOfFrame = 0,
    #[doc = "1: beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus"]
    StartOfFrame = 1,
}
impl From<SOFR> for bool {
    #[inline(always)]
    fn from(variant: SOFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - SOF"]
pub type SOF_R = crate::BitReader<SOFR>;
impl SOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOFR {
        match self.bits {
            false => SOFR::NotStartOfFrame,
            true => SOFR::StartOfFrame,
        }
    }
    #[doc = "NotStartOfFrame"]
    #[inline(always)]
    pub fn is_not_start_of_frame(&self) -> bool {
        *self == SOFR::NotStartOfFrame
    }
    #[doc = "beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus"]
    #[inline(always)]
    pub fn is_start_of_frame(&self) -> bool {
        *self == SOFR::StartOfFrame
    }
}
#[doc = "SOF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<SOFW> for bool {
    #[inline(always)]
    fn from(variant: SOFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` writer - SOF"]
pub type SOF_W<'a, REG> = crate::BitWriter0C<'a, REG, SOFW>;
impl<'a, REG> SOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SOFW::Clear)
    }
}
#[doc = "RESET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETR {
    #[doc = "0: NotReset"]
    NotReset = 0,
    #[doc = "1: peripheral detects an active USB RESET signal at its inputs"]
    Reset = 1,
}
impl From<RESETR> for bool {
    #[inline(always)]
    fn from(variant: RESETR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` reader - RESET"]
pub type RESET_R = crate::BitReader<RESETR>;
impl RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESETR {
        match self.bits {
            false => RESETR::NotReset,
            true => RESETR::Reset,
        }
    }
    #[doc = "NotReset"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == RESETR::NotReset
    }
    #[doc = "peripheral detects an active USB RESET signal at its inputs"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESETR::Reset
    }
}
#[doc = "RESET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<RESETW> for bool {
    #[inline(always)]
    fn from(variant: RESETW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - RESET"]
pub type RESET_W<'a, REG> = crate::BitWriter0C<'a, REG, RESETW>;
impl<'a, REG> RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RESETW::Clear)
    }
}
#[doc = "SUSP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPR {
    #[doc = "0: NotSuspend"]
    NotSuspend = 0,
    #[doc = "1: no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus"]
    Suspend = 1,
}
impl From<SUSPR> for bool {
    #[inline(always)]
    fn from(variant: SUSPR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSP` reader - SUSP"]
pub type SUSP_R = crate::BitReader<SUSPR>;
impl SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSPR {
        match self.bits {
            false => SUSPR::NotSuspend,
            true => SUSPR::Suspend,
        }
    }
    #[doc = "NotSuspend"]
    #[inline(always)]
    pub fn is_not_suspend(&self) -> bool {
        *self == SUSPR::NotSuspend
    }
    #[doc = "no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == SUSPR::Suspend
    }
}
#[doc = "SUSP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<SUSPW> for bool {
    #[inline(always)]
    fn from(variant: SUSPW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSP` writer - SUSP"]
pub type SUSP_W<'a, REG> = crate::BitWriter0C<'a, REG, SUSPW>;
impl<'a, REG> SUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPW::Clear)
    }
}
#[doc = "WKUP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPR {
    #[doc = "0: NotWakeup"]
    NotWakeup = 0,
    #[doc = "1: activity is detected that wakes up the USB peripheral"]
    Wakeup = 1,
}
impl From<WKUPR> for bool {
    #[inline(always)]
    fn from(variant: WKUPR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUP` reader - WKUP"]
pub type WKUP_R = crate::BitReader<WKUPR>;
impl WKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WKUPR {
        match self.bits {
            false => WKUPR::NotWakeup,
            true => WKUPR::Wakeup,
        }
    }
    #[doc = "NotWakeup"]
    #[inline(always)]
    pub fn is_not_wakeup(&self) -> bool {
        *self == WKUPR::NotWakeup
    }
    #[doc = "activity is detected that wakes up the USB peripheral"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WKUPR::Wakeup
    }
}
#[doc = "WKUP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<WKUPW> for bool {
    #[inline(always)]
    fn from(variant: WKUPW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUP` writer - WKUP"]
pub type WKUP_W<'a, REG> = crate::BitWriter0C<'a, REG, WKUPW>;
impl<'a, REG> WKUP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPW::Clear)
    }
}
#[doc = "ERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRR {
    #[doc = "0: Errors are not occurred"]
    NotOverrun = 0,
    #[doc = "1: One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred"]
    Error = 1,
}
impl From<ERRR> for bool {
    #[inline(always)]
    fn from(variant: ERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR` reader - ERR"]
pub type ERR_R = crate::BitReader<ERRR>;
impl ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRR {
        match self.bits {
            false => ERRR::NotOverrun,
            true => ERRR::Error,
        }
    }
    #[doc = "Errors are not occurred"]
    #[inline(always)]
    pub fn is_not_overrun(&self) -> bool {
        *self == ERRR::NotOverrun
    }
    #[doc = "One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERRR::Error
    }
}
#[doc = "ERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<ERRW> for bool {
    #[inline(always)]
    fn from(variant: ERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR` writer - ERR"]
pub type ERR_W<'a, REG> = crate::BitWriter0C<'a, REG, ERRW>;
impl<'a, REG> ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERRW::Clear)
    }
}
#[doc = "PMAOVR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMAOVRR {
    #[doc = "0: Overrun is not occurred"]
    NotOverrun = 0,
    #[doc = "1: microcontroller has not been able to respond in time to an USB memory request"]
    Overrun = 1,
}
impl From<PMAOVRR> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMAOVR` reader - PMAOVR"]
pub type PMAOVR_R = crate::BitReader<PMAOVRR>;
impl PMAOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PMAOVRR {
        match self.bits {
            false => PMAOVRR::NotOverrun,
            true => PMAOVRR::Overrun,
        }
    }
    #[doc = "Overrun is not occurred"]
    #[inline(always)]
    pub fn is_not_overrun(&self) -> bool {
        *self == PMAOVRR::NotOverrun
    }
    #[doc = "microcontroller has not been able to respond in time to an USB memory request"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == PMAOVRR::Overrun
    }
}
#[doc = "PMAOVR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMAOVRW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<PMAOVRW> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMAOVR` writer - PMAOVR"]
pub type PMAOVR_W<'a, REG> = crate::BitWriter0C<'a, REG, PMAOVRW>;
impl<'a, REG> PMAOVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PMAOVRW::Clear)
    }
}
#[doc = "CTR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTR {
    #[doc = "1: endpoint has successfully completed a transaction"]
    Completed = 1,
}
impl From<CTR> for bool {
    #[inline(always)]
    fn from(variant: CTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTR` reader - CTR"]
pub type CTR_R = crate::BitReader<CTR>;
impl CTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTR> {
        match self.bits {
            true => Some(CTR::Completed),
            _ => None,
        }
    }
    #[doc = "endpoint has successfully completed a transaction"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == CTR::Completed
    }
}
#[doc = "Field `CTR` writer - CTR"]
pub type CTR_W<'a, REG> = crate::BitWriter<'a, REG, CTR>;
impl<'a, REG> CTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "endpoint has successfully completed a transaction"]
    #[inline(always)]
    pub fn completed(self) -> &'a mut crate::W<REG> {
        self.variant(CTR::Completed)
    }
}
impl R {
    #[doc = "Bits 0:3 - EP_ID"]
    #[inline(always)]
    pub fn ep_id(&self) -> EP_ID_R {
        EP_ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - L1REQ"]
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ESOF"]
    #[inline(always)]
    pub fn esof(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SUSP"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - WKUP"]
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ERR"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PMAOVR"]
    #[inline(always)]
    pub fn pmaovr(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTR"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - EP_ID"]
    #[inline(always)]
    #[must_use]
    pub fn ep_id(&mut self) -> EP_ID_W<ISTRrs> {
        EP_ID_W::new(self, 0)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<ISTRrs> {
        DIR_W::new(self, 4)
    }
    #[doc = "Bit 7 - L1REQ"]
    #[inline(always)]
    #[must_use]
    pub fn l1req(&mut self) -> L1REQ_W<ISTRrs> {
        L1REQ_W::new(self, 7)
    }
    #[doc = "Bit 8 - ESOF"]
    #[inline(always)]
    #[must_use]
    pub fn esof(&mut self) -> ESOF_W<ISTRrs> {
        ESOF_W::new(self, 8)
    }
    #[doc = "Bit 9 - SOF"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<ISTRrs> {
        SOF_W::new(self, 9)
    }
    #[doc = "Bit 10 - RESET"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<ISTRrs> {
        RESET_W::new(self, 10)
    }
    #[doc = "Bit 11 - SUSP"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<ISTRrs> {
        SUSP_W::new(self, 11)
    }
    #[doc = "Bit 12 - WKUP"]
    #[inline(always)]
    #[must_use]
    pub fn wkup(&mut self) -> WKUP_W<ISTRrs> {
        WKUP_W::new(self, 12)
    }
    #[doc = "Bit 13 - ERR"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<ISTRrs> {
        ERR_W::new(self, 13)
    }
    #[doc = "Bit 14 - PMAOVR"]
    #[inline(always)]
    #[must_use]
    pub fn pmaovr(&mut self) -> PMAOVR_W<ISTRrs> {
        PMAOVR_W::new(self, 14)
    }
    #[doc = "Bit 15 - CTR"]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CTR_W<ISTRrs> {
        CTR_W::new(self, 15)
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`istr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISTRrs;
impl crate::RegisterSpec for ISTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`istr::R`](R) reader structure"]
impl crate::Readable for ISTRrs {}
#[doc = "`write(|w| ..)` method takes [`istr::W`](W) writer structure"]
impl crate::Writable for ISTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7f80;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISTR to value 0"]
impl crate::Resettable for ISTRrs {
    const RESET_VALUE: u32 = 0;
}
