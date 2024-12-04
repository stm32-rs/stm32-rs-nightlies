///Register `CCR2` reader
pub type R = crate::R<CCR2rs>;
///Register `CCR2` writer
pub type W = crate::W<CCR2rs>;
/**Field `CCR2` reader - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc2 output. Non-dithering mode (DITHEN = 0) The register holds the compare value. Dithering mode (DITHEN = 1) The register holds the integer part in CCR2\[31:4\]. The CCR2\[3:0\]
bitfield contains the dithered part. If channel CC2 is configured as input: CCR2 is the counter value transferred by the last input capture 2 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value. Dithering mode (DITHEN = 1) The register holds the capture in CCR2\[31:0\]. The CCR2\[3:0\]
bits are reset.*/
pub type CCR2_R = crate::FieldReader<u32>;
/**Field `CCR2` writer - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc2 output. Non-dithering mode (DITHEN = 0) The register holds the compare value. Dithering mode (DITHEN = 1) The register holds the integer part in CCR2\[31:4\]. The CCR2\[3:0\]
bitfield contains the dithered part. If channel CC2 is configured as input: CCR2 is the counter value transferred by the last input capture 2 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value. Dithering mode (DITHEN = 1) The register holds the capture in CCR2\[31:0\]. The CCR2\[3:0\]
bits are reset.*/
pub type CCR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
///Field `DITHER` reader - Dithered part in dithering mode
pub type DITHER_R = crate::FieldReader;
///Field `DITHER` writer - Dithered part in dithering mode
pub type DITHER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `INTEGER` reader - Integer part in dithering mode
pub type INTEGER_R = crate::FieldReader<u32>;
///Field `INTEGER` writer - Integer part in dithering mode
pub type INTEGER_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    /**Bits 0:31 - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc2 output. Non-dithering mode (DITHEN = 0) The register holds the compare value. Dithering mode (DITHEN = 1) The register holds the integer part in CCR2\[31:4\]. The CCR2\[3:0\]
    bitfield contains the dithered part. If channel CC2 is configured as input: CCR2 is the counter value transferred by the last input capture 2 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value. Dithering mode (DITHEN = 1) The register holds the capture in CCR2\[31:0\]. The CCR2\[3:0\]
    bits are reset.*/
    #[inline(always)]
    pub fn ccr2(&self) -> CCR2_R {
        CCR2_R::new(self.bits)
    }
    ///Bits 0:3 - Dithered part in dithering mode
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:31 - Integer part in dithering mode
    #[inline(always)]
    pub fn integer(&self) -> INTEGER_R {
        INTEGER_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR2")
            .field("ccr2", &self.ccr2())
            .field("integer", &self.integer())
            .field("dither", &self.dither())
            .finish()
    }
}
impl W {
    /**Bits 0:31 - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc2 output. Non-dithering mode (DITHEN = 0) The register holds the compare value. Dithering mode (DITHEN = 1) The register holds the integer part in CCR2\[31:4\]. The CCR2\[3:0\]
    bitfield contains the dithered part. If channel CC2 is configured as input: CCR2 is the counter value transferred by the last input capture 2 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value. Dithering mode (DITHEN = 1) The register holds the capture in CCR2\[31:0\]. The CCR2\[3:0\]
    bits are reset.*/
    #[inline(always)]
    pub fn ccr2(&mut self) -> CCR2_W<CCR2rs> {
        CCR2_W::new(self, 0)
    }
    ///Bits 0:3 - Dithered part in dithering mode
    #[inline(always)]
    pub fn dither(&mut self) -> DITHER_W<CCR2rs> {
        DITHER_W::new(self, 0)
    }
    ///Bits 4:31 - Integer part in dithering mode
    #[inline(always)]
    pub fn integer(&mut self) -> INTEGER_W<CCR2rs> {
        INTEGER_W::new(self, 4)
    }
}
/**TIM5 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`ccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#TIM5:CCR2)*/
pub struct CCR2rs;
impl crate::RegisterSpec for CCR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccr2::R`](R) reader structure
impl crate::Readable for CCR2rs {}
///`write(|w| ..)` method takes [`ccr2::W`](W) writer structure
impl crate::Writable for CCR2rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR2 to value 0
impl crate::Resettable for CCR2rs {
    const RESET_VALUE: u32 = 0;
}
