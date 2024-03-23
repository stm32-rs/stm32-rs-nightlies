#[doc = "Register `SECCFGR` reader"]
pub type R = crate::R<SECCFGRrs>;
#[doc = "Register `SECCFGR` writer"]
pub type W = crate::W<SECCFGRrs>;
#[doc = "Field `WUP1SEC` reader - WKUP1 pin security"]
pub type WUP1SEC_R = crate::BitReader;
#[doc = "Field `WUP1SEC` writer - WKUP1 pin security"]
pub type WUP1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP2SEC` reader - WKUP2 pin security"]
pub type WUP2SEC_R = crate::BitReader;
#[doc = "Field `WUP2SEC` writer - WKUP2 pin security"]
pub type WUP2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP3SEC` reader - WKUP3 pin security"]
pub type WUP3SEC_R = crate::BitReader;
#[doc = "Field `WUP3SEC` writer - WKUP3 pin security"]
pub type WUP3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP4SEC` reader - WKUP4 pin security"]
pub type WUP4SEC_R = crate::BitReader;
#[doc = "Field `WUP4SEC` writer - WKUP4 pin security"]
pub type WUP4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP5SEC` reader - WKUP5 pin security"]
pub type WUP5SEC_R = crate::BitReader;
#[doc = "Field `WUP5SEC` writer - WKUP5 pin security"]
pub type WUP5SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMSEC` reader - LPMSEC"]
pub type LPMSEC_R = crate::BitReader;
#[doc = "Field `LPMSEC` writer - LPMSEC"]
pub type LPMSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMSEC` reader - VDMSEC"]
pub type VDMSEC_R = crate::BitReader;
#[doc = "Field `VDMSEC` writer - VDMSEC"]
pub type VDMSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBSEC` reader - VBSEC"]
pub type VBSEC_R = crate::BitReader;
#[doc = "Field `VBSEC` writer - VBSEC"]
pub type VBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APCSEC` reader - APCSEC"]
pub type APCSEC_R = crate::BitReader;
#[doc = "Field `APCSEC` writer - APCSEC"]
pub type APCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WKUP1 pin security"]
    #[inline(always)]
    pub fn wup1sec(&self) -> WUP1SEC_R {
        WUP1SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WKUP2 pin security"]
    #[inline(always)]
    pub fn wup2sec(&self) -> WUP2SEC_R {
        WUP2SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WKUP3 pin security"]
    #[inline(always)]
    pub fn wup3sec(&self) -> WUP3SEC_R {
        WUP3SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUP4 pin security"]
    #[inline(always)]
    pub fn wup4sec(&self) -> WUP4SEC_R {
        WUP4SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WKUP5 pin security"]
    #[inline(always)]
    pub fn wup5sec(&self) -> WUP5SEC_R {
        WUP5SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - LPMSEC"]
    #[inline(always)]
    pub fn lpmsec(&self) -> LPMSEC_R {
        LPMSEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VDMSEC"]
    #[inline(always)]
    pub fn vdmsec(&self) -> VDMSEC_R {
        VDMSEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VBSEC"]
    #[inline(always)]
    pub fn vbsec(&self) -> VBSEC_R {
        VBSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - APCSEC"]
    #[inline(always)]
    pub fn apcsec(&self) -> APCSEC_R {
        APCSEC_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WKUP1 pin security"]
    #[inline(always)]
    #[must_use]
    pub fn wup1sec(&mut self) -> WUP1SEC_W<SECCFGRrs> {
        WUP1SEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - WKUP2 pin security"]
    #[inline(always)]
    #[must_use]
    pub fn wup2sec(&mut self) -> WUP2SEC_W<SECCFGRrs> {
        WUP2SEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - WKUP3 pin security"]
    #[inline(always)]
    #[must_use]
    pub fn wup3sec(&mut self) -> WUP3SEC_W<SECCFGRrs> {
        WUP3SEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - WKUP4 pin security"]
    #[inline(always)]
    #[must_use]
    pub fn wup4sec(&mut self) -> WUP4SEC_W<SECCFGRrs> {
        WUP4SEC_W::new(self, 3)
    }
    #[doc = "Bit 4 - WKUP5 pin security"]
    #[inline(always)]
    #[must_use]
    pub fn wup5sec(&mut self) -> WUP5SEC_W<SECCFGRrs> {
        WUP5SEC_W::new(self, 4)
    }
    #[doc = "Bit 8 - LPMSEC"]
    #[inline(always)]
    #[must_use]
    pub fn lpmsec(&mut self) -> LPMSEC_W<SECCFGRrs> {
        LPMSEC_W::new(self, 8)
    }
    #[doc = "Bit 9 - VDMSEC"]
    #[inline(always)]
    #[must_use]
    pub fn vdmsec(&mut self) -> VDMSEC_W<SECCFGRrs> {
        VDMSEC_W::new(self, 9)
    }
    #[doc = "Bit 10 - VBSEC"]
    #[inline(always)]
    #[must_use]
    pub fn vbsec(&mut self) -> VBSEC_W<SECCFGRrs> {
        VBSEC_W::new(self, 10)
    }
    #[doc = "Bit 11 - APCSEC"]
    #[inline(always)]
    #[must_use]
    pub fn apcsec(&mut self) -> APCSEC_W<SECCFGRrs> {
        APCSEC_W::new(self, 11)
    }
}
#[doc = "Power secure configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
