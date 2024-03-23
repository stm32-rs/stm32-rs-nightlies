#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `B0OIE` reader - Buffer 0 overflow interrupt enable"]
pub type B0OIE_R = crate::BitReader;
#[doc = "Field `B0OIE` writer - Buffer 0 overflow interrupt enable"]
pub type B0OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B1OIE` reader - Buffer 1 overflow interrupt enable"]
pub type B1OIE_R = crate::BitReader;
#[doc = "Field `B1OIE` writer - Buffer 1 overflow interrupt enable"]
pub type B1OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2OIE` reader - Buffer 2 overflow interrupt enable"]
pub type B2OIE_R = crate::BitReader;
#[doc = "Field `B2OIE` writer - Buffer 2 overflow interrupt enable"]
pub type B2OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B3OIE` reader - Buffer 3 overflow interrupt enable"]
pub type B3OIE_R = crate::BitReader;
#[doc = "Field `B3OIE` writer - Buffer 3 overflow interrupt enable"]
pub type B3OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMEIE` reader - AHB master error interrupt enable"]
pub type AMEIE_R = crate::BitReader;
#[doc = "Field `AMEIE` writer - AHB master error interrupt enable"]
pub type AMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BM192` reader - 192 Block mode"]
pub type BM192_R = crate::BitReader;
#[doc = "Field `BM192` writer - 192 Block mode"]
pub type BM192_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Buffer 0 overflow interrupt enable"]
    #[inline(always)]
    pub fn b0oie(&self) -> B0OIE_R {
        B0OIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer 1 overflow interrupt enable"]
    #[inline(always)]
    pub fn b1oie(&self) -> B1OIE_R {
        B1OIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer 2 overflow interrupt enable"]
    #[inline(always)]
    pub fn b2oie(&self) -> B2OIE_R {
        B2OIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Buffer 3 overflow interrupt enable"]
    #[inline(always)]
    pub fn b3oie(&self) -> B3OIE_R {
        B3OIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB master error interrupt enable"]
    #[inline(always)]
    pub fn ameie(&self) -> AMEIE_R {
        AMEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - 192 Block mode"]
    #[inline(always)]
    pub fn bm192(&self) -> BM192_R {
        BM192_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer 0 overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn b0oie(&mut self) -> B0OIE_W<CRrs> {
        B0OIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Buffer 1 overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn b1oie(&mut self) -> B1OIE_W<CRrs> {
        B1OIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Buffer 2 overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn b2oie(&mut self) -> B2OIE_W<CRrs> {
        B2OIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Buffer 3 overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn b3oie(&mut self) -> B3OIE_W<CRrs> {
        B3OIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - AHB master error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ameie(&mut self) -> AMEIE_W<CRrs> {
        AMEIE_W::new(self, 4)
    }
    #[doc = "Bit 6 - 192 Block mode"]
    #[inline(always)]
    #[must_use]
    pub fn bm192(&mut self) -> BM192_W<CRrs> {
        BM192_W::new(self, 6)
    }
}
#[doc = "Graphic MMU configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
