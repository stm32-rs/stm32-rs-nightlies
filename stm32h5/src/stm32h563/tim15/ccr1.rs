#[doc = "Register `CCR1` reader"]
pub type R = crate::R<CCR1rs>;
#[doc = "Register `CCR1` writer"]
pub type W = crate::W<CCR1rs>;
#[doc = "Field `CCR1` reader - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signaled on tim_oc1 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bitfield contains the dithered part. If channel CC1 is configured as input: CR1 is the counter value transferred by the last input capture 1 event (tim_ic1). The TIMx_CCR1 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bits are reset."]
pub type CCR1_R = crate::FieldReader<u32>;
#[doc = "Field `CCR1` writer - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signaled on tim_oc1 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bitfield contains the dithered part. If channel CC1 is configured as input: CR1 is the counter value transferred by the last input capture 1 event (tim_ic1). The TIMx_CCR1 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bits are reset."]
pub type CCR1_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `DITHER` reader - Dithered part in dithering mode"]
pub type DITHER_R = crate::FieldReader;
#[doc = "Field `DITHER` writer - Dithered part in dithering mode"]
pub type DITHER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTEGER` reader - Integer part in dithering mode"]
pub type INTEGER_R = crate::FieldReader<u16>;
#[doc = "Field `INTEGER` writer - Integer part in dithering mode"]
pub type INTEGER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:19 - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signaled on tim_oc1 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bitfield contains the dithered part. If channel CC1 is configured as input: CR1 is the counter value transferred by the last input capture 1 event (tim_ic1). The TIMx_CCR1 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bits are reset."]
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new(self.bits & 0x000f_ffff)
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
    #[doc = "Bits 0:19 - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIM15_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIM15_CNT and signaled on tim_oc1 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bitfield contains the dithered part. If channel CC1 is configured as input: CR1 is the counter value transferred by the last input capture 1 event (tim_ic1). The TIMx_CCR1 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR1\\[15:0\\]. The CCR1\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR1\\[19:4\\]. The CCR1\\[3:0\\]
bits are reset."]
    #[inline(always)]
    #[must_use]
    pub fn ccr1(&mut self) -> CCR1_W<CCR1rs> {
        CCR1_W::new(self, 0)
    }
    #[doc = "Bits 0:3 - Dithered part in dithering mode"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DITHER_W<CCR1rs> {
        DITHER_W::new(self, 0)
    }
    #[doc = "Bits 4:19 - Integer part in dithering mode"]
    #[inline(always)]
    #[must_use]
    pub fn integer(&mut self) -> INTEGER_W<CCR1rs> {
        INTEGER_W::new(self, 4)
    }
}
#[doc = "TIM15 capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR1rs;
impl crate::RegisterSpec for CCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr1::R`](R) reader structure"]
impl crate::Readable for CCR1rs {}
#[doc = "`write(|w| ..)` method takes [`ccr1::W`](W) writer structure"]
impl crate::Writable for CCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR1 to value 0"]
impl crate::Resettable for CCR1rs {
    const RESET_VALUE: u32 = 0;
}
