///Register `CCR4` reader
pub type R = crate::R<CCR4rs>;
///Register `CCR4` writer
pub type W = crate::W<CCR4rs>;
/**Field `CCR4` reader - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR4 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc4 output. Non-dithering mode (DITHEN = 0) The register holds the compare value. Dithering mode (DITHEN = 1) The register holds the integer part in CCR4\[31:4\]. The CCR4\[3:0\]
bitfield contains the dithered part. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (tim_ic4). The TIMx_CCR4 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value. Dithering mode (DITHEN = 1) The register holds the capture in CCR4\[31:0\]. The CCR4\[3:0\]
bits are reset.*/
pub type CCR4_R = crate::FieldReader<u32>;
/**Field `CCR4` writer - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR4 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc4 output. Non-dithering mode (DITHEN = 0) The register holds the compare value. Dithering mode (DITHEN = 1) The register holds the integer part in CCR4\[31:4\]. The CCR4\[3:0\]
bitfield contains the dithered part. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (tim_ic4). The TIMx_CCR4 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value. Dithering mode (DITHEN = 1) The register holds the capture in CCR4\[31:0\]. The CCR4\[3:0\]
bits are reset.*/
pub type CCR4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
///Field `DITHER` reader - Dithered part in dithering mode
pub type DITHER_R = crate::FieldReader;
///Field `DITHER` writer - Dithered part in dithering mode
pub type DITHER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `INTEGER` reader - Integer part in dithering mode
pub type INTEGER_R = crate::FieldReader<u32>;
///Field `INTEGER` writer - Integer part in dithering mode
pub type INTEGER_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    /**Bits 0:31 - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR4 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc4 output. Non-dithering mode (DITHEN = 0) The register holds the compare value. Dithering mode (DITHEN = 1) The register holds the integer part in CCR4\[31:4\]. The CCR4\[3:0\]
    bitfield contains the dithered part. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (tim_ic4). The TIMx_CCR4 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value. Dithering mode (DITHEN = 1) The register holds the capture in CCR4\[31:0\]. The CCR4\[3:0\]
    bits are reset.*/
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new(self.bits)
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
        f.debug_struct("CCR4")
            .field("ccr4", &self.ccr4())
            .field("integer", &self.integer())
            .field("dither", &self.dither())
            .finish()
    }
}
impl W {
    /**Bits 0:31 - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR4 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc4 output. Non-dithering mode (DITHEN = 0) The register holds the compare value. Dithering mode (DITHEN = 1) The register holds the integer part in CCR4\[31:4\]. The CCR4\[3:0\]
    bitfield contains the dithered part. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (tim_ic4). The TIMx_CCR4 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value. Dithering mode (DITHEN = 1) The register holds the capture in CCR4\[31:0\]. The CCR4\[3:0\]
    bits are reset.*/
    #[inline(always)]
    pub fn ccr4(&mut self) -> CCR4_W<CCR4rs> {
        CCR4_W::new(self, 0)
    }
    ///Bits 0:3 - Dithered part in dithering mode
    #[inline(always)]
    pub fn dither(&mut self) -> DITHER_W<CCR4rs> {
        DITHER_W::new(self, 0)
    }
    ///Bits 4:31 - Integer part in dithering mode
    #[inline(always)]
    pub fn integer(&mut self) -> INTEGER_W<CCR4rs> {
        INTEGER_W::new(self, 4)
    }
}
/**TIM2 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`ccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#TIM2:CCR4)*/
pub struct CCR4rs;
impl crate::RegisterSpec for CCR4rs {
    type Ux = u32;
}
///`read()` method returns [`ccr4::R`](R) reader structure
impl crate::Readable for CCR4rs {}
///`write(|w| ..)` method takes [`ccr4::W`](W) writer structure
impl crate::Writable for CCR4rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR4 to value 0
impl crate::Resettable for CCR4rs {
    const RESET_VALUE: u32 = 0;
}
