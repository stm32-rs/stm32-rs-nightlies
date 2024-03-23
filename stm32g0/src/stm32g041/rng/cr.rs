#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `RNGEN` reader - Random number generator enable"]
pub type RNGEN_R = crate::BitReader;
#[doc = "Field `RNGEN` writer - Random number generator enable"]
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - Interrupt enable"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - Interrupt enable"]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CED` reader - Clock error detection"]
pub type CED_R = crate::BitReader;
#[doc = "Field `CED` writer - Clock error detection"]
pub type CED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYP` reader - Bypass mode enable"]
pub type BYP_R = crate::BitReader;
#[doc = "Field `BYP` writer - Bypass mode enable"]
pub type BYP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Random number generator enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error detection"]
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bypass mode enable"]
    #[inline(always)]
    pub fn byp(&self) -> BYP_R {
        BYP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Random number generator enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<CRrs> {
        RNGEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CRrs> {
        IE_W::new(self, 3)
    }
    #[doc = "Bit 5 - Clock error detection"]
    #[inline(always)]
    #[must_use]
    pub fn ced(&mut self) -> CED_W<CRrs> {
        CED_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bypass mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn byp(&mut self) -> BYP_W<CRrs> {
        BYP_W::new(self, 6)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
