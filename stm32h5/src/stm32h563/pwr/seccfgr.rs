#[doc = "Register `SECCFGR` reader"]
pub type R = crate::R<SECCFGRrs>;
#[doc = "Register `SECCFGR` writer"]
pub type W = crate::W<SECCFGRrs>;
#[doc = "Field `WUP1SEC` reader - WUPx secure protection"]
pub type WUP1SEC_R = crate::BitReader;
#[doc = "Field `WUP1SEC` writer - WUPx secure protection"]
pub type WUP1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP2SEC` reader - WUPx secure protection"]
pub type WUP2SEC_R = crate::BitReader;
#[doc = "Field `WUP2SEC` writer - WUPx secure protection"]
pub type WUP2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP3SEC` reader - WUPx secure protection"]
pub type WUP3SEC_R = crate::BitReader;
#[doc = "Field `WUP3SEC` writer - WUPx secure protection"]
pub type WUP3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP4SEC` reader - WUPx secure protection"]
pub type WUP4SEC_R = crate::BitReader;
#[doc = "Field `WUP4SEC` writer - WUPx secure protection"]
pub type WUP4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP5SEC` reader - WUPx secure protection"]
pub type WUP5SEC_R = crate::BitReader;
#[doc = "Field `WUP5SEC` writer - WUPx secure protection"]
pub type WUP5SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP6SEC` reader - WUPx secure protection"]
pub type WUP6SEC_R = crate::BitReader;
#[doc = "Field `WUP6SEC` writer - WUPx secure protection"]
pub type WUP6SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP7SEC` reader - WUPx secure protection"]
pub type WUP7SEC_R = crate::BitReader;
#[doc = "Field `WUP7SEC` writer - WUPx secure protection"]
pub type WUP7SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP8SEC` reader - WUPx secure protection"]
pub type WUP8SEC_R = crate::BitReader;
#[doc = "Field `WUP8SEC` writer - WUPx secure protection"]
pub type WUP8SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETSEC` reader - retention secure protection"]
pub type RETSEC_R = crate::BitReader;
#[doc = "Field `RETSEC` writer - retention secure protection"]
pub type RETSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMSEC` reader - low-power modes secure protection"]
pub type LPMSEC_R = crate::BitReader;
#[doc = "Field `LPMSEC` writer - low-power modes secure protection"]
pub type LPMSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMSEC` reader - supply configuration and monitoring secure protection."]
pub type SCMSEC_R = crate::BitReader;
#[doc = "Field `SCMSEC` writer - supply configuration and monitoring secure protection."]
pub type SCMSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBSEC` reader - backup domain secure protection"]
pub type VBSEC_R = crate::BitReader;
#[doc = "Field `VBSEC` writer - backup domain secure protection"]
pub type VBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUSBSEC` reader - voltage USB secure protection"]
pub type VUSBSEC_R = crate::BitReader;
#[doc = "Field `VUSBSEC` writer - voltage USB secure protection"]
pub type VUSBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup1sec(&self) -> WUP1SEC_R {
        WUP1SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup2sec(&self) -> WUP2SEC_R {
        WUP2SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup3sec(&self) -> WUP3SEC_R {
        WUP3SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup4sec(&self) -> WUP4SEC_R {
        WUP4SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup5sec(&self) -> WUP5SEC_R {
        WUP5SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup6sec(&self) -> WUP6SEC_R {
        WUP6SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup7sec(&self) -> WUP7SEC_R {
        WUP7SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WUPx secure protection"]
    #[inline(always)]
    pub fn wup8sec(&self) -> WUP8SEC_R {
        WUP8SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - retention secure protection"]
    #[inline(always)]
    pub fn retsec(&self) -> RETSEC_R {
        RETSEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - low-power modes secure protection"]
    #[inline(always)]
    pub fn lpmsec(&self) -> LPMSEC_R {
        LPMSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - supply configuration and monitoring secure protection."]
    #[inline(always)]
    pub fn scmsec(&self) -> SCMSEC_R {
        SCMSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - backup domain secure protection"]
    #[inline(always)]
    pub fn vbsec(&self) -> VBSEC_R {
        VBSEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - voltage USB secure protection"]
    #[inline(always)]
    pub fn vusbsec(&self) -> VUSBSEC_R {
        VUSBSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup1sec(&mut self) -> WUP1SEC_W<SECCFGRrs> {
        WUP1SEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup2sec(&mut self) -> WUP2SEC_W<SECCFGRrs> {
        WUP2SEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup3sec(&mut self) -> WUP3SEC_W<SECCFGRrs> {
        WUP3SEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup4sec(&mut self) -> WUP4SEC_W<SECCFGRrs> {
        WUP4SEC_W::new(self, 3)
    }
    #[doc = "Bit 4 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup5sec(&mut self) -> WUP5SEC_W<SECCFGRrs> {
        WUP5SEC_W::new(self, 4)
    }
    #[doc = "Bit 5 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup6sec(&mut self) -> WUP6SEC_W<SECCFGRrs> {
        WUP6SEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup7sec(&mut self) -> WUP7SEC_W<SECCFGRrs> {
        WUP7SEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - WUPx secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn wup8sec(&mut self) -> WUP8SEC_W<SECCFGRrs> {
        WUP8SEC_W::new(self, 7)
    }
    #[doc = "Bit 11 - retention secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn retsec(&mut self) -> RETSEC_W<SECCFGRrs> {
        RETSEC_W::new(self, 11)
    }
    #[doc = "Bit 12 - low-power modes secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn lpmsec(&mut self) -> LPMSEC_W<SECCFGRrs> {
        LPMSEC_W::new(self, 12)
    }
    #[doc = "Bit 13 - supply configuration and monitoring secure protection."]
    #[inline(always)]
    #[must_use]
    pub fn scmsec(&mut self) -> SCMSEC_W<SECCFGRrs> {
        SCMSEC_W::new(self, 13)
    }
    #[doc = "Bit 14 - backup domain secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn vbsec(&mut self) -> VBSEC_W<SECCFGRrs> {
        VBSEC_W::new(self, 14)
    }
    #[doc = "Bit 15 - voltage USB secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn vusbsec(&mut self) -> VUSBSEC_W<SECCFGRrs> {
        VUSBSEC_W::new(self, 15)
    }
}
#[doc = "PWR security configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
