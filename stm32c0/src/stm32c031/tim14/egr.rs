#[doc = "Register `EGR` writer"]
pub type W = crate::W<EGRrs>;
#[doc = "Field `UG` writer - Update generation This bit can be set by software, it is automatically cleared by hardware."]
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` writer - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<EGRrs> {
        UG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<EGRrs> {
        CC1G_W::new(self, 1)
    }
}
#[doc = "TIM14 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EGRrs;
impl crate::RegisterSpec for EGRrs {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGRrs {
    const RESET_VALUE: u16 = 0;
}
