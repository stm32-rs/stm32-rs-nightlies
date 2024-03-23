#[doc = "Register `CCMR2_input` reader"]
pub type R = crate::R<CCMR2_INPUTrs>;
#[doc = "Register `CCMR2_input` writer"]
pub type W = crate::W<CCMR2_INPUTrs>;
#[doc = "Field `CC3S` reader - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = ‘0’ in TIMx_CCER)."]
pub type CC3S_R = crate::FieldReader;
#[doc = "Field `CC3S` writer - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = ‘0’ in TIMx_CCER)."]
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3PSC` reader - Input capture 3 prescaler Refer to IC1PSC\\[1:0\\]
description."]
pub type IC3PSC_R = crate::FieldReader;
#[doc = "Field `IC3PSC` writer - Input capture 3 prescaler Refer to IC1PSC\\[1:0\\]
description."]
pub type IC3PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3F` reader - Input capture 3 filter Refer to IC1F\\[3:0\\]
description."]
pub type IC3F_R = crate::FieldReader;
#[doc = "Field `IC3F` writer - Input capture 3 filter Refer to IC1F\\[3:0\\]
description."]
pub type IC3F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC4S` reader - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = ‘0’ in TIMx_CCER)."]
pub type CC4S_R = crate::FieldReader;
#[doc = "Field `CC4S` writer - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = ‘0’ in TIMx_CCER)."]
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4PSC` reader - Input capture 4 prescaler Refer to IC1PSC\\[1:0\\]
description."]
pub type IC4PSC_R = crate::FieldReader;
#[doc = "Field `IC4PSC` writer - Input capture 4 prescaler Refer to IC1PSC\\[1:0\\]
description."]
pub type IC4PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4F` reader - Input capture 4 filter Refer to IC1F\\[3:0\\]
description."]
pub type IC4F_R = crate::FieldReader;
#[doc = "Field `IC4F` writer - Input capture 4 filter Refer to IC1F\\[3:0\\]
description."]
pub type IC4F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> CC3S_W<CCMR2_INPUTrs> {
        CC3S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn ic3psc(&mut self) -> IC3PSC_W<CCMR2_INPUTrs> {
        IC3PSC_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn ic3f(&mut self) -> IC3F_W<CCMR2_INPUTrs> {
        IC3F_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> CC4S_W<CCMR2_INPUTrs> {
        CC4S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn ic4psc(&mut self) -> IC4PSC_W<CCMR2_INPUTrs> {
        IC4PSC_W::new(self, 10)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn ic4f(&mut self) -> IC4F_W<CCMR2_INPUTrs> {
        IC4F_W::new(self, 12)
    }
}
#[doc = "TIM1 capture/compare mode register 2 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr2_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr2_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR2_INPUTrs;
impl crate::RegisterSpec for CCMR2_INPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2_input::R`](R) reader structure"]
impl crate::Readable for CCMR2_INPUTrs {}
#[doc = "`write(|w| ..)` method takes [`ccmr2_input::W`](W) writer structure"]
impl crate::Writable for CCMR2_INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR2_input to value 0"]
impl crate::Resettable for CCMR2_INPUTrs {
    const RESET_VALUE: u32 = 0;
}
