#[doc = "Register `TIM1_SR` reader"]
pub type R = crate::R<TIM1_SRrs>;
#[doc = "Register `TIM1_SR` writer"]
pub type W = crate::W<TIM1_SRrs>;
#[doc = "Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIMx_SMCR)(x = 1, 8)), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
pub type UIF_R = crate::BitReader;
#[doc = "Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIMx_SMCR)(x = 1, 8)), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IF` reader - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
pub type CC1IF_R = crate::BitReader;
#[doc = "Field `CC1IF` writer - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IF` reader - Capture/compare 2 interrupt flag Refer to CC1IF description"]
pub type CC2IF_R = crate::BitReader;
#[doc = "Field `CC2IF` writer - Capture/compare 2 interrupt flag Refer to CC1IF description"]
pub type CC2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IF` reader - Capture/compare 3 interrupt flag Refer to CC1IF description"]
pub type CC3IF_R = crate::BitReader;
#[doc = "Field `CC3IF` writer - Capture/compare 3 interrupt flag Refer to CC1IF description"]
pub type CC3IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IF` reader - Capture/compare 4 interrupt flag Refer to CC1IF description"]
pub type CC4IF_R = crate::BitReader;
#[doc = "Field `CC4IF` writer - Capture/compare 4 interrupt flag Refer to CC1IF description"]
pub type CC4IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIF` reader - COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software."]
pub type COMIF_R = crate::BitReader;
#[doc = "Field `COMIF` writer - COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software."]
pub type COMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF` reader - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
pub type TIF_R = crate::BitReader;
#[doc = "Field `TIF` writer - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
pub type TIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIF` reader - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active."]
pub type BIF_R = crate::BitReader;
#[doc = "Field `BIF` writer - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active."]
pub type BIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2IF` reader - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active."]
pub type B2IF_R = crate::BitReader;
#[doc = "Field `B2IF` writer - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active."]
pub type B2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OF` reader - Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0â\u{80}\u{99}."]
pub type CC1OF_R = crate::BitReader;
#[doc = "Field `CC1OF` writer - Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0â\u{80}\u{99}."]
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OF` reader - Capture/compare 2 overcapture flag Refer to CC1OF description"]
pub type CC2OF_R = crate::BitReader;
#[doc = "Field `CC2OF` writer - Capture/compare 2 overcapture flag Refer to CC1OF description"]
pub type CC2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3OF` reader - Capture/compare 3 overcapture flag Refer to CC1OF description"]
pub type CC3OF_R = crate::BitReader;
#[doc = "Field `CC3OF` writer - Capture/compare 3 overcapture flag Refer to CC1OF description"]
pub type CC3OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4OF` reader - Capture/compare 4 overcapture flag Refer to CC1OF description"]
pub type CC4OF_R = crate::BitReader;
#[doc = "Field `CC4OF` writer - Capture/compare 4 overcapture flag Refer to CC1OF description"]
pub type CC4OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBIF` reader - System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation."]
pub type SBIF_R = crate::BitReader;
#[doc = "Field `SBIF` writer - System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation."]
pub type SBIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5IF` reader - Compare 5 interrupt flag Refer to CC1IF description Note: Channel 5 can only be configured as output."]
pub type CC5IF_R = crate::BitReader;
#[doc = "Field `CC5IF` writer - Compare 5 interrupt flag Refer to CC1IF description Note: Channel 5 can only be configured as output."]
pub type CC5IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6IF` reader - Compare 6 interrupt flag Refer to CC1IF description Note: Channel 6 can only be configured as output."]
pub type CC6IF_R = crate::BitReader;
#[doc = "Field `CC6IF` writer - Compare 6 interrupt flag Refer to CC1IF description Note: Channel 6 can only be configured as output."]
pub type CC6IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDXF` reader - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to '0â\u{80}\u{99}."]
pub type IDXF_R = crate::BitReader;
#[doc = "Field `IDXF` writer - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to '0â\u{80}\u{99}."]
pub type IDXF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRF` reader - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to '0â\u{80}\u{99}."]
pub type DIRF_R = crate::BitReader;
#[doc = "Field `DIRF` writer - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to '0â\u{80}\u{99}."]
pub type DIRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IERRF` reader - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to '0â\u{80}\u{99}."]
pub type IERRF_R = crate::BitReader;
#[doc = "Field `IERRF` writer - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to '0â\u{80}\u{99}."]
pub type IERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRF` reader - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to '0â\u{80}\u{99}."]
pub type TERRF_R = crate::BitReader;
#[doc = "Field `TERRF` writer - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to '0â\u{80}\u{99}."]
pub type TERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIMx_SMCR)(x = 1, 8)), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
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
    #[doc = "Bit 9 - Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0â\u{80}\u{99}."]
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
    #[doc = "Bit 20 - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to '0â\u{80}\u{99}."]
    #[inline(always)]
    pub fn idxf(&self) -> IDXF_R {
        IDXF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to '0â\u{80}\u{99}."]
    #[inline(always)]
    pub fn dirf(&self) -> DIRF_R {
        DIRF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to '0â\u{80}\u{99}."]
    #[inline(always)]
    pub fn ierrf(&self) -> IERRF_R {
        IERRF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to '0â\u{80}\u{99}."]
    #[inline(always)]
    pub fn terrf(&self) -> TERRF_R {
        TERRF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIMx_SMCR)(x = 1, 8)), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<TIM1_SRrs> {
        UIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<TIM1_SRrs> {
        CC1IF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/compare 2 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CC2IF_W<TIM1_SRrs> {
        CC2IF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare 3 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3if(&mut self) -> CC3IF_W<TIM1_SRrs> {
        CC3IF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/compare 4 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4if(&mut self) -> CC4IF_W<TIM1_SRrs> {
        CC4IF_W::new(self, 4)
    }
    #[doc = "Bit 5 - COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn comif(&mut self) -> COMIF_W<TIM1_SRrs> {
        COMIF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<TIM1_SRrs> {
        TIF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active."]
    #[inline(always)]
    #[must_use]
    pub fn bif(&mut self) -> BIF_W<TIM1_SRrs> {
        BIF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active."]
    #[inline(always)]
    #[must_use]
    pub fn b2if(&mut self) -> B2IF_W<TIM1_SRrs> {
        B2IF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0â\u{80}\u{99}."]
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CC1OF_W<TIM1_SRrs> {
        CC1OF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2of(&mut self) -> CC2OF_W<TIM1_SRrs> {
        CC2OF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/compare 3 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3of(&mut self) -> CC3OF_W<TIM1_SRrs> {
        CC3OF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/compare 4 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4of(&mut self) -> CC4OF_W<TIM1_SRrs> {
        CC4OF_W::new(self, 12)
    }
    #[doc = "Bit 13 - System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation."]
    #[inline(always)]
    #[must_use]
    pub fn sbif(&mut self) -> SBIF_W<TIM1_SRrs> {
        SBIF_W::new(self, 13)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag Refer to CC1IF description Note: Channel 5 can only be configured as output."]
    #[inline(always)]
    #[must_use]
    pub fn cc5if(&mut self) -> CC5IF_W<TIM1_SRrs> {
        CC5IF_W::new(self, 16)
    }
    #[doc = "Bit 17 - Compare 6 interrupt flag Refer to CC1IF description Note: Channel 6 can only be configured as output."]
    #[inline(always)]
    #[must_use]
    pub fn cc6if(&mut self) -> CC6IF_W<TIM1_SRrs> {
        CC6IF_W::new(self, 17)
    }
    #[doc = "Bit 20 - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to '0â\u{80}\u{99}."]
    #[inline(always)]
    #[must_use]
    pub fn idxf(&mut self) -> IDXF_W<TIM1_SRrs> {
        IDXF_W::new(self, 20)
    }
    #[doc = "Bit 21 - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to '0â\u{80}\u{99}."]
    #[inline(always)]
    #[must_use]
    pub fn dirf(&mut self) -> DIRF_W<TIM1_SRrs> {
        DIRF_W::new(self, 21)
    }
    #[doc = "Bit 22 - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to '0â\u{80}\u{99}."]
    #[inline(always)]
    #[must_use]
    pub fn ierrf(&mut self) -> IERRF_W<TIM1_SRrs> {
        IERRF_W::new(self, 22)
    }
    #[doc = "Bit 23 - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to '0â\u{80}\u{99}."]
    #[inline(always)]
    #[must_use]
    pub fn terrf(&mut self) -> TERRF_W<TIM1_SRrs> {
        TERRF_W::new(self, 23)
    }
}
#[doc = "TIM1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_SRrs;
impl crate::RegisterSpec for TIM1_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_sr::R`](R) reader structure"]
impl crate::Readable for TIM1_SRrs {}
#[doc = "`write(|w| ..)` method takes [`tim1_sr::W`](W) writer structure"]
impl crate::Writable for TIM1_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_SR to value 0"]
impl crate::Resettable for TIM1_SRrs {
    const RESET_VALUE: u32 = 0;
}
