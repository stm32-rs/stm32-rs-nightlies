#[doc = "Register `SECCFGR` reader"]
pub type R = crate::R<SECCFGRrs>;
#[doc = "Register `SECCFGR` writer"]
pub type W = crate::W<SECCFGRrs>;
#[doc = "Field `ALRASEC` reader - ALRASEC"]
pub type ALRASEC_R = crate::BitReader;
#[doc = "Field `ALRASEC` writer - ALRASEC"]
pub type ALRASEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBSEC` reader - ALRBSEC"]
pub type ALRBSEC_R = crate::BitReader;
#[doc = "Field `ALRBSEC` writer - ALRBSEC"]
pub type ALRBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTSEC` reader - WUTSEC"]
pub type WUTSEC_R = crate::BitReader;
#[doc = "Field `WUTSEC` writer - WUTSEC"]
pub type WUTSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSEC` reader - TSSEC"]
pub type TSSEC_R = crate::BitReader;
#[doc = "Field `TSSEC` writer - TSSEC"]
pub type TSSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSEC` reader - CALSEC"]
pub type CALSEC_R = crate::BitReader;
#[doc = "Field `CALSEC` writer - CALSEC"]
pub type CALSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITSEC` reader - INITSEC"]
pub type INITSEC_R = crate::BitReader;
#[doc = "Field `INITSEC` writer - INITSEC"]
pub type INITSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC` reader - SEC"]
pub type SEC_R = crate::BitReader;
#[doc = "Field `SEC` writer - SEC"]
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ALRASEC"]
    #[inline(always)]
    pub fn alrasec(&self) -> ALRASEC_R {
        ALRASEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ALRBSEC"]
    #[inline(always)]
    pub fn alrbsec(&self) -> ALRBSEC_R {
        ALRBSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUTSEC"]
    #[inline(always)]
    pub fn wutsec(&self) -> WUTSEC_R {
        WUTSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSSEC"]
    #[inline(always)]
    pub fn tssec(&self) -> TSSEC_R {
        TSSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 13 - CALSEC"]
    #[inline(always)]
    pub fn calsec(&self) -> CALSEC_R {
        CALSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - INITSEC"]
    #[inline(always)]
    pub fn initsec(&self) -> INITSEC_R {
        INITSEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SEC"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ALRASEC"]
    #[inline(always)]
    #[must_use]
    pub fn alrasec(&mut self) -> ALRASEC_W<SECCFGRrs> {
        ALRASEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - ALRBSEC"]
    #[inline(always)]
    #[must_use]
    pub fn alrbsec(&mut self) -> ALRBSEC_W<SECCFGRrs> {
        ALRBSEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - WUTSEC"]
    #[inline(always)]
    #[must_use]
    pub fn wutsec(&mut self) -> WUTSEC_W<SECCFGRrs> {
        WUTSEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - TSSEC"]
    #[inline(always)]
    #[must_use]
    pub fn tssec(&mut self) -> TSSEC_W<SECCFGRrs> {
        TSSEC_W::new(self, 3)
    }
    #[doc = "Bit 13 - CALSEC"]
    #[inline(always)]
    #[must_use]
    pub fn calsec(&mut self) -> CALSEC_W<SECCFGRrs> {
        CALSEC_W::new(self, 13)
    }
    #[doc = "Bit 14 - INITSEC"]
    #[inline(always)]
    #[must_use]
    pub fn initsec(&mut self) -> INITSEC_W<SECCFGRrs> {
        INITSEC_W::new(self, 14)
    }
    #[doc = "Bit 15 - SEC"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<SECCFGRrs> {
        SEC_W::new(self, 15)
    }
}
#[doc = "RTC secure mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfgr::R`](R) reader structure"]
impl crate::Readable for SECCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure"]
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0;
}
