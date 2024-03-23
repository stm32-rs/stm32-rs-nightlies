#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to Section 65.6.3: TIM1 slave mode control register (TIM1_SMCR)), if URS=0 and UDIS=0 in the TIMx_CR1 register.\n\nValue on reset: 0"]
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
#[doc = "Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to Section 65.6.3: TIM1 slave mode control register (TIM1_SMCR)), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
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
#[doc = "Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to Section 65.6.3: TIM1 slave mode control register (TIM1_SMCR)), if URS=0 and UDIS=0 in the TIMx_CR1 register.\n\nValue on reset: 0"]
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
#[doc = "Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to Section 65.6.3: TIM1 slave mode control register (TIM1_SMCR)), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
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
#[doc = "Field `CC2IF` reader - Capture/compare 2 interrupt flag Refer to CC1IF description"]
pub use CC1IF_R as CC2IF_R;
#[doc = "Field `CC3IF` reader - Capture/compare 3 interrupt flag Refer to CC1IF description"]
pub use CC1IF_R as CC3IF_R;
#[doc = "Field `CC4IF` reader - Capture/compare 4 interrupt flag Refer to CC1IF description"]
pub use CC1IF_R as CC4IF_R;
#[doc = "Field `CC2IF` writer - Capture/compare 2 interrupt flag Refer to CC1IF description"]
pub use CC1IF_W as CC2IF_W;
#[doc = "Field `CC3IF` writer - Capture/compare 3 interrupt flag Refer to CC1IF description"]
pub use CC1IF_W as CC3IF_W;
#[doc = "Field `CC4IF` writer - Capture/compare 4 interrupt flag Refer to CC1IF description"]
pub use CC1IF_W as CC4IF_W;
#[doc = "COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMIFR {
    #[doc = "0: No COM event occurred"]
    NoCom = 0,
    #[doc = "1: COM interrupt pending"]
    Com = 1,
}
impl From<COMIFR> for bool {
    #[inline(always)]
    fn from(variant: COMIFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMIF` reader - COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software."]
pub type COMIF_R = crate::BitReader<COMIFR>;
impl COMIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMIFR {
        match self.bits {
            false => COMIFR::NoCom,
            true => COMIFR::Com,
        }
    }
    #[doc = "No COM event occurred"]
    #[inline(always)]
    pub fn is_no_com(&self) -> bool {
        *self == COMIFR::NoCom
    }
    #[doc = "COM interrupt pending"]
    #[inline(always)]
    pub fn is_com(&self) -> bool {
        *self == COMIFR::Com
    }
}
#[doc = "COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMIFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<COMIFW> for bool {
    #[inline(always)]
    fn from(variant: COMIFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMIF` writer - COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software."]
pub type COMIF_W<'a, REG> = crate::BitWriter0C<'a, REG, COMIFW>;
impl<'a, REG> COMIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(COMIFW::Clear)
    }
}
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
#[doc = "Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIFR {
    #[doc = "0: No break event occurred"]
    NoTrigger = 0,
    #[doc = "1: An active level has been detected on the break input. An interrupt is generated if BIE=1 in the TIMx_DIER register"]
    Trigger = 1,
}
impl From<BIFR> for bool {
    #[inline(always)]
    fn from(variant: BIFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIF` reader - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active."]
pub type BIF_R = crate::BitReader<BIFR>;
impl BIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BIFR {
        match self.bits {
            false => BIFR::NoTrigger,
            true => BIFR::Trigger,
        }
    }
    #[doc = "No break event occurred"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == BIFR::NoTrigger
    }
    #[doc = "An active level has been detected on the break input. An interrupt is generated if BIE=1 in the TIMx_DIER register"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == BIFR::Trigger
    }
}
#[doc = "Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<BIFW> for bool {
    #[inline(always)]
    fn from(variant: BIFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIF` writer - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active."]
pub type BIF_W<'a, REG> = crate::BitWriter0C<'a, REG, BIFW>;
impl<'a, REG> BIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BIFW::Clear)
    }
}
#[doc = "Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2IFR {
    #[doc = "0: No break event occurred"]
    NoTrigger = 0,
    #[doc = "1: An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the TIMx_DIER register"]
    Trigger = 1,
}
impl From<B2IFR> for bool {
    #[inline(always)]
    fn from(variant: B2IFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B2IF` reader - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active."]
pub type B2IF_R = crate::BitReader<B2IFR>;
impl B2IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B2IFR {
        match self.bits {
            false => B2IFR::NoTrigger,
            true => B2IFR::Trigger,
        }
    }
    #[doc = "No break event occurred"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == B2IFR::NoTrigger
    }
    #[doc = "An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the TIMx_DIER register"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == B2IFR::Trigger
    }
}
#[doc = "Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2IFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<B2IFW> for bool {
    #[inline(always)]
    fn from(variant: B2IFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B2IF` writer - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active."]
pub type B2IF_W<'a, REG> = crate::BitWriter0C<'a, REG, B2IFW>;
impl<'a, REG> B2IF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(B2IFW::Clear)
    }
}
#[doc = "Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’.\n\nValue on reset: 0"]
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
#[doc = "Field `CC1OF` reader - Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
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
#[doc = "Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’.\n\nValue on reset: 0"]
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
#[doc = "Field `CC1OF` writer - Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
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
#[doc = "Field `CC2OF` reader - Capture/compare 2 overcapture flag Refer to CC1OF description"]
pub use CC1OF_R as CC2OF_R;
#[doc = "Field `CC3OF` reader - Capture/compare 3 overcapture flag Refer to CC1OF description"]
pub use CC1OF_R as CC3OF_R;
#[doc = "Field `CC4OF` reader - Capture/compare 4 overcapture flag Refer to CC1OF description"]
pub use CC1OF_R as CC4OF_R;
#[doc = "Field `CC2OF` writer - Capture/compare 2 overcapture flag Refer to CC1OF description"]
pub use CC1OF_W as CC2OF_W;
#[doc = "Field `CC3OF` writer - Capture/compare 3 overcapture flag Refer to CC1OF description"]
pub use CC1OF_W as CC3OF_W;
#[doc = "Field `CC4OF` writer - Capture/compare 4 overcapture flag Refer to CC1OF description"]
pub use CC1OF_W as CC4OF_W;
#[doc = "System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBIFR {
    #[doc = "0: No break event occurred"]
    NoTrigger = 0,
    #[doc = "1: An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the TIMx_DIER register"]
    Trigger = 1,
}
impl From<SBIFR> for bool {
    #[inline(always)]
    fn from(variant: SBIFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBIF` reader - System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation."]
pub type SBIF_R = crate::BitReader<SBIFR>;
impl SBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBIFR {
        match self.bits {
            false => SBIFR::NoTrigger,
            true => SBIFR::Trigger,
        }
    }
    #[doc = "No break event occurred"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SBIFR::NoTrigger
    }
    #[doc = "An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the TIMx_DIER register"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == SBIFR::Trigger
    }
}
#[doc = "System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBIFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<SBIFW> for bool {
    #[inline(always)]
    fn from(variant: SBIFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBIF` writer - System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation."]
pub type SBIF_W<'a, REG> = crate::BitWriter0C<'a, REG, SBIFW>;
impl<'a, REG> SBIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SBIFW::Clear)
    }
}
#[doc = "Field `CC5IF` reader - Compare 5 interrupt flag Refer to CC1IF description Note: Channel 5 can only be configured as output."]
pub use CC1IF_R as CC5IF_R;
#[doc = "Field `CC6IF` reader - Compare 6 interrupt flag Refer to CC1IF description Note: Channel 6 can only be configured as output."]
pub use CC1IF_R as CC6IF_R;
#[doc = "Field `CC5IF` writer - Compare 5 interrupt flag Refer to CC1IF description Note: Channel 5 can only be configured as output."]
pub use CC1IF_W as CC5IF_W;
#[doc = "Field `CC6IF` writer - Compare 6 interrupt flag Refer to CC1IF description Note: Channel 6 can only be configured as output."]
pub use CC1IF_W as CC6IF_W;
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
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to Section 65.6.3: TIM1 slave mode control register (TIM1_SMCR)), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare 2 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare 3 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/compare 4 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software."]
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active."]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active."]
    #[inline(always)]
    pub fn b2if(&self) -> B2IF_R {
        B2IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/compare 3 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/compare 4 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation."]
    #[inline(always)]
    pub fn sbif(&self) -> SBIF_R {
        SBIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag Refer to CC1IF description Note: Channel 5 can only be configured as output."]
    #[inline(always)]
    pub fn cc5if(&self) -> CC5IF_R {
        CC5IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Compare 6 interrupt flag Refer to CC1IF description Note: Channel 6 can only be configured as output."]
    #[inline(always)]
    pub fn cc6if(&self) -> CC6IF_R {
        CC6IF_R::new(((self.bits >> 17) & 1) != 0)
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
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to Section 65.6.3: TIM1 slave mode control register (TIM1_SMCR)), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
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
    #[doc = "Bit 2 - Capture/compare 2 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CC2IF_W<SRrs> {
        CC2IF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare 3 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3if(&mut self) -> CC3IF_W<SRrs> {
        CC3IF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/compare 4 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4if(&mut self) -> CC4IF_W<SRrs> {
        CC4IF_W::new(self, 4)
    }
    #[doc = "Bit 5 - COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn comif(&mut self) -> COMIF_W<SRrs> {
        COMIF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<SRrs> {
        TIF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active."]
    #[inline(always)]
    #[must_use]
    pub fn bif(&mut self) -> BIF_W<SRrs> {
        BIF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active."]
    #[inline(always)]
    #[must_use]
    pub fn b2if(&mut self) -> B2IF_W<SRrs> {
        B2IF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CC1OF_W<SRrs> {
        CC1OF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2of(&mut self) -> CC2OF_W<SRrs> {
        CC2OF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/compare 3 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3of(&mut self) -> CC3OF_W<SRrs> {
        CC3OF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/compare 4 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4of(&mut self) -> CC4OF_W<SRrs> {
        CC4OF_W::new(self, 12)
    }
    #[doc = "Bit 13 - System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation."]
    #[inline(always)]
    #[must_use]
    pub fn sbif(&mut self) -> SBIF_W<SRrs> {
        SBIF_W::new(self, 13)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag Refer to CC1IF description Note: Channel 5 can only be configured as output."]
    #[inline(always)]
    #[must_use]
    pub fn cc5if(&mut self) -> CC5IF_W<SRrs> {
        CC5IF_W::new(self, 16)
    }
    #[doc = "Bit 17 - Compare 6 interrupt flag Refer to CC1IF description Note: Channel 6 can only be configured as output."]
    #[inline(always)]
    #[must_use]
    pub fn cc6if(&mut self) -> CC6IF_W<SRrs> {
        CC6IF_W::new(self, 17)
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
#[doc = "TIM1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x00f3_3fff;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
