#[doc = "Register `CCMR2_output` reader"]
pub type R = crate::R<CCMR2_OUTPUTrs>;
#[doc = "Register `CCMR2_output` writer"]
pub type W = crate::W<CCMR2_OUTPUTrs>;
#[doc = "Field `CC3S` reader - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = ‘0’ in TIMx_CCER)."]
pub type CC3S_R = crate::FieldReader;
#[doc = "Field `CC3S` writer - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = ‘0’ in TIMx_CCER)."]
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC3FE` reader - Output compare 3 fast enable Refer to OC1FE description."]
pub type OC3FE_R = crate::BitReader;
#[doc = "Field `OC3FE` writer - Output compare 3 fast enable Refer to OC1FE description."]
pub type OC3FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3PE` reader - Output compare 3 preload enable Refer to OC1PE description."]
pub type OC3PE_R = crate::BitReader;
#[doc = "Field `OC3PE` writer - Output compare 3 preload enable Refer to OC1PE description."]
pub type OC3PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M1` reader - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
pub type OC3M1_R = crate::FieldReader;
#[doc = "Field `OC3M1` writer - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
pub type OC3M1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC3CE` reader - Output compare 3 clear enable Refer to OC1CE description."]
pub type OC3CE_R = crate::BitReader;
#[doc = "Field `OC3CE` writer - Output compare 3 clear enable Refer to OC1CE description."]
pub type OC3CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4S` reader - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = ‘0’ in TIMx_CCER)."]
pub type CC4S_R = crate::FieldReader;
#[doc = "Field `CC4S` writer - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = ‘0’ in TIMx_CCER)."]
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC4FE` reader - Output compare 4 fast enable Refer to OC1FE description."]
pub type OC4FE_R = crate::BitReader;
#[doc = "Field `OC4FE` writer - Output compare 4 fast enable Refer to OC1FE description."]
pub type OC4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4PE` reader - Output compare 4 preload enable Refer to OC1PE description."]
pub type OC4PE_R = crate::BitReader;
#[doc = "Field `OC4PE` writer - Output compare 4 preload enable Refer to OC1PE description."]
pub type OC4PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M1` reader - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
pub type OC4M1_R = crate::FieldReader;
#[doc = "Field `OC4M1` writer - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
pub type OC4M1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC4CE` reader - Output compare 4 clear enable Refer to OC1CE description."]
pub type OC4CE_R = crate::BitReader;
#[doc = "Field `OC4CE` writer - Output compare 4 clear enable Refer to OC1CE description."]
pub type OC4CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M2` reader - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
pub type OC3M2_R = crate::BitReader;
#[doc = "Field `OC3M2` writer - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
pub type OC3M2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M2` reader - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
pub type OC4M2_R = crate::BitReader;
#[doc = "Field `OC4M2` writer - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
pub type OC4M2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc3m1(&self) -> OC3M1_R {
        OC3M1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc4m1(&self) -> OC4M1_R {
        OC4M1_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc4ce(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc3m2(&self) -> OC3M2_R {
        OC3M2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
    #[inline(always)]
    pub fn oc4m2(&self) -> OC4M2_R {
        OC4M2_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> CC3S_W<CCMR2_OUTPUTrs> {
        CC3S_W::new(self, 0)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable Refer to OC1FE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc3fe(&mut self) -> OC3FE_W<CCMR2_OUTPUTrs> {
        OC3FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable Refer to OC1PE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc3pe(&mut self) -> OC3PE_W<CCMR2_OUTPUTrs> {
        OC3PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn oc3m1(&mut self) -> OC3M1_W<CCMR2_OUTPUTrs> {
        OC3M1_W::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable Refer to OC1CE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc3ce(&mut self) -> OC3CE_W<CCMR2_OUTPUTrs> {
        OC3CE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = ‘0’ in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> CC4S_W<CCMR2_OUTPUTrs> {
        CC4S_W::new(self, 8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable Refer to OC1FE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc4fe(&mut self) -> OC4FE_W<CCMR2_OUTPUTrs> {
        OC4FE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable Refer to OC1PE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc4pe(&mut self) -> OC4PE_W<CCMR2_OUTPUTrs> {
        OC4PE_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn oc4m1(&mut self) -> OC4M1_W<CCMR2_OUTPUTrs> {
        OC4M1_W::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable Refer to OC1CE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc4ce(&mut self) -> OC4CE_W<CCMR2_OUTPUTrs> {
        OC4CE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Output compare 3 mode Refer to OC1M\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn oc3m2(&mut self) -> OC3M2_W<CCMR2_OUTPUTrs> {
        OC3M2_W::new(self, 16)
    }
    #[doc = "Bit 24 - Output compare 4 mode Refer to OC3M\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn oc4m2(&mut self) -> OC4M2_W<CCMR2_OUTPUTrs> {
        OC4M2_W::new(self, 24)
    }
}
#[doc = "TIM1 capture/compare mode register 2 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr2_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR2_OUTPUTrs;
impl crate::RegisterSpec for CCMR2_OUTPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2_output::R`](R) reader structure"]
impl crate::Readable for CCMR2_OUTPUTrs {}
#[doc = "`write(|w| ..)` method takes [`ccmr2_output::W`](W) writer structure"]
impl crate::Writable for CCMR2_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR2_output to value 0"]
impl crate::Resettable for CCMR2_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
