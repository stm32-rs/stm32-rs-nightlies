#[doc = "Register `CCMR1_Output` reader"]
pub type R = crate::R<CCMR1_OUTPUTrs>;
#[doc = "Register `CCMR1_Output` writer"]
pub type W = crate::W<CCMR1_OUTPUTrs>;
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type CC1S_R = crate::FieldReader;
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC1FE` reader - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
pub type OC1FE_R = crate::BitReader;
#[doc = "Field `OC1FE` writer - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
pub type OC1PE_R = crate::BitReader;
#[doc = "Field `OC1PE` writer - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M1` reader - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
pub type OC1M1_R = crate::FieldReader;
#[doc = "Field `OC1M1` writer - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
pub type OC1M1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CC2S` reader - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
pub type CC2S_R = crate::FieldReader;
#[doc = "Field `CC2S` writer - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC2FE` reader - Output compare 2 fast enable"]
pub type OC2FE_R = crate::BitReader;
#[doc = "Field `OC2FE` writer - Output compare 2 fast enable"]
pub type OC2FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2PE` reader - Output compare 2 preload enable"]
pub type OC2PE_R = crate::BitReader;
#[doc = "Field `OC2PE` writer - Output compare 2 preload enable"]
pub type OC2PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2M1` reader - Output compare 2 mode refer to OC1M description on bits 6:4"]
pub type OC2M1_R = crate::FieldReader;
#[doc = "Field `OC2M1` writer - Output compare 2 mode refer to OC1M description on bits 6:4"]
pub type OC2M1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC1M2` reader - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
pub type OC1M2_R = crate::BitReader;
#[doc = "Field `OC1M2` writer - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
pub type OC1M2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2M2` reader - Output compare 2 mode refer to OC1M description on bits 6:4"]
pub type OC2M2_R = crate::BitReader;
#[doc = "Field `OC2M2` writer - Output compare 2 mode refer to OC1M description on bits 6:4"]
pub type OC2M2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
    #[inline(always)]
    pub fn oc1m1(&self) -> OC1M1_R {
        OC1M1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output compare 2 fast enable"]
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 2 preload enable"]
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 2 mode refer to OC1M description on bits 6:4"]
    #[inline(always)]
    pub fn oc2m1(&self) -> OC2M1_R {
        OC2M1_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
    #[inline(always)]
    pub fn oc1m2(&self) -> OC1M2_R {
        OC1M2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output compare 2 mode refer to OC1M description on bits 6:4"]
    #[inline(always)]
    pub fn oc2m2(&self) -> OC2M2_R {
        OC2M2_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<CCMR1_OUTPUTrs> {
        CC1S_W::new(self, 0)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<CCMR1_OUTPUTrs> {
        OC1FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable Note: The PWM mode can be used without validating the preload register only in one-pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<CCMR1_OUTPUTrs> {
        OC1PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
    #[inline(always)]
    #[must_use]
    pub fn oc1m1(&mut self) -> OC1M1_W<CCMR1_OUTPUTrs> {
        OC1M1_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<CCMR1_OUTPUTrs> {
        CC2S_W::new(self, 8)
    }
    #[doc = "Bit 10 - Output compare 2 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2fe(&mut self) -> OC2FE_W<CCMR1_OUTPUTrs> {
        OC2FE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 2 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2pe(&mut self) -> OC2PE_W<CCMR1_OUTPUTrs> {
        OC2PE_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output compare 2 mode refer to OC1M description on bits 6:4"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m1(&mut self) -> OC2M1_W<CCMR1_OUTPUTrs> {
        OC2M1_W::new(self, 12)
    }
    #[doc = "Bit 16 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
    #[inline(always)]
    #[must_use]
    pub fn oc1m2(&mut self) -> OC1M2_W<CCMR1_OUTPUTrs> {
        OC1M2_W::new(self, 16)
    }
    #[doc = "Bit 24 - Output compare 2 mode refer to OC1M description on bits 6:4"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m2(&mut self) -> OC2M2_W<CCMR1_OUTPUTrs> {
        OC2M2_W::new(self, 24)
    }
}
#[doc = "TIM2 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR1_OUTPUTrs;
impl crate::RegisterSpec for CCMR1_OUTPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_output::R`](R) reader structure"]
impl crate::Readable for CCMR1_OUTPUTrs {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure"]
impl crate::Writable for CCMR1_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR1_Output to value 0"]
impl crate::Resettable for CCMR1_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
