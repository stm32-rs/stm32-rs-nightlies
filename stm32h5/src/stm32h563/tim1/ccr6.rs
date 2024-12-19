///Register `CCR6` reader
pub type R = crate::R<CCR6rs>;
///Register `CCR6` writer
pub type W = crate::W<CCR6rs>;
/**Field `CCR6` reader - Capture/compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc6 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR6\[15:0\]. The CCR6\[19:16\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR6\[19:4\]. The CCR6\[3:0\]
bitfield contains the dithered part.*/
pub type CCR6_R = crate::FieldReader<u32>;
/**Field `CCR6` writer - Capture/compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc6 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR6\[15:0\]. The CCR6\[19:16\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR6\[19:4\]. The CCR6\[3:0\]
bitfield contains the dithered part.*/
pub type CCR6_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32, crate::Safe>;
///Field `DITHER` reader - Dithered part in dithering mode
pub type DITHER_R = crate::FieldReader;
///Field `DITHER` writer - Dithered part in dithering mode
pub type DITHER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `INTEGER` reader - Integer part in dithering mode
pub type INTEGER_R = crate::FieldReader<u16>;
///Field `INTEGER` writer - Integer part in dithering mode
pub type INTEGER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    /**Bits 0:19 - Capture/compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc6 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR6\[15:0\]. The CCR6\[19:16\]
    bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR6\[19:4\]. The CCR6\[3:0\]
    bitfield contains the dithered part.*/
    #[inline(always)]
    pub fn ccr6(&self) -> CCR6_R {
        CCR6_R::new(self.bits & 0x000f_ffff)
    }
    ///Bits 0:3 - Dithered part in dithering mode
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:19 - Integer part in dithering mode
    #[inline(always)]
    pub fn integer(&self) -> INTEGER_R {
        INTEGER_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR6")
            .field("ccr6", &self.ccr6())
            .field("integer", &self.integer())
            .field("dither", &self.dither())
            .finish()
    }
}
impl W {
    /**Bits 0:19 - Capture/compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc6 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR6\[15:0\]. The CCR6\[19:16\]
    bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR6\[19:4\]. The CCR6\[3:0\]
    bitfield contains the dithered part.*/
    #[inline(always)]
    pub fn ccr6(&mut self) -> CCR6_W<CCR6rs> {
        CCR6_W::new(self, 0)
    }
    ///Bits 0:3 - Dithered part in dithering mode
    #[inline(always)]
    pub fn dither(&mut self) -> DITHER_W<CCR6rs> {
        DITHER_W::new(self, 0)
    }
    ///Bits 4:19 - Integer part in dithering mode
    #[inline(always)]
    pub fn integer(&mut self) -> INTEGER_W<CCR6rs> {
        INTEGER_W::new(self, 4)
    }
}
/**TIM1 capture/compare register 6

You can [`read`](crate::Reg::read) this register and get [`ccr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#TIM1:CCR6)*/
pub struct CCR6rs;
impl crate::RegisterSpec for CCR6rs {
    type Ux = u32;
}
///`read()` method returns [`ccr6::R`](R) reader structure
impl crate::Readable for CCR6rs {}
///`write(|w| ..)` method takes [`ccr6::W`](W) writer structure
impl crate::Writable for CCR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR6 to value 0
impl crate::Resettable for CCR6rs {
    const RESET_VALUE: u32 = 0;
}
