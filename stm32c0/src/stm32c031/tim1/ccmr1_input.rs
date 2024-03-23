#[doc = "Register `CCMR1_input` reader"]
pub type R = crate::R<CCMR1_INPUTrs>;
#[doc = "Register `CCMR1_input` writer"]
pub type W = crate::W<CCMR1_INPUTrs>;
#[doc = "Field `CC1S` reader - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = ‘0’ in TIMx_CCER)."]
pub type CC1S_R = crate::FieldReader;
#[doc = "Field `CC1S` writer - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = ‘0’ in TIMx_CCER)."]
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1PSC` reader - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=’0’ (TIMx_CCER register)."]
pub type IC1PSC_R = crate::FieldReader;
#[doc = "Field `IC1PSC` writer - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=’0’ (TIMx_CCER register)."]
pub type IC1PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1F` reader - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type IC1F_R = crate::FieldReader;
#[doc = "Field `IC1F` writer - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type IC1F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC2S` reader - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = ‘0’ in TIMx_CCER)."]
pub type CC2S_R = crate::FieldReader;
#[doc = "Field `CC2S` writer - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = ‘0’ in TIMx_CCER)."]
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC2PSC` reader - Input capture 2 prescaler Refer to IC1PSC\\[1:0\\]
description."]
pub type IC2PSC_R = crate::FieldReader;
#[doc = "Field `IC2PSC` writer - Input capture 2 prescaler Refer to IC1PSC\\[1:0\\]
description."]
pub type IC2PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC2F` reader - Input capture 2 filter Refer to IC1F\\[3:0\\]
description."]
pub type IC2F_R = crate::FieldReader;
#[doc = "Field `IC2F` writer - Input capture 2 filter Refer to IC1F\\[3:0\\]
description."]
pub type IC2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=’0’ (TIMx_CCER register)."]
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 2 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<CCMR1_INPUTrs> {
        CC1S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=’0’ (TIMx_CCER register)."]
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> IC1PSC_W<CCMR1_INPUTrs> {
        IC1PSC_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> IC1F_W<CCMR1_INPUTrs> {
        IC1F_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<CCMR1_INPUTrs> {
        CC2S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn ic2psc(&mut self) -> IC2PSC_W<CCMR1_INPUTrs> {
        IC2PSC_W::new(self, 10)
    }
    #[doc = "Bits 12:15 - Input capture 2 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn ic2f(&mut self) -> IC2F_W<CCMR1_INPUTrs> {
        IC2F_W::new(self, 12)
    }
}
#[doc = "TIM1 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR1_INPUTrs;
impl crate::RegisterSpec for CCMR1_INPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_input::R`](R) reader structure"]
impl crate::Readable for CCMR1_INPUTrs {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_input::W`](W) writer structure"]
impl crate::Writable for CCMR1_INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR1_input to value 0"]
impl crate::Resettable for CCMR1_INPUTrs {
    const RESET_VALUE: u32 = 0;
}
