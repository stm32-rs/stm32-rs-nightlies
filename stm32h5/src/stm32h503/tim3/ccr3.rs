#[doc = "Register `CCR3` reader"]
pub type R = crate::R<CCR3rs>;
#[doc = "Register `CCR3` writer"]
pub type W = crate::W<CCR3rs>;
#[doc = "Field `CCR3` reader - Capture/compare 3 value If channel CC3 is configured as output: CCR3 is the value to be loaded in the actual capture/compare 3 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC3PE). Else the preload value is copied in the active capture/compare 3 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc3 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR3\\[15:0\\]. The CCR3\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR3\\[19:4\\]. The CCR3\\[3:0\\]
bitfield contains the dithered part. If channel CC3 is configured as input: CCR3 is the counter value transferred by the last input capture 3 event (tim_ic3). The TIMx_CCR3 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The CCR3\\[15:0\\]
bits hold the capture value. The CCR3\\[19:16\\]
bits are reserved. Dithering mode (DITHEN = 1) The register holds the capture in CCR3\\[19:0\\]. The CCR3\\[3:0\\]
bits are reset."]
pub type CCR3_R = crate::FieldReader<u32>;
#[doc = "Field `CCR3` writer - Capture/compare 3 value If channel CC3 is configured as output: CCR3 is the value to be loaded in the actual capture/compare 3 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC3PE). Else the preload value is copied in the active capture/compare 3 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc3 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR3\\[15:0\\]. The CCR3\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR3\\[19:4\\]. The CCR3\\[3:0\\]
bitfield contains the dithered part. If channel CC3 is configured as input: CCR3 is the counter value transferred by the last input capture 3 event (tim_ic3). The TIMx_CCR3 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The CCR3\\[15:0\\]
bits hold the capture value. The CCR3\\[19:16\\]
bits are reserved. Dithering mode (DITHEN = 1) The register holds the capture in CCR3\\[19:0\\]. The CCR3\\[3:0\\]
bits are reset."]
pub type CCR3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 20, u32>;
#[doc = "Field `DITHER` reader - Dithered part in dithering mode"]
pub type DITHER_R = crate::FieldReader;
#[doc = "Field `DITHER` writer - Dithered part in dithering mode"]
pub type DITHER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTEGER` reader - Integer part in dithering mode"]
pub type INTEGER_R = crate::FieldReader<u16>;
#[doc = "Field `INTEGER` writer - Integer part in dithering mode"]
pub type INTEGER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:19 - Capture/compare 3 value If channel CC3 is configured as output: CCR3 is the value to be loaded in the actual capture/compare 3 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC3PE). Else the preload value is copied in the active capture/compare 3 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc3 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR3\\[15:0\\]. The CCR3\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR3\\[19:4\\]. The CCR3\\[3:0\\]
bitfield contains the dithered part. If channel CC3 is configured as input: CCR3 is the counter value transferred by the last input capture 3 event (tim_ic3). The TIMx_CCR3 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The CCR3\\[15:0\\]
bits hold the capture value. The CCR3\\[19:16\\]
bits are reserved. Dithering mode (DITHEN = 1) The register holds the capture in CCR3\\[19:0\\]. The CCR3\\[3:0\\]
bits are reset."]
    #[inline(always)]
    pub fn ccr3(&self) -> CCR3_R {
        CCR3_R::new(self.bits & 0x000f_ffff)
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
    #[doc = "Bits 0:19 - Capture/compare 3 value If channel CC3 is configured as output: CCR3 is the value to be loaded in the actual capture/compare 3 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC3PE). Else the preload value is copied in the active capture/compare 3 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc3 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR3\\[15:0\\]. The CCR3\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR3\\[19:4\\]. The CCR3\\[3:0\\]
bitfield contains the dithered part. If channel CC3 is configured as input: CCR3 is the counter value transferred by the last input capture 3 event (tim_ic3). The TIMx_CCR3 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The CCR3\\[15:0\\]
bits hold the capture value. The CCR3\\[19:16\\]
bits are reserved. Dithering mode (DITHEN = 1) The register holds the capture in CCR3\\[19:0\\]. The CCR3\\[3:0\\]
bits are reset."]
    #[inline(always)]
    #[must_use]
    pub fn ccr3(&mut self) -> CCR3_W<CCR3rs> {
        CCR3_W::new(self, 0)
    }
    #[doc = "Bits 0:3 - Dithered part in dithering mode"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DITHER_W<CCR3rs> {
        DITHER_W::new(self, 0)
    }
    #[doc = "Bits 4:19 - Integer part in dithering mode"]
    #[inline(always)]
    #[must_use]
    pub fn integer(&mut self) -> INTEGER_W<CCR3rs> {
        INTEGER_W::new(self, 4)
    }
}
#[doc = "TIM3 capture/compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR3rs;
impl crate::RegisterSpec for CCR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr3::R`](R) reader structure"]
impl crate::Readable for CCR3rs {}
#[doc = "`write(|w| ..)` method takes [`ccr3::W`](W) writer structure"]
impl crate::Writable for CCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR3 to value 0"]
impl crate::Resettable for CCR3rs {
    const RESET_VALUE: u32 = 0;
}
