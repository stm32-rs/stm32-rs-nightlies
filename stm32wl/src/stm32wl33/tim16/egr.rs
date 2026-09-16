///Register `EGR` reader
pub type R = crate::R<EGRrs>;
///Register `EGR` writer
pub type W = crate::W<EGRrs>;
///Field `UG` writer - UG: Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action. 1: Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected).
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1G` writer - CC1G: Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action. 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMG` writer - COMG: Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits Note: This bit acts only on channels that have a complementary output.
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TG` writer - TG: Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BG` writer - BG: Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action. 1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled.
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 5 - COMG: Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W<'_, EGRrs> {
        COMG_W::new(self, 5)
    }
    ///Bit 6 - TG: Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<'_, EGRrs> {
        TG_W::new(self, 6)
    }
    ///Bit 7 - BG: Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action. 1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled.
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<'_, EGRrs> {
        BG_W::new(self, 7)
    }
}
/**EGR register

You can [`read`](crate::Reg::read) this register and get [`egr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM16:EGR)*/
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
