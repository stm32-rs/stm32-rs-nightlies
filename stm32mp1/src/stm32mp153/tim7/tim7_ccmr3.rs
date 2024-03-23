#[doc = "Register `TIM7_CCMR3` reader"]
pub type R = crate::R<TIM7_CCMR3rs>;
#[doc = "Register `TIM7_CCMR3` writer"]
pub type W = crate::W<TIM7_CCMR3rs>;
#[doc = "Field `OC5FE` reader - OC5FE"]
pub type OC5FE_R = crate::BitReader;
#[doc = "Field `OC5FE` writer - OC5FE"]
pub type OC5FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5PE` reader - OC5PE"]
pub type OC5PE_R = crate::BitReader;
#[doc = "Field `OC5PE` writer - OC5PE"]
pub type OC5PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M` reader - OC5M"]
pub type OC5M_R = crate::FieldReader;
#[doc = "Field `OC5M` writer - OC5M"]
pub type OC5M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC5CE` reader - OC5CE"]
pub type OC5CE_R = crate::BitReader;
#[doc = "Field `OC5CE` writer - OC5CE"]
pub type OC5CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6FE` reader - OC6FE"]
pub type OC6FE_R = crate::BitReader;
#[doc = "Field `OC6FE` writer - OC6FE"]
pub type OC6FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6PE` reader - OC6PE"]
pub type OC6PE_R = crate::BitReader;
#[doc = "Field `OC6PE` writer - OC6PE"]
pub type OC6PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6M` reader - OC6M"]
pub type OC6M_R = crate::FieldReader;
#[doc = "Field `OC6M` writer - OC6M"]
pub type OC6M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC6CE` reader - OC6CE"]
pub type OC6CE_R = crate::BitReader;
#[doc = "Field `OC6CE` writer - OC6CE"]
pub type OC6CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M3` reader - OC5M3"]
pub type OC5M3_R = crate::BitReader;
#[doc = "Field `OC5M3` writer - OC5M3"]
pub type OC5M3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6M3` reader - OC6M3"]
pub type OC6M3_R = crate::BitReader;
#[doc = "Field `OC6M3` writer - OC6M3"]
pub type OC6M3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - OC5FE"]
    #[inline(always)]
    pub fn oc5fe(&self) -> OC5FE_R {
        OC5FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OC5PE"]
    #[inline(always)]
    pub fn oc5pe(&self) -> OC5PE_R {
        OC5PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC5M"]
    #[inline(always)]
    pub fn oc5m(&self) -> OC5M_R {
        OC5M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - OC5CE"]
    #[inline(always)]
    pub fn oc5ce(&self) -> OC5CE_R {
        OC5CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - OC6FE"]
    #[inline(always)]
    pub fn oc6fe(&self) -> OC6FE_R {
        OC6FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OC6PE"]
    #[inline(always)]
    pub fn oc6pe(&self) -> OC6PE_R {
        OC6PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - OC6M"]
    #[inline(always)]
    pub fn oc6m(&self) -> OC6M_R {
        OC6M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - OC6CE"]
    #[inline(always)]
    pub fn oc6ce(&self) -> OC6CE_R {
        OC6CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OC5M3"]
    #[inline(always)]
    pub fn oc5m3(&self) -> OC5M3_R {
        OC5M3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - OC6M3"]
    #[inline(always)]
    pub fn oc6m3(&self) -> OC6M3_R {
        OC6M3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - OC5FE"]
    #[inline(always)]
    #[must_use]
    pub fn oc5fe(&mut self) -> OC5FE_W<TIM7_CCMR3rs> {
        OC5FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - OC5PE"]
    #[inline(always)]
    #[must_use]
    pub fn oc5pe(&mut self) -> OC5PE_W<TIM7_CCMR3rs> {
        OC5PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - OC5M"]
    #[inline(always)]
    #[must_use]
    pub fn oc5m(&mut self) -> OC5M_W<TIM7_CCMR3rs> {
        OC5M_W::new(self, 4)
    }
    #[doc = "Bit 7 - OC5CE"]
    #[inline(always)]
    #[must_use]
    pub fn oc5ce(&mut self) -> OC5CE_W<TIM7_CCMR3rs> {
        OC5CE_W::new(self, 7)
    }
    #[doc = "Bit 10 - OC6FE"]
    #[inline(always)]
    #[must_use]
    pub fn oc6fe(&mut self) -> OC6FE_W<TIM7_CCMR3rs> {
        OC6FE_W::new(self, 10)
    }
    #[doc = "Bit 11 - OC6PE"]
    #[inline(always)]
    #[must_use]
    pub fn oc6pe(&mut self) -> OC6PE_W<TIM7_CCMR3rs> {
        OC6PE_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - OC6M"]
    #[inline(always)]
    #[must_use]
    pub fn oc6m(&mut self) -> OC6M_W<TIM7_CCMR3rs> {
        OC6M_W::new(self, 12)
    }
    #[doc = "Bit 15 - OC6CE"]
    #[inline(always)]
    #[must_use]
    pub fn oc6ce(&mut self) -> OC6CE_W<TIM7_CCMR3rs> {
        OC6CE_W::new(self, 15)
    }
    #[doc = "Bit 16 - OC5M3"]
    #[inline(always)]
    #[must_use]
    pub fn oc5m3(&mut self) -> OC5M3_W<TIM7_CCMR3rs> {
        OC5M3_W::new(self, 16)
    }
    #[doc = "Bit 24 - OC6M3"]
    #[inline(always)]
    #[must_use]
    pub fn oc6m3(&mut self) -> OC6M3_W<TIM7_CCMR3rs> {
        OC6M3_W::new(self, 24)
    }
}
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim7_ccmr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim7_ccmr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM7_CCMR3rs;
impl crate::RegisterSpec for TIM7_CCMR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim7_ccmr3::R`](R) reader structure"]
impl crate::Readable for TIM7_CCMR3rs {}
#[doc = "`write(|w| ..)` method takes [`tim7_ccmr3::W`](W) writer structure"]
impl crate::Writable for TIM7_CCMR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM7_CCMR3 to value 0"]
impl crate::Resettable for TIM7_CCMR3rs {
    const RESET_VALUE: u32 = 0;
}
