#[doc = "Register `CCR6` reader"]
pub type R = crate::R<CCR6rs>;
#[doc = "Register `CCR6` writer"]
pub type W = crate::W<CCR6rs>;
#[doc = "Field `CCR6` reader - Capture/compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc6 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR6\\[15:0\\]. The CCR6\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR6\\[19:4\\]. The CCR6\\[3:0\\]
bitfield contains the dithered part."]
pub type CCR6_R = crate::FieldReader<u32>;
#[doc = "Field `CCR6` writer - Capture/compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc6 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR6\\[15:0\\]. The CCR6\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR6\\[19:4\\]. The CCR6\\[3:0\\]
bitfield contains the dithered part."]
pub type CCR6_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 20, u32>;
#[doc = "Field `DITHER` reader - Dithered part in dithering mode"]
pub type DITHER_R = crate::FieldReader;
#[doc = "Field `DITHER` writer - Dithered part in dithering mode"]
pub type DITHER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTEGER` reader - Integer part in dithering mode"]
pub type INTEGER_R = crate::FieldReader<u16>;
#[doc = "Field `INTEGER` writer - Integer part in dithering mode"]
pub type INTEGER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:19 - Capture/compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc6 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR6\\[15:0\\]. The CCR6\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR6\\[19:4\\]. The CCR6\\[3:0\\]
bitfield contains the dithered part."]
    #[inline(always)]
    pub fn ccr6(&self) -> CCR6_R {
        CCR6_R::new(self.bits & 0x000f_ffff)
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
}
impl W {
    #[doc = "Bits 0:19 - Capture/compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc6 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR6\\[15:0\\]. The CCR6\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR6\\[19:4\\]. The CCR6\\[3:0\\]
bitfield contains the dithered part."]
    #[inline(always)]
    #[must_use]
    pub fn ccr6(&mut self) -> CCR6_W<CCR6rs> {
        CCR6_W::new(self, 0)
    }
    #[doc = "Bits 0:3 - Dithered part in dithering mode"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DITHER_W<CCR6rs> {
        DITHER_W::new(self, 0)
    }
    #[doc = "Bits 4:19 - Integer part in dithering mode"]
    #[inline(always)]
    #[must_use]
    pub fn integer(&mut self) -> INTEGER_W<CCR6rs> {
        INTEGER_W::new(self, 4)
    }
}
#[doc = "TIM1 capture/compare register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR6rs;
impl crate::RegisterSpec for CCR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr6::R`](R) reader structure"]
impl crate::Readable for CCR6rs {}
#[doc = "`write(|w| ..)` method takes [`ccr6::W`](W) writer structure"]
impl crate::Writable for CCR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR6 to value 0"]
impl crate::Resettable for CCR6rs {
    const RESET_VALUE: u32 = 0;
}
