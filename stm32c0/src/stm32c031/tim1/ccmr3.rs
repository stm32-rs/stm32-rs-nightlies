#[doc = "Register `CCMR3` reader"]
pub type R = crate::R<CCMR3rs>;
#[doc = "Register `CCMR3` writer"]
pub type W = crate::W<CCMR3rs>;
#[doc = "Field `OC5FE` reader - Output compare 5 fast enable Refer to OC1FE description."]
pub type OC5FE_R = crate::BitReader;
#[doc = "Field `OC5FE` writer - Output compare 5 fast enable Refer to OC1FE description."]
pub type OC5FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5PE` reader - Output compare 5 preload enable Refer to OC1PE description."]
pub type OC5PE_R = crate::BitReader;
#[doc = "Field `OC5PE` writer - Output compare 5 preload enable Refer to OC1PE description."]
pub type OC5PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M1` reader - Output compare 5 mode Refer to OC1M description."]
pub type OC5M1_R = crate::FieldReader;
#[doc = "Field `OC5M1` writer - Output compare 5 mode Refer to OC1M description."]
pub type OC5M1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC5CE` reader - Output compare 5 clear enable Refer to OC1CE description."]
pub type OC5CE_R = crate::BitReader;
#[doc = "Field `OC5CE` writer - Output compare 5 clear enable Refer to OC1CE description."]
pub type OC5CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6FE` reader - Output compare 6 fast enable Refer to OC1FE description."]
pub type OC6FE_R = crate::BitReader;
#[doc = "Field `OC6FE` writer - Output compare 6 fast enable Refer to OC1FE description."]
pub type OC6FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6PE` reader - Output compare 6 preload enable Refer to OC1PE description."]
pub type OC6PE_R = crate::BitReader;
#[doc = "Field `OC6PE` writer - Output compare 6 preload enable Refer to OC1PE description."]
pub type OC6PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6M1` reader - Output compare 6 mode Refer to OC1M description."]
pub type OC6M1_R = crate::FieldReader;
#[doc = "Field `OC6M1` writer - Output compare 6 mode Refer to OC1M description."]
pub type OC6M1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC6CE` reader - Output compare 6 clear enable Refer to OC1CE description."]
pub type OC6CE_R = crate::BitReader;
#[doc = "Field `OC6CE` writer - Output compare 6 clear enable Refer to OC1CE description."]
pub type OC6CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M2` reader - Output compare 5 mode Refer to OC1M description."]
pub type OC5M2_R = crate::BitReader;
#[doc = "Field `OC5M2` writer - Output compare 5 mode Refer to OC1M description."]
pub type OC5M2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6M2` reader - Output compare 6 mode Refer to OC1M description."]
pub type OC6M2_R = crate::BitReader;
#[doc = "Field `OC6M2` writer - Output compare 6 mode Refer to OC1M description."]
pub type OC6M2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Output compare 5 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc5fe(&self) -> OC5FE_R {
        OC5FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc5pe(&self) -> OC5PE_R {
        OC5PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 5 mode Refer to OC1M description."]
    #[inline(always)]
    pub fn oc5m1(&self) -> OC5M1_R {
        OC5M1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 5 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc5ce(&self) -> OC5CE_R {
        OC5CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc6fe(&self) -> OC6FE_R {
        OC6FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc6pe(&self) -> OC6PE_R {
        OC6PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 6 mode Refer to OC1M description."]
    #[inline(always)]
    pub fn oc6m1(&self) -> OC6M1_R {
        OC6M1_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc6ce(&self) -> OC6CE_R {
        OC6CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output compare 5 mode Refer to OC1M description."]
    #[inline(always)]
    pub fn oc5m2(&self) -> OC5M2_R {
        OC5M2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output compare 6 mode Refer to OC1M description."]
    #[inline(always)]
    pub fn oc6m2(&self) -> OC6M2_R {
        OC6M2_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Output compare 5 fast enable Refer to OC1FE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc5fe(&mut self) -> OC5FE_W<CCMR3rs> {
        OC5FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable Refer to OC1PE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc5pe(&mut self) -> OC5PE_W<CCMR3rs> {
        OC5PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 5 mode Refer to OC1M description."]
    #[inline(always)]
    #[must_use]
    pub fn oc5m1(&mut self) -> OC5M1_W<CCMR3rs> {
        OC5M1_W::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 5 clear enable Refer to OC1CE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc5ce(&mut self) -> OC5CE_W<CCMR3rs> {
        OC5CE_W::new(self, 7)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable Refer to OC1FE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc6fe(&mut self) -> OC6FE_W<CCMR3rs> {
        OC6FE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable Refer to OC1PE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc6pe(&mut self) -> OC6PE_W<CCMR3rs> {
        OC6PE_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output compare 6 mode Refer to OC1M description."]
    #[inline(always)]
    #[must_use]
    pub fn oc6m1(&mut self) -> OC6M1_W<CCMR3rs> {
        OC6M1_W::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable Refer to OC1CE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc6ce(&mut self) -> OC6CE_W<CCMR3rs> {
        OC6CE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Output compare 5 mode Refer to OC1M description."]
    #[inline(always)]
    #[must_use]
    pub fn oc5m2(&mut self) -> OC5M2_W<CCMR3rs> {
        OC5M2_W::new(self, 16)
    }
    #[doc = "Bit 24 - Output compare 6 mode Refer to OC1M description."]
    #[inline(always)]
    #[must_use]
    pub fn oc6m2(&mut self) -> OC6M2_W<CCMR3rs> {
        OC6M2_W::new(self, 24)
    }
}
#[doc = "TIM1 capture/compare mode register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR3rs;
impl crate::RegisterSpec for CCMR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr3::R`](R) reader structure"]
impl crate::Readable for CCMR3rs {}
#[doc = "`write(|w| ..)` method takes [`ccmr3::W`](W) writer structure"]
impl crate::Writable for CCMR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR3 to value 0"]
impl crate::Resettable for CCMR3rs {
    const RESET_VALUE: u32 = 0;
}
