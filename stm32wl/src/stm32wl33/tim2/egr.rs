///Register `EGR` reader
pub type R = crate::R<EGRrs>;
///Register `EGR` writer
pub type W = crate::W<EGRrs>;
///Field `UG` writer - UG: Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action. 1: Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected).
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1G` writer - CC1G: Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action. 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2G` writer - CC2G: Capture/Compare 2 generation refer to CC1G description
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3G` writer - CC3G: Capture/Compare 3 generation refer to CC1G description
pub type CC3G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4G` writer - CC4G: Capture/Compare 4 generation refer to CC1G description
pub type CC4G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TG` writer - TG: Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in TIMx_SR register. Related interrupt can occur if enabled.
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EGR").finish()
    }
}
impl W {
    ///Bit 0 - UG: Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action. 1: Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected).
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<'_, EGRrs> {
        UG_W::new(self, 0)
    }
    ///Bit 1 - CC1G: Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action. 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<'_, EGRrs> {
        CC1G_W::new(self, 1)
    }
    ///Bit 2 - CC2G: Capture/Compare 2 generation refer to CC1G description
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W<'_, EGRrs> {
        CC2G_W::new(self, 2)
    }
    ///Bit 3 - CC3G: Capture/Compare 3 generation refer to CC1G description
    #[inline(always)]
    pub fn cc3g(&mut self) -> CC3G_W<'_, EGRrs> {
        CC3G_W::new(self, 3)
    }
    ///Bit 4 - CC4G: Capture/Compare 4 generation refer to CC1G description
    #[inline(always)]
    pub fn cc4g(&mut self) -> CC4G_W<'_, EGRrs> {
        CC4G_W::new(self, 4)
    }
    ///Bit 6 - TG: Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in TIMx_SR register. Related interrupt can occur if enabled.
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<'_, EGRrs> {
        TG_W::new(self, 6)
    }
}
/**EGR register

You can [`read`](crate::Reg::read) this register and get [`egr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM2:EGR)*/
pub struct EGRrs;
impl crate::RegisterSpec for EGRrs {
    type Ux = u32;
}
///`read()` method returns [`egr::R`](R) reader structure
impl crate::Readable for EGRrs {}
///`write(|w| ..)` method takes [`egr::W`](W) writer structure
impl crate::Writable for EGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EGR to value 0
impl crate::Resettable for EGRrs {}
