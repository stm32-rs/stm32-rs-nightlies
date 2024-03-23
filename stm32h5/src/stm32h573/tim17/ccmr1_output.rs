#[doc = "Register `CCMR1_Output` reader"]
pub type R = crate::R<CCMR1_OUTPUTrs>;
#[doc = "Register `CCMR1_Output` writer"]
pub type W = crate::W<CCMR1_OUTPUTrs>;
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = ‘0’ in TIMx_CCER)."]
pub type CC1S_R = crate::FieldReader;
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = ‘0’ in TIMx_CCER)."]
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC1FE` reader - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
pub type OC1FE_R = crate::BitReader;
#[doc = "Field `OC1FE` writer - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=’00’ (the channel is configured in output). The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
pub type OC1PE_R = crate::BitReader;
#[doc = "Field `OC1PE` writer - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=’00’ (the channel is configured in output). The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M1` reader - Output Compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Others: Reserved Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=’00’ (the channel is configured in output). In PWM mode 1 or 2, the tim_oc1ref level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
pub type OC1M1_R = crate::FieldReader;
#[doc = "Field `OC1M1` writer - Output Compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Others: Reserved Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=’00’ (the channel is configured in output). In PWM mode 1 or 2, the tim_oc1ref level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
pub type OC1M1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC1CE` reader - Output Compare 1 clear enable"]
pub type OC1CE_R = crate::BitReader;
#[doc = "Field `OC1CE` writer - Output Compare 1 clear enable"]
pub type OC1CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M2` reader - Output Compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Others: Reserved Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=’00’ (the channel is configured in output). In PWM mode 1 or 2, the tim_oc1ref level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
pub type OC1M2_R = crate::BitReader;
#[doc = "Field `OC1M2` writer - Output Compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Others: Reserved Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=’00’ (the channel is configured in output). In PWM mode 1 or 2, the tim_oc1ref level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
pub type OC1M2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=’00’ (the channel is configured in output). The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Others: Reserved Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=’00’ (the channel is configured in output). In PWM mode 1 or 2, the tim_oc1ref level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
    #[inline(always)]
    pub fn oc1m1(&self) -> OC1M1_R {
        OC1M1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output Compare 1 clear enable"]
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Others: Reserved Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=’00’ (the channel is configured in output). In PWM mode 1 or 2, the tim_oc1ref level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
    #[inline(always)]
    pub fn oc1m2(&self) -> OC1M2_R {
        OC1M2_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<CCMR1_OUTPUTrs> {
        CC1S_W::new(self, 0)
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<CCMR1_OUTPUTrs> {
        OC1FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=’00’ (the channel is configured in output). The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<CCMR1_OUTPUTrs> {
        OC1PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Others: Reserved Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=’00’ (the channel is configured in output). In PWM mode 1 or 2, the tim_oc1ref level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
    #[inline(always)]
    #[must_use]
    pub fn oc1m1(&mut self) -> OC1M1_W<CCMR1_OUTPUTrs> {
        OC1M1_W::new(self, 4)
    }
    #[doc = "Bit 7 - Output Compare 1 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1ce(&mut self) -> OC1CE_W<CCMR1_OUTPUTrs> {
        OC1CE_W::new(self, 7)
    }
    #[doc = "Bit 16 - Output Compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Others: Reserved Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=’00’ (the channel is configured in output). In PWM mode 1 or 2, the tim_oc1ref level changes only when the result of the comparison changes or when the output compare mode switches from “frozen” mode to “PWM” mode."]
    #[inline(always)]
    #[must_use]
    pub fn oc1m2(&mut self) -> OC1M2_W<CCMR1_OUTPUTrs> {
        OC1M2_W::new(self, 16)
    }
}
#[doc = "TIM17 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
