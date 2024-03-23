#[doc = "Register `SECCFGR1` reader"]
pub type R = crate::R<SECCFGR1rs>;
#[doc = "Register `SECCFGR1` writer"]
pub type W = crate::W<SECCFGR1rs>;
#[doc = "Field `AESSEC` reader - AESSEC"]
pub type AESSEC_R = crate::BitReader;
#[doc = "Field `AESSEC` writer - AESSEC"]
pub type AESSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGSEC` reader - RNGSEC"]
pub type RNGSEC_R = crate::BitReader;
#[doc = "Field `RNGSEC` writer - RNGSEC"]
pub type RNGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKASEC` reader - PKASEC"]
pub type PKASEC_R = crate::BitReader;
#[doc = "Field `PKASEC` writer - PKASEC"]
pub type PKASEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - AESSEC"]
    #[inline(always)]
    pub fn aessec(&self) -> AESSEC_R {
        AESSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RNGSEC"]
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 13 - PKASEC"]
    #[inline(always)]
    pub fn pkasec(&self) -> PKASEC_R {
        PKASEC_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - AESSEC"]
    #[inline(always)]
    #[must_use]
    pub fn aessec(&mut self) -> AESSEC_W<SECCFGR1rs> {
        AESSEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - RNGSEC"]
    #[inline(always)]
    #[must_use]
    pub fn rngsec(&mut self) -> RNGSEC_W<SECCFGR1rs> {
        RNGSEC_W::new(self, 3)
    }
    #[doc = "Bit 13 - PKASEC"]
    #[inline(always)]
    #[must_use]
    pub fn pkasec(&mut self) -> PKASEC_W<SECCFGR1rs> {
        PKASEC_W::new(self, 13)
    }
}
#[doc = "TZSC security configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFGR1rs;
impl crate::RegisterSpec for SECCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfgr1::R`](R) reader structure"]
impl crate::Readable for SECCFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`seccfgr1::W`](W) writer structure"]
impl crate::Writable for SECCFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFGR1 to value 0"]
impl crate::Resettable for SECCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
