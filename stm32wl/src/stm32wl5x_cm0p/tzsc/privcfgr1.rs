#[doc = "Register `PRIVCFGR1` reader"]
pub type R = crate::R<PRIVCFGR1rs>;
#[doc = "Register `PRIVCFGR1` writer"]
pub type W = crate::W<PRIVCFGR1rs>;
#[doc = "Field `AESPRIV` reader - AESPRIV"]
pub type AESPRIV_R = crate::BitReader;
#[doc = "Field `AESPRIV` writer - AESPRIV"]
pub type AESPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGPRIV` reader - RNGPRIV"]
pub type RNGPRIV_R = crate::BitReader;
#[doc = "Field `RNGPRIV` writer - RNGPRIV"]
pub type RNGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUBGHZSPIPRIV` reader - SUBGHZSPIPRIV"]
pub type SUBGHZSPIPRIV_R = crate::BitReader;
#[doc = "Field `SUBGHZSPIPRIV` writer - SUBGHZSPIPRIV"]
pub type SUBGHZSPIPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKAPRIV` reader - PKAPRIV"]
pub type PKAPRIV_R = crate::BitReader;
#[doc = "Field `PKAPRIV` writer - PKAPRIV"]
pub type PKAPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - AESPRIV"]
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RNGPRIV"]
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SUBGHZSPIPRIV"]
    #[inline(always)]
    pub fn subghzspipriv(&self) -> SUBGHZSPIPRIV_R {
        SUBGHZSPIPRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 13 - PKAPRIV"]
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - AESPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn aespriv(&mut self) -> AESPRIV_W<PRIVCFGR1rs> {
        AESPRIV_W::new(self, 2)
    }
    #[doc = "Bit 3 - RNGPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn rngpriv(&mut self) -> RNGPRIV_W<PRIVCFGR1rs> {
        RNGPRIV_W::new(self, 3)
    }
    #[doc = "Bit 4 - SUBGHZSPIPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn subghzspipriv(&mut self) -> SUBGHZSPIPRIV_W<PRIVCFGR1rs> {
        SUBGHZSPIPRIV_W::new(self, 4)
    }
    #[doc = "Bit 13 - PKAPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn pkapriv(&mut self) -> PKAPRIV_W<PRIVCFGR1rs> {
        PKAPRIV_W::new(self, 13)
    }
}
#[doc = "TZSC privilege configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGR1rs;
impl crate::RegisterSpec for PRIVCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcfgr1::R`](R) reader structure"]
impl crate::Readable for PRIVCFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`privcfgr1::W`](W) writer structure"]
impl crate::Writable for PRIVCFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIVCFGR1 to value 0"]
impl crate::Resettable for PRIVCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
