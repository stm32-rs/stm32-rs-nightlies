#[doc = "Register `LPTIM_CR` reader"]
pub type R = crate::R<LPTIM_CRrs>;
#[doc = "Register `LPTIM_CR` writer"]
pub type W = crate::W<LPTIM_CRrs>;
#[doc = "Field `ENABLE` reader - ENABLE"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - ENABLE"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNGSTRT` reader - SNGSTRT"]
pub type SNGSTRT_R = crate::BitReader;
#[doc = "Field `SNGSTRT` writer - SNGSTRT"]
pub type SNGSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTSTRT` reader - CNTSTRT"]
pub type CNTSTRT_R = crate::BitReader;
#[doc = "Field `CNTSTRT` writer - CNTSTRT"]
pub type CNTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTRST` reader - COUNTRST"]
pub type COUNTRST_R = crate::BitReader;
#[doc = "Field `COUNTRST` writer - COUNTRST"]
pub type COUNTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTARE` reader - RSTARE"]
pub type RSTARE_R = crate::BitReader;
#[doc = "Field `RSTARE` writer - RSTARE"]
pub type RSTARE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SNGSTRT"]
    #[inline(always)]
    pub fn sngstrt(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CNTSTRT"]
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COUNTRST"]
    #[inline(always)]
    pub fn countrst(&self) -> COUNTRST_R {
        COUNTRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RSTARE"]
    #[inline(always)]
    pub fn rstare(&self) -> RSTARE_R {
        RSTARE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<LPTIM_CRrs> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - SNGSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn sngstrt(&mut self) -> SNGSTRT_W<LPTIM_CRrs> {
        SNGSTRT_W::new(self, 1)
    }
    #[doc = "Bit 2 - CNTSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn cntstrt(&mut self) -> CNTSTRT_W<LPTIM_CRrs> {
        CNTSTRT_W::new(self, 2)
    }
    #[doc = "Bit 3 - COUNTRST"]
    #[inline(always)]
    #[must_use]
    pub fn countrst(&mut self) -> COUNTRST_W<LPTIM_CRrs> {
        COUNTRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - RSTARE"]
    #[inline(always)]
    #[must_use]
    pub fn rstare(&mut self) -> RSTARE_W<LPTIM_CRrs> {
        RSTARE_W::new(self, 4)
    }
}
#[doc = "LPTIM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPTIM_CRrs;
impl crate::RegisterSpec for LPTIM_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim_cr::R`](R) reader structure"]
impl crate::Readable for LPTIM_CRrs {}
#[doc = "`write(|w| ..)` method takes [`lptim_cr::W`](W) writer structure"]
impl crate::Writable for LPTIM_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM_CR to value 0"]
impl crate::Resettable for LPTIM_CRrs {
    const RESET_VALUE: u32 = 0;
}
