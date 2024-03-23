#[doc = "Register `TIM12_CCMR1_output` reader"]
pub type R = crate::R<TIM12_CCMR1_OUTPUTrs>;
#[doc = "Register `TIM12_CCMR1_output` writer"]
pub type W = crate::W<TIM12_CCMR1_OUTPUTrs>;
#[doc = "Field `CC1S` reader - CC1S"]
pub type CC1S_R = crate::FieldReader;
#[doc = "Field `CC1S` writer - CC1S"]
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC1FE` reader - OC1FE"]
pub type OC1FE_R = crate::BitReader;
#[doc = "Field `OC1FE` writer - OC1FE"]
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - OC1FE"]
pub type OC1PE_R = crate::BitReader;
#[doc = "Field `OC1PE` writer - OC1FE"]
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M` reader - OC1M"]
pub type OC1M_R = crate::FieldReader;
#[doc = "Field `OC1M` writer - OC1M"]
pub type OC1M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CC2S` reader - CC2S"]
pub type CC2S_R = crate::FieldReader;
#[doc = "Field `CC2S` writer - CC2S"]
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC2FE` reader - OC2FE"]
pub type OC2FE_R = crate::BitReader;
#[doc = "Field `OC2FE` writer - OC2FE"]
pub type OC2FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2PE` reader - OC2PE"]
pub type OC2PE_R = crate::BitReader;
#[doc = "Field `OC2PE` writer - OC2PE"]
pub type OC2PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2M` reader - OC2M"]
pub type OC2M_R = crate::FieldReader;
#[doc = "Field `OC2M` writer - OC2M"]
pub type OC2M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC1M_3` reader - OC1M_3"]
pub type OC1M_3_R = crate::BitReader;
#[doc = "Field `OC1M_3` writer - OC1M_3"]
pub type OC1M_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2M_3` reader - OC2M_3"]
pub type OC2M_3_R = crate::BitReader;
#[doc = "Field `OC2M_3` writer - OC2M_3"]
pub type OC2M_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CC1S"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - OC1FE"]
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OC1FE"]
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC1M"]
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - CC2S"]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - OC2FE"]
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OC2PE"]
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - OC2M"]
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - OC1M_3"]
    #[inline(always)]
    pub fn oc1m_3(&self) -> OC1M_3_R {
        OC1M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - OC2M_3"]
    #[inline(always)]
    pub fn oc2m_3(&self) -> OC2M_3_R {
        OC2M_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC1S"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<TIM12_CCMR1_OUTPUTrs> {
        CC1S_W::new(self, 0)
    }
    #[doc = "Bit 2 - OC1FE"]
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<TIM12_CCMR1_OUTPUTrs> {
        OC1FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - OC1FE"]
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<TIM12_CCMR1_OUTPUTrs> {
        OC1PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - OC1M"]
    #[inline(always)]
    #[must_use]
    pub fn oc1m(&mut self) -> OC1M_W<TIM12_CCMR1_OUTPUTrs> {
        OC1M_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - CC2S"]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<TIM12_CCMR1_OUTPUTrs> {
        CC2S_W::new(self, 8)
    }
    #[doc = "Bit 10 - OC2FE"]
    #[inline(always)]
    #[must_use]
    pub fn oc2fe(&mut self) -> OC2FE_W<TIM12_CCMR1_OUTPUTrs> {
        OC2FE_W::new(self, 10)
    }
    #[doc = "Bit 11 - OC2PE"]
    #[inline(always)]
    #[must_use]
    pub fn oc2pe(&mut self) -> OC2PE_W<TIM12_CCMR1_OUTPUTrs> {
        OC2PE_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - OC2M"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m(&mut self) -> OC2M_W<TIM12_CCMR1_OUTPUTrs> {
        OC2M_W::new(self, 12)
    }
    #[doc = "Bit 16 - OC1M_3"]
    #[inline(always)]
    #[must_use]
    pub fn oc1m_3(&mut self) -> OC1M_3_W<TIM12_CCMR1_OUTPUTrs> {
        OC1M_3_W::new(self, 16)
    }
    #[doc = "Bit 24 - OC2M_3"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m_3(&mut self) -> OC2M_3_W<TIM12_CCMR1_OUTPUTrs> {
        OC2M_3_W::new(self, 24)
    }
}
#[doc = "TIM12 capture/compare mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim12_ccmr1_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim12_ccmr1_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM12_CCMR1_OUTPUTrs;
impl crate::RegisterSpec for TIM12_CCMR1_OUTPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim12_ccmr1_output::R`](R) reader structure"]
impl crate::Readable for TIM12_CCMR1_OUTPUTrs {}
#[doc = "`write(|w| ..)` method takes [`tim12_ccmr1_output::W`](W) writer structure"]
impl crate::Writable for TIM12_CCMR1_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM12_CCMR1_output to value 0"]
impl crate::Resettable for TIM12_CCMR1_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
