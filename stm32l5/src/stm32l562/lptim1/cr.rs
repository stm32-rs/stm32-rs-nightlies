#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `ENABLE` reader - LPTIM Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - LPTIM Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNGSTRT` reader - LPTIM start in single mode"]
pub type SNGSTRT_R = crate::BitReader;
#[doc = "Field `SNGSTRT` writer - LPTIM start in single mode"]
pub type SNGSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTSTRT` reader - Timer start in continuous mode"]
pub type CNTSTRT_R = crate::BitReader;
#[doc = "Field `CNTSTRT` writer - Timer start in continuous mode"]
pub type CNTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTARE` reader - Reset after read enable"]
pub type RSTARE_R = crate::BitReader;
#[doc = "Field `RSTARE` writer - Reset after read enable"]
pub type RSTARE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTRST` reader - Counter reset"]
pub type COUNTRST_R = crate::BitReader;
#[doc = "Field `COUNTRST` writer - Counter reset"]
pub type COUNTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPTIM Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPTIM start in single mode"]
    #[inline(always)]
    pub fn sngstrt(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer start in continuous mode"]
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset after read enable"]
    #[inline(always)]
    pub fn rstare(&self) -> RSTARE_R {
        RSTARE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counter reset"]
    #[inline(always)]
    pub fn countrst(&self) -> COUNTRST_R {
        COUNTRST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CRrs> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - LPTIM start in single mode"]
    #[inline(always)]
    #[must_use]
    pub fn sngstrt(&mut self) -> SNGSTRT_W<CRrs> {
        SNGSTRT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer start in continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn cntstrt(&mut self) -> CNTSTRT_W<CRrs> {
        CNTSTRT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reset after read enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstare(&mut self) -> RSTARE_W<CRrs> {
        RSTARE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn countrst(&mut self) -> COUNTRST_W<CRrs> {
        COUNTRST_W::new(self, 4)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
