#[doc = "Register `CCR5` reader"]
pub type R = crate::R<CCR5rs>;
#[doc = "Register `CCR5` writer"]
pub type W = crate::W<CCR5rs>;
#[doc = "Field `CCR5` reader - Capture/compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc5 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR5\\[15:0\\]. The CCR5\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR5\\[19:4\\]. The CCR5\\[3:0\\]
bitfield contains the dithered part."]
pub type CCR5_R = crate::FieldReader<u32>;
#[doc = "Field `CCR5` writer - Capture/compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc5 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR5\\[15:0\\]. The CCR5\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR5\\[19:4\\]. The CCR5\\[3:0\\]
bitfield contains the dithered part."]
pub type CCR5_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 20, u32>;
#[doc = "Field `DITHER` reader - Dithered part in dithering mode"]
pub type DITHER_R = crate::FieldReader;
#[doc = "Field `DITHER` writer - Dithered part in dithering mode"]
pub type DITHER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTEGER` reader - Integer part in dithering mode"]
pub type INTEGER_R = crate::FieldReader<u16>;
#[doc = "Field `INTEGER` writer - Integer part in dithering mode"]
pub type INTEGER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `GC5C1` reader - Group channel 5 and channel 1 Distortion on channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C1_R = crate::BitReader;
#[doc = "Field `GC5C1` writer - Group channel 5 and channel 1 Distortion on channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC5C2` reader - Group channel 5 and channel 2 Distortion on channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C2_R = crate::BitReader;
#[doc = "Field `GC5C2` writer - Group channel 5 and channel 2 Distortion on channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC5C3` reader - Group channel 5 and channel 3 Distortion on channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C3_R = crate::BitReader;
#[doc = "Field `GC5C3` writer - Group channel 5 and channel 3 Distortion on channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - Capture/compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc5 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR5\\[15:0\\]. The CCR5\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR5\\[19:4\\]. The CCR5\\[3:0\\]
bitfield contains the dithered part."]
    #[inline(always)]
    pub fn ccr5(&self) -> CCR5_R {
        CCR5_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 0:3 - Dithered part in dithering mode"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:19 - Integer part in dithering mode"]
    #[inline(always)]
    pub fn integer(&self) -> INTEGER_R {
        INTEGER_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - Group channel 5 and channel 1 Distortion on channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Group channel 5 and channel 2 Distortion on channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Group channel 5 and channel 3 Distortion on channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Capture/compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc5 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR5\\[15:0\\]. The CCR5\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR5\\[19:4\\]. The CCR5\\[3:0\\]
bitfield contains the dithered part."]
    #[inline(always)]
    #[must_use]
    pub fn ccr5(&mut self) -> CCR5_W<CCR5rs> {
        CCR5_W::new(self, 0)
    }
    #[doc = "Bits 0:3 - Dithered part in dithering mode"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DITHER_W<CCR5rs> {
        DITHER_W::new(self, 0)
    }
    #[doc = "Bits 4:19 - Integer part in dithering mode"]
    #[inline(always)]
    #[must_use]
    pub fn integer(&mut self) -> INTEGER_W<CCR5rs> {
        INTEGER_W::new(self, 4)
    }
    #[doc = "Bit 29 - Group channel 5 and channel 1 Distortion on channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c1(&mut self) -> GC5C1_W<CCR5rs> {
        GC5C1_W::new(self, 29)
    }
    #[doc = "Bit 30 - Group channel 5 and channel 2 Distortion on channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c2(&mut self) -> GC5C2_W<CCR5rs> {
        GC5C2_W::new(self, 30)
    }
    #[doc = "Bit 31 - Group channel 5 and channel 3 Distortion on channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c3(&mut self) -> GC5C3_W<CCR5rs> {
        GC5C3_W::new(self, 31)
    }
}
#[doc = "TIM1 capture/compare register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR5rs;
impl crate::RegisterSpec for CCR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr5::R`](R) reader structure"]
impl crate::Readable for CCR5rs {}
#[doc = "`write(|w| ..)` method takes [`ccr5::W`](W) writer structure"]
impl crate::Writable for CCR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR5 to value 0"]
impl crate::Resettable for CCR5rs {
    const RESET_VALUE: u32 = 0;
}
