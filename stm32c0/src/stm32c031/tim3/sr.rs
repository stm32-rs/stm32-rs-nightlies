#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
pub type UIF_R = crate::BitReader;
#[doc = "Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IF` reader - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
pub type CC1IF_R = crate::BitReader;
#[doc = "Field `CC1IF` writer - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IF` reader - Capture/Compare 2 interrupt flag Refer to CC1IF description"]
pub type CC2IF_R = crate::BitReader;
#[doc = "Field `CC2IF` writer - Capture/Compare 2 interrupt flag Refer to CC1IF description"]
pub type CC2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IF` reader - Capture/Compare 3 interrupt flag Refer to CC1IF description"]
pub type CC3IF_R = crate::BitReader;
#[doc = "Field `CC3IF` writer - Capture/Compare 3 interrupt flag Refer to CC1IF description"]
pub type CC3IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IF` reader - Capture/Compare 4 interrupt flag Refer to CC1IF description"]
pub type CC4IF_R = crate::BitReader;
#[doc = "Field `CC4IF` writer - Capture/Compare 4 interrupt flag Refer to CC1IF description"]
pub type CC4IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF` reader - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
pub type TIF_R = crate::BitReader;
#[doc = "Field `TIF` writer - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
pub type TIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OF` reader - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
pub type CC1OF_R = crate::BitReader;
#[doc = "Field `CC1OF` writer - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’."]
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OF` reader - Capture/compare 2 overcapture flag refer to CC1OF description"]
pub type CC2OF_R = crate::BitReader;
#[doc = "Field `CC2OF` writer - Capture/compare 2 overcapture flag refer to CC1OF description"]
pub type CC2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3OF` reader - Capture/Compare 3 overcapture flag refer to CC1OF description"]
pub type CC3OF_R = crate::BitReader;
#[doc = "Field `CC3OF` writer - Capture/Compare 3 overcapture flag refer to CC1OF description"]
pub type CC3OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4OF` reader - Capture/Compare 4 overcapture flag refer to CC1OF description"]
pub type CC4OF_R = crate::BitReader;
#[doc = "Field `CC4OF` writer - Capture/Compare 4 overcapture flag refer to CC1OF description"]
pub type CC4OF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
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
    #[doc = "Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
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
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<SRrs> {
        UIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
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
    #[doc = "Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
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
}
#[doc = "TIM3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u16 = 0;
}
