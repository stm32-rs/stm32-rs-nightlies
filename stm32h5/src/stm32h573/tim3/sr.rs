#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIFR {
    #[doc = "0: No update occurred"]
    NoUpdateOccurred = 0,
    #[doc = "1: Update interrupt pending"]
    UpdatePending = 1,
}
impl From<UIFR> for bool {
    #[inline(always)]
    fn from(variant: UIFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
pub type UIF_R = crate::BitReader<UIFR>;
impl UIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UIFR {
        match self.bits {
            false => UIFR::NoUpdateOccurred,
            true => UIFR::UpdatePending,
        }
    }
    #[doc = "No update occurred"]
    #[inline(always)]
    pub fn is_no_update_occurred(&self) -> bool {
        *self == UIFR::NoUpdateOccurred
    }
    #[doc = "Update interrupt pending"]
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        *self == UIFR::UpdatePending
    }
}
#[doc = "Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<UIFW> for bool {
    #[inline(always)]
    fn from(variant: UIFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
pub type UIF_W<'a, REG> = crate::BitWriter0C<'a, REG, UIFW>;
impl<'a, REG> UIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(UIFW::Clear)
    }
}
#[doc = "Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IFR {
    #[doc = "1: If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register."]
    Match = 1,
}
impl From<CC1IFR> for bool {
    #[inline(always)]
    fn from(variant: CC1IFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IF` reader - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
pub type CC1IF_R = crate::BitReader<CC1IFR>;
impl CC1IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC1IFR> {
        match self.bits {
            true => Some(CC1IFR::Match),
            _ => None,
        }
    }
    #[doc = "If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register."]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CC1IFR::Match
    }
}
#[doc = "Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<CC1IFW> for bool {
    #[inline(always)]
    fn from(variant: CC1IFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IF` writer - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
pub type CC1IF_W<'a, REG> = crate::BitWriter0C<'a, REG, CC1IFW>;
impl<'a, REG> CC1IF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IFW::Clear)
    }
}
#[doc = "Field `CC2IF` reader - Capture/Compare 2 interrupt flag Refer to CC1IF description"]
pub use CC1IF_R as CC2IF_R;
#[doc = "Field `CC3IF` reader - Capture/Compare 3 interrupt flag Refer to CC1IF description"]
pub use CC1IF_R as CC3IF_R;
#[doc = "Field `CC4IF` reader - Capture/Compare 4 interrupt flag Refer to CC1IF description"]
pub use CC1IF_R as CC4IF_R;
#[doc = "Field `CC2IF` writer - Capture/Compare 2 interrupt flag Refer to CC1IF description"]
pub use CC1IF_W as CC2IF_W;
#[doc = "Field `CC3IF` writer - Capture/Compare 3 interrupt flag Refer to CC1IF description"]
pub use CC1IF_W as CC3IF_W;
#[doc = "Field `CC4IF` writer - Capture/Compare 4 interrupt flag Refer to CC1IF description"]
pub use CC1IF_W as CC4IF_W;
#[doc = "Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFR {
    #[doc = "0: No trigger event occurred"]
    NoTrigger = 0,
    #[doc = "1: Trigger interrupt pending"]
    Trigger = 1,
}
impl From<TIFR> for bool {
    #[inline(always)]
    fn from(variant: TIFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF` reader - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
pub type TIF_R = crate::BitReader<TIFR>;
impl TIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIFR {
        match self.bits {
            false => TIFR::NoTrigger,
            true => TIFR::Trigger,
        }
    }
    #[doc = "No trigger event occurred"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == TIFR::NoTrigger
    }
    #[doc = "Trigger interrupt pending"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TIFR::Trigger
    }
}
#[doc = "Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<TIFW> for bool {
    #[inline(always)]
    fn from(variant: TIFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF` writer - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
pub type TIF_W<'a, REG> = crate::BitWriter0C<'a, REG, TIFW>;
impl<'a, REG> TIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TIFW::Clear)
    }
}
#[doc = "Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OFR {
    #[doc = "1: The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set"]
    Overcapture = 1,
}
impl From<CC1OFR> for bool {
    #[inline(always)]
    fn from(variant: CC1OFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1OF` reader - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
pub type CC1OF_R = crate::BitReader<CC1OFR>;
impl CC1OF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC1OFR> {
        match self.bits {
            true => Some(CC1OFR::Overcapture),
            _ => None,
        }
    }
    #[doc = "The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set"]
    #[inline(always)]
    pub fn is_overcapture(&self) -> bool {
        *self == CC1OFR::Overcapture
    }
}
#[doc = "Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<CC1OFW> for bool {
    #[inline(always)]
    fn from(variant: CC1OFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1OF` writer - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
pub type CC1OF_W<'a, REG> = crate::BitWriter0C<'a, REG, CC1OFW>;
impl<'a, REG> CC1OF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CC1OFW::Clear)
    }
}
#[doc = "Field `CC2OF` reader - Capture/compare 2 overcapture flag refer to CC1OF description"]
pub use CC1OF_R as CC2OF_R;
#[doc = "Field `CC3OF` reader - Capture/Compare 3 overcapture flag refer to CC1OF description"]
pub use CC1OF_R as CC3OF_R;
#[doc = "Field `CC4OF` reader - Capture/Compare 4 overcapture flag refer to CC1OF description"]
pub use CC1OF_R as CC4OF_R;
#[doc = "Field `CC2OF` writer - Capture/compare 2 overcapture flag refer to CC1OF description"]
pub use CC1OF_W as CC2OF_W;
#[doc = "Field `CC3OF` writer - Capture/Compare 3 overcapture flag refer to CC1OF description"]
pub use CC1OF_W as CC3OF_W;
#[doc = "Field `CC4OF` writer - Capture/Compare 4 overcapture flag refer to CC1OF description"]
pub use CC1OF_W as CC4OF_W;
#[doc = "Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to ‘0’.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDXFR {
    #[doc = "0: No index event occurred"]
    NoTrigger = 0,
    #[doc = "1: An index event has occurred"]
    Trigger = 1,
}
impl From<IDXFR> for bool {
    #[inline(always)]
    fn from(variant: IDXFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDXF` reader - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to ‘0’."]
pub type IDXF_R = crate::BitReader<IDXFR>;
impl IDXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDXFR {
        match self.bits {
            false => IDXFR::NoTrigger,
            true => IDXFR::Trigger,
        }
    }
    #[doc = "No index event occurred"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == IDXFR::NoTrigger
    }
    #[doc = "An index event has occurred"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == IDXFR::Trigger
    }
}
#[doc = "Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to ‘0’.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDXFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<IDXFW> for bool {
    #[inline(always)]
    fn from(variant: IDXFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDXF` writer - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to ‘0’."]
pub type IDXF_W<'a, REG> = crate::BitWriter0C<'a, REG, IDXFW>;
impl<'a, REG> IDXF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IDXFW::Clear)
    }
}
#[doc = "Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to ‘0’.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRFR {
    #[doc = "0: No direction change has been detected"]
    NoTrigger = 0,
    #[doc = "1: A direction change has been detected"]
    Trigger = 1,
}
impl From<DIRFR> for bool {
    #[inline(always)]
    fn from(variant: DIRFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRF` reader - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to ‘0’."]
pub type DIRF_R = crate::BitReader<DIRFR>;
impl DIRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIRFR {
        match self.bits {
            false => DIRFR::NoTrigger,
            true => DIRFR::Trigger,
        }
    }
    #[doc = "No direction change has been detected"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == DIRFR::NoTrigger
    }
    #[doc = "A direction change has been detected"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == DIRFR::Trigger
    }
}
#[doc = "Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to ‘0’.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<DIRFW> for bool {
    #[inline(always)]
    fn from(variant: DIRFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRF` writer - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to ‘0’."]
pub type DIRF_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRFW>;
impl<'a, REG> DIRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DIRFW::Clear)
    }
}
#[doc = "Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to ‘0’.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IERRFR {
    #[doc = "0: No index error has been detected"]
    NoTrigger = 0,
    #[doc = "1: An index erorr has been detected"]
    Trigger = 1,
}
impl From<IERRFR> for bool {
    #[inline(always)]
    fn from(variant: IERRFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IERRF` reader - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to ‘0’."]
pub type IERRF_R = crate::BitReader<IERRFR>;
impl IERRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IERRFR {
        match self.bits {
            false => IERRFR::NoTrigger,
            true => IERRFR::Trigger,
        }
    }
    #[doc = "No index error has been detected"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == IERRFR::NoTrigger
    }
    #[doc = "An index erorr has been detected"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == IERRFR::Trigger
    }
}
#[doc = "Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to ‘0’.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IERRFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<IERRFW> for bool {
    #[inline(always)]
    fn from(variant: IERRFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IERRF` writer - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to ‘0’."]
pub type IERRF_W<'a, REG> = crate::BitWriter0C<'a, REG, IERRFW>;
impl<'a, REG> IERRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IERRFW::Clear)
    }
}
#[doc = "Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to ‘0’.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TERRFR {
    #[doc = "0: No encoder transition error has been detected"]
    NoTrigger = 0,
    #[doc = "1: An encoder transition error has been detected"]
    Trigger = 1,
}
impl From<TERRFR> for bool {
    #[inline(always)]
    fn from(variant: TERRFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TERRF` reader - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to ‘0’."]
pub type TERRF_R = crate::BitReader<TERRFR>;
impl TERRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TERRFR {
        match self.bits {
            false => TERRFR::NoTrigger,
            true => TERRFR::Trigger,
        }
    }
    #[doc = "No encoder transition error has been detected"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == TERRFR::NoTrigger
    }
    #[doc = "An encoder transition error has been detected"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TERRFR::Trigger
    }
}
#[doc = "Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to ‘0’.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TERRFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<TERRFW> for bool {
    #[inline(always)]
    fn from(variant: TERRFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TERRF` writer - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to ‘0’."]
pub type TERRF_W<'a, REG> = crate::BitWriter0C<'a, REG, TERRFW>;
impl<'a, REG> TERRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TERRFW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 20 - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    pub fn idxf(&self) -> IDXF_R {
        IDXF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    pub fn dirf(&self) -> DIRF_R {
        DIRF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    pub fn ierrf(&self) -> IERRF_R {
        IERRF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    pub fn terrf(&self) -> TERRF_R {
        TERRF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<SRrs> {
        UIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<SRrs> {
        CC1IF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CC2IF_W<SRrs> {
        CC2IF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3if(&mut self) -> CC3IF_W<SRrs> {
        CC3IF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4if(&mut self) -> CC4IF_W<SRrs> {
        CC4IF_W::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<SRrs> {
        TIF_W::new(self, 6)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CC1OF_W<SRrs> {
        CC1OF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2of(&mut self) -> CC2OF_W<SRrs> {
        CC2OF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3of(&mut self) -> CC3OF_W<SRrs> {
        CC3OF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4of(&mut self) -> CC4OF_W<SRrs> {
        CC4OF_W::new(self, 12)
    }
    #[doc = "Bit 20 - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    #[must_use]
    pub fn idxf(&mut self) -> IDXF_W<SRrs> {
        IDXF_W::new(self, 20)
    }
    #[doc = "Bit 21 - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    #[must_use]
    pub fn dirf(&mut self) -> DIRF_W<SRrs> {
        DIRF_W::new(self, 21)
    }
    #[doc = "Bit 22 - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    #[must_use]
    pub fn ierrf(&mut self) -> IERRF_W<SRrs> {
        IERRF_W::new(self, 22)
    }
    #[doc = "Bit 23 - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    #[must_use]
    pub fn terrf(&mut self) -> TERRF_W<SRrs> {
        TERRF_W::new(self, 23)
    }
}
#[doc = "TIM3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x00f0_1e5f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
