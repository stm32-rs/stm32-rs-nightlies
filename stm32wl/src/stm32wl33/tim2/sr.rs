///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `UIF` reader - UIF: Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred. 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register.
pub type UIF_R = crate::BitReader;
///Field `UIF` writer - UIF: Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred. 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register.
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1IF` reader - CC1IF: Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value, with some exception in center-aligned mode (refer to the CMS bits in the TIMx_CR1 register description). It is cleared by software. 0: No match. 1: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the contents of TIMx_CCR1 are greater than the contents of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in upcounting and up/down-counting modes) or underflow (in downcounting mode) If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the TIMx_CCR1 register. 0: No input capture occurred 1: The counter value has been captured in TIMx_CCR1 register (An edge has been detected on IC1 which matches the selected polarity)
pub type CC1IF_R = crate::BitReader;
///Field `CC1IF` writer - CC1IF: Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value, with some exception in center-aligned mode (refer to the CMS bits in the TIMx_CR1 register description). It is cleared by software. 0: No match. 1: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the contents of TIMx_CCR1 are greater than the contents of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in upcounting and up/down-counting modes) or underflow (in downcounting mode) If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the TIMx_CCR1 register. 0: No input capture occurred 1: The counter value has been captured in TIMx_CCR1 register (An edge has been detected on IC1 which matches the selected polarity)
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2IF` reader - CC2IF: Capture/Compare 2 interrupt flag refer to CC1IF description
pub type CC2IF_R = crate::BitReader;
///Field `CC2IF` writer - CC2IF: Capture/Compare 2 interrupt flag refer to CC1IF description
pub type CC2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3IF` reader - CC3IF: Capture/Compare 3 interrupt flag refer to CC1IF description
pub type CC3IF_R = crate::BitReader;
///Field `CC3IF` writer - CC3IF: Capture/Compare 3 interrupt flag refer to CC1IF description
pub type CC3IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4IF` reader - CC4IF: Capture/Compare 4 interrupt flag refer to CC1IF description
pub type CC4IF_R = crate::BitReader;
///Field `CC4IF` writer - CC4IF: Capture/Compare 4 interrupt flag refer to CC1IF description
pub type CC4IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIF` reader - TIF: Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.. 0: No trigger event occurred. 1: Trigger interrupt pending.
pub type TIF_R = crate::BitReader;
///Field `TIF` writer - TIF: Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.. 0: No trigger event occurred. 1: Trigger interrupt pending.
pub type TIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1OF` reader - CC1OF: Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected 1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set
pub type CC1OF_R = crate::BitReader;
///Field `CC1OF` writer - CC1OF: Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected 1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2OF` reader - CC2OF: Capture/Compare 2 overcapture flag refer to CC1OF description
pub type CC2OF_R = crate::BitReader;
///Field `CC2OF` writer - CC2OF: Capture/Compare 2 overcapture flag refer to CC1OF description
pub type CC2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3OF` reader - CC3OF: Capture/Compare 3 overcapture flag refer to CC1OF description
pub type CC3OF_R = crate::BitReader;
///Field `CC3OF` writer - CC3OF: Capture/Compare 3 overcapture flag refer to CC1OF description
pub type CC3OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4OF` reader - CC4OF: Capture/Compare 4 overcapture flag refer to CC1OF description
pub type CC4OF_R = crate::BitReader;
///Field `CC4OF` writer - CC4OF: Capture/Compare 4 overcapture flag refer to CC1OF description
pub type CC4OF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - UIF: Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred. 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CC1IF: Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value, with some exception in center-aligned mode (refer to the CMS bits in the TIMx_CR1 register description). It is cleared by software. 0: No match. 1: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the contents of TIMx_CCR1 are greater than the contents of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in upcounting and up/down-counting modes) or underflow (in downcounting mode) If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the TIMx_CCR1 register. 0: No input capture occurred 1: The counter value has been captured in TIMx_CCR1 register (An edge has been detected on IC1 which matches the selected polarity)
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CC2IF: Capture/Compare 2 interrupt flag refer to CC1IF description
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CC3IF: Capture/Compare 3 interrupt flag refer to CC1IF description
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CC4IF: Capture/Compare 4 interrupt flag refer to CC1IF description
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - TIF: Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.. 0: No trigger event occurred. 1: Trigger interrupt pending.
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - CC1OF: Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected 1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CC2OF: Capture/Compare 2 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CC3OF: Capture/Compare 3 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CC4OF: Capture/Compare 4 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("uif", &self.uif())
            .field("cc1if", &self.cc1if())
            .field("cc2if", &self.cc2if())
            .field("cc3if", &self.cc3if())
            .field("cc4if", &self.cc4if())
            .field("tif", &self.tif())
            .field("cc1of", &self.cc1of())
            .field("cc2of", &self.cc2of())
            .field("cc3of", &self.cc3of())
            .field("cc4of", &self.cc4of())
            .finish()
    }
}
impl W {
    ///Bit 0 - UIF: Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred. 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<'_, SRrs> {
        UIF_W::new(self, 0)
    }
    ///Bit 1 - CC1IF: Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value, with some exception in center-aligned mode (refer to the CMS bits in the TIMx_CR1 register description). It is cleared by software. 0: No match. 1: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the contents of TIMx_CCR1 are greater than the contents of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in upcounting and up/down-counting modes) or underflow (in downcounting mode) If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the TIMx_CCR1 register. 0: No input capture occurred 1: The counter value has been captured in TIMx_CCR1 register (An edge has been detected on IC1 which matches the selected polarity)
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<'_, SRrs> {
        CC1IF_W::new(self, 1)
    }
    ///Bit 2 - CC2IF: Capture/Compare 2 interrupt flag refer to CC1IF description
    #[inline(always)]
    pub fn cc2if(&mut self) -> CC2IF_W<'_, SRrs> {
        CC2IF_W::new(self, 2)
    }
    ///Bit 3 - CC3IF: Capture/Compare 3 interrupt flag refer to CC1IF description
    #[inline(always)]
    pub fn cc3if(&mut self) -> CC3IF_W<'_, SRrs> {
        CC3IF_W::new(self, 3)
    }
    ///Bit 4 - CC4IF: Capture/Compare 4 interrupt flag refer to CC1IF description
    #[inline(always)]
    pub fn cc4if(&mut self) -> CC4IF_W<'_, SRrs> {
        CC4IF_W::new(self, 4)
    }
    ///Bit 6 - TIF: Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.. 0: No trigger event occurred. 1: Trigger interrupt pending.
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W<'_, SRrs> {
        TIF_W::new(self, 6)
    }
    ///Bit 9 - CC1OF: Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected 1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W<'_, SRrs> {
        CC1OF_W::new(self, 9)
    }
    ///Bit 10 - CC2OF: Capture/Compare 2 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc2of(&mut self) -> CC2OF_W<'_, SRrs> {
        CC2OF_W::new(self, 10)
    }
    ///Bit 11 - CC3OF: Capture/Compare 3 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc3of(&mut self) -> CC3OF_W<'_, SRrs> {
        CC3OF_W::new(self, 11)
    }
    ///Bit 12 - CC4OF: Capture/Compare 4 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc4of(&mut self) -> CC4OF_W<'_, SRrs> {
        CC4OF_W::new(self, 12)
    }
}
/**SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM2:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
