#[doc = "Register `TIMx_CCMR1_Output` reader"]
pub type R = crate::R<TIMX_CCMR1_OUTPUTrs>;
#[doc = "Register `TIMx_CCMR1_Output` writer"]
pub type W = crate::W<TIMX_CCMR1_OUTPUTrs>;
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection"]
pub type CC1S_R = crate::FieldReader;
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection"]
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC1FE` reader - Output compare 1 fast enable"]
pub type OC1FE_R = crate::BitReader;
#[doc = "Field `OC1FE` writer - Output compare 1 fast enable"]
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - Output compare 1 preload enable"]
pub type OC1PE_R = crate::BitReader;
#[doc = "Field `OC1PE` writer - Output compare 1 preload enable"]
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M` reader - Output compare 1 mode"]
pub type OC1M_R = crate::FieldReader;
#[doc = "Field `OC1M` writer - Output compare 1 mode"]
pub type OC1M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC1CE` reader - Output compare 1 clear enable"]
pub type OC1CE_R = crate::BitReader;
#[doc = "Field `OC1CE` writer - Output compare 1 clear enable"]
pub type OC1CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2S` reader - Capture/Compare 2 selection"]
pub type CC2S_R = crate::FieldReader;
#[doc = "Field `CC2S` writer - Capture/Compare 2 selection"]
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC2FE` reader - Output compare 2 fast enable"]
pub type OC2FE_R = crate::BitReader;
#[doc = "Field `OC2FE` writer - Output compare 2 fast enable"]
pub type OC2FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2PE` reader - Output compare 2 preload enable"]
pub type OC2PE_R = crate::BitReader;
#[doc = "Field `OC2PE` writer - Output compare 2 preload enable"]
pub type OC2PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2M` reader - Output compare 2 mode"]
pub type OC2M_R = crate::FieldReader;
#[doc = "Field `OC2M` writer - Output compare 2 mode"]
pub type OC2M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC2CE` reader - Output compare 2 clear enable"]
pub type OC2CE_R = crate::BitReader;
#[doc = "Field `OC2CE` writer - Output compare 2 clear enable"]
pub type OC2CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M_3` reader - Output Compare 1 mode - bit 3"]
pub type OC1M_3_R = crate::BitReader;
#[doc = "Field `OC1M_3` writer - Output Compare 1 mode - bit 3"]
pub type OC1M_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2M_3` reader - Output Compare 2 mode - bit 3"]
pub type OC2M_3_R = crate::BitReader;
#[doc = "Field `OC2M_3` writer - Output Compare 2 mode - bit 3"]
pub type OC2M_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode"]
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 1 clear enable"]
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
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
    #[doc = "Bits 12:14 - Output compare 2 mode"]
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 2 clear enable"]
    #[inline(always)]
    pub fn oc2ce(&self) -> OC2CE_R {
        OC2CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Compare 1 mode - bit 3"]
    #[inline(always)]
    pub fn oc1m_3(&self) -> OC1M_3_R {
        OC1M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output Compare 2 mode - bit 3"]
    #[inline(always)]
    pub fn oc2m_3(&self) -> OC2M_3_R {
        OC2M_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<TIMX_CCMR1_OUTPUTrs> {
        CC1S_W::new(self, 0)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<TIMX_CCMR1_OUTPUTrs> {
        OC1FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<TIMX_CCMR1_OUTPUTrs> {
        OC1PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc1m(&mut self) -> OC1M_W<TIMX_CCMR1_OUTPUTrs> {
        OC1M_W::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 1 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc1ce(&mut self) -> OC1CE_W<TIMX_CCMR1_OUTPUTrs> {
        OC1CE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<TIMX_CCMR1_OUTPUTrs> {
        CC2S_W::new(self, 8)
    }
    #[doc = "Bit 10 - Output compare 2 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2fe(&mut self) -> OC2FE_W<TIMX_CCMR1_OUTPUTrs> {
        OC2FE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 2 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2pe(&mut self) -> OC2PE_W<TIMX_CCMR1_OUTPUTrs> {
        OC2PE_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output compare 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m(&mut self) -> OC2M_W<TIMX_CCMR1_OUTPUTrs> {
        OC2M_W::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 2 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc2ce(&mut self) -> OC2CE_W<TIMX_CCMR1_OUTPUTrs> {
        OC2CE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Output Compare 1 mode - bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc1m_3(&mut self) -> OC1M_3_W<TIMX_CCMR1_OUTPUTrs> {
        OC1M_3_W::new(self, 16)
    }
    #[doc = "Bit 24 - Output Compare 2 mode - bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc2m_3(&mut self) -> OC2M_3_W<TIMX_CCMR1_OUTPUTrs> {
        OC2M_3_W::new(self, 24)
    }
}
#[doc = "capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_ccmr1_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_ccmr1_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMX_CCMR1_OUTPUTrs;
impl crate::RegisterSpec for TIMX_CCMR1_OUTPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timx_ccmr1_output::R`](R) reader structure"]
impl crate::Readable for TIMX_CCMR1_OUTPUTrs {}
#[doc = "`write(|w| ..)` method takes [`timx_ccmr1_output::W`](W) writer structure"]
impl crate::Writable for TIMX_CCMR1_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMx_CCMR1_Output to value 0"]
impl crate::Resettable for TIMX_CCMR1_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
