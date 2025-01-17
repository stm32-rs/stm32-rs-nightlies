///Register `CCR2` reader
pub type R = crate::R<CCR2rs>;
///Register `CCR2` writer
pub type W = crate::W<CCR2rs>;
/**Field `CCR2` reader - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signalled on tim_oc2 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR2\[15:0\]. The CCR2\[19:16\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR2\[19:4\]. The CCR2\[3:0\]
bitfield contains the dithered part. If channel CC2 is configured as input: CCR2 is the counter value transferred by the last input capture 1 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR2\[15:0\]. The CCR2\[19:16\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR2\[19:4\]. The CCR2\[3:0\]
bits are reset.*/
pub type CCR2_R = crate::FieldReader<u32>;
/**Field `CCR2` writer - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signalled on tim_oc2 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR2\[15:0\]. The CCR2\[19:16\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR2\[19:4\]. The CCR2\[3:0\]
bitfield contains the dithered part. If channel CC2 is configured as input: CCR2 is the counter value transferred by the last input capture 1 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR2\[15:0\]. The CCR2\[19:16\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR2\[19:4\]. The CCR2\[3:0\]
bits are reset.*/
pub type CCR2_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    /**Bits 0:19 - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signalled on tim_oc2 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR2\[15:0\]. The CCR2\[19:16\]
    bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR2\[19:4\]. The CCR2\[3:0\]
    bitfield contains the dithered part. If channel CC2 is configured as input: CCR2 is the counter value transferred by the last input capture 1 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR2\[15:0\]. The CCR2\[19:16\]
    bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR2\[19:4\]. The CCR2\[3:0\]
    bits are reset.*/
    #[inline(always)]
    pub fn ccr2(&self) -> CCR2_R {
        CCR2_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR2").field("ccr2", &self.ccr2()).finish()
    }
}
impl W {
    /**Bits 0:19 - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signalled on tim_oc2 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR2\[15:0\]. The CCR2\[19:16\]
    bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR2\[19:4\]. The CCR2\[3:0\]
    bitfield contains the dithered part. If channel CC2 is configured as input: CCR2 is the counter value transferred by the last input capture 1 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR2\[15:0\]. The CCR2\[19:16\]
    bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR2\[19:4\]. The CCR2\[3:0\]
    bits are reset.*/
    #[inline(always)]
    pub fn ccr2(&mut self) -> CCR2_W<CCR2rs> {
        CCR2_W::new(self, 0)
    }
}
/**TIM15 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`ccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#TIM15:CCR2)*/
pub struct CCR2rs;
impl crate::RegisterSpec for CCR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccr2::R`](R) reader structure
impl crate::Readable for CCR2rs {}
///`write(|w| ..)` method takes [`ccr2::W`](W) writer structure
impl crate::Writable for CCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR2 to value 0
impl crate::Resettable for CCR2rs {
    const RESET_VALUE: u32 = 0;
}
