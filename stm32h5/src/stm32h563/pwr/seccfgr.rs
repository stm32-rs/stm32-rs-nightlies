///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `WUP1SEC` reader - WUPx secure protection
pub type WUP1SEC_R = crate::BitReader;
///Field `WUP1SEC` writer - WUPx secure protection
pub type WUP1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP2SEC` reader - WUPx secure protection
pub type WUP2SEC_R = crate::BitReader;
///Field `WUP2SEC` writer - WUPx secure protection
pub type WUP2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP3SEC` reader - WUPx secure protection
pub type WUP3SEC_R = crate::BitReader;
///Field `WUP3SEC` writer - WUPx secure protection
pub type WUP3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP4SEC` reader - WUPx secure protection
pub type WUP4SEC_R = crate::BitReader;
///Field `WUP4SEC` writer - WUPx secure protection
pub type WUP4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP5SEC` reader - WUPx secure protection
pub type WUP5SEC_R = crate::BitReader;
///Field `WUP5SEC` writer - WUPx secure protection
pub type WUP5SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP6SEC` reader - WUPx secure protection
pub type WUP6SEC_R = crate::BitReader;
///Field `WUP6SEC` writer - WUPx secure protection
pub type WUP6SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP7SEC` reader - WUPx secure protection
pub type WUP7SEC_R = crate::BitReader;
///Field `WUP7SEC` writer - WUPx secure protection
pub type WUP7SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP8SEC` reader - WUPx secure protection
pub type WUP8SEC_R = crate::BitReader;
///Field `WUP8SEC` writer - WUPx secure protection
pub type WUP8SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RETSEC` reader - retention secure protection
pub type RETSEC_R = crate::BitReader;
///Field `RETSEC` writer - retention secure protection
pub type RETSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPMSEC` reader - low-power modes secure protection
pub type LPMSEC_R = crate::BitReader;
///Field `LPMSEC` writer - low-power modes secure protection
pub type LPMSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCMSEC` reader - supply configuration and monitoring secure protection.
pub type SCMSEC_R = crate::BitReader;
///Field `SCMSEC` writer - supply configuration and monitoring secure protection.
pub type SCMSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBSEC` reader - backup domain secure protection
pub type VBSEC_R = crate::BitReader;
///Field `VBSEC` writer - backup domain secure protection
pub type VBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VUSBSEC` reader - voltage USB secure protection
pub type VUSBSEC_R = crate::BitReader;
///Field `VUSBSEC` writer - voltage USB secure protection
pub type VUSBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - WUPx secure protection
    #[inline(always)]
    pub fn wup1sec(&self) -> WUP1SEC_R {
        WUP1SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WUPx secure protection
    #[inline(always)]
    pub fn wup2sec(&self) -> WUP2SEC_R {
        WUP2SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WUPx secure protection
    #[inline(always)]
    pub fn wup3sec(&self) -> WUP3SEC_R {
        WUP3SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WUPx secure protection
    #[inline(always)]
    pub fn wup4sec(&self) -> WUP4SEC_R {
        WUP4SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WUPx secure protection
    #[inline(always)]
    pub fn wup5sec(&self) -> WUP5SEC_R {
        WUP5SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - WUPx secure protection
    #[inline(always)]
    pub fn wup6sec(&self) -> WUP6SEC_R {
        WUP6SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - WUPx secure protection
    #[inline(always)]
    pub fn wup7sec(&self) -> WUP7SEC_R {
        WUP7SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - WUPx secure protection
    #[inline(always)]
    pub fn wup8sec(&self) -> WUP8SEC_R {
        WUP8SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - retention secure protection
    #[inline(always)]
    pub fn retsec(&self) -> RETSEC_R {
        RETSEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - low-power modes secure protection
    #[inline(always)]
    pub fn lpmsec(&self) -> LPMSEC_R {
        LPMSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - supply configuration and monitoring secure protection.
    #[inline(always)]
    pub fn scmsec(&self) -> SCMSEC_R {
        SCMSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - backup domain secure protection
    #[inline(always)]
    pub fn vbsec(&self) -> VBSEC_R {
        VBSEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - voltage USB secure protection
    #[inline(always)]
    pub fn vusbsec(&self) -> VUSBSEC_R {
        VUSBSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("wup1sec", &self.wup1sec())
            .field("wup2sec", &self.wup2sec())
            .field("wup3sec", &self.wup3sec())
            .field("wup4sec", &self.wup4sec())
            .field("wup5sec", &self.wup5sec())
            .field("wup6sec", &self.wup6sec())
            .field("wup7sec", &self.wup7sec())
            .field("wup8sec", &self.wup8sec())
            .field("retsec", &self.retsec())
            .field("lpmsec", &self.lpmsec())
            .field("scmsec", &self.scmsec())
            .field("vbsec", &self.vbsec())
            .field("vusbsec", &self.vusbsec())
            .finish()
    }
}
impl W {
    ///Bit 0 - WUPx secure protection
    #[inline(always)]
    pub fn wup1sec(&mut self) -> WUP1SEC_W<'_, SECCFGRrs> {
        WUP1SEC_W::new(self, 0)
    }
    ///Bit 1 - WUPx secure protection
    #[inline(always)]
    pub fn wup2sec(&mut self) -> WUP2SEC_W<'_, SECCFGRrs> {
        WUP2SEC_W::new(self, 1)
    }
    ///Bit 2 - WUPx secure protection
    #[inline(always)]
    pub fn wup3sec(&mut self) -> WUP3SEC_W<'_, SECCFGRrs> {
        WUP3SEC_W::new(self, 2)
    }
    ///Bit 3 - WUPx secure protection
    #[inline(always)]
    pub fn wup4sec(&mut self) -> WUP4SEC_W<'_, SECCFGRrs> {
        WUP4SEC_W::new(self, 3)
    }
    ///Bit 4 - WUPx secure protection
    #[inline(always)]
    pub fn wup5sec(&mut self) -> WUP5SEC_W<'_, SECCFGRrs> {
        WUP5SEC_W::new(self, 4)
    }
    ///Bit 5 - WUPx secure protection
    #[inline(always)]
    pub fn wup6sec(&mut self) -> WUP6SEC_W<'_, SECCFGRrs> {
        WUP6SEC_W::new(self, 5)
    }
    ///Bit 6 - WUPx secure protection
    #[inline(always)]
    pub fn wup7sec(&mut self) -> WUP7SEC_W<'_, SECCFGRrs> {
        WUP7SEC_W::new(self, 6)
    }
    ///Bit 7 - WUPx secure protection
    #[inline(always)]
    pub fn wup8sec(&mut self) -> WUP8SEC_W<'_, SECCFGRrs> {
        WUP8SEC_W::new(self, 7)
    }
    ///Bit 11 - retention secure protection
    #[inline(always)]
    pub fn retsec(&mut self) -> RETSEC_W<'_, SECCFGRrs> {
        RETSEC_W::new(self, 11)
    }
    ///Bit 12 - low-power modes secure protection
    #[inline(always)]
    pub fn lpmsec(&mut self) -> LPMSEC_W<'_, SECCFGRrs> {
        LPMSEC_W::new(self, 12)
    }
    ///Bit 13 - supply configuration and monitoring secure protection.
    #[inline(always)]
    pub fn scmsec(&mut self) -> SCMSEC_W<'_, SECCFGRrs> {
        SCMSEC_W::new(self, 13)
    }
    ///Bit 14 - backup domain secure protection
    #[inline(always)]
    pub fn vbsec(&mut self) -> VBSEC_W<'_, SECCFGRrs> {
        VBSEC_W::new(self, 14)
    }
    ///Bit 15 - voltage USB secure protection
    #[inline(always)]
    pub fn vusbsec(&mut self) -> VUSBSEC_W<'_, SECCFGRrs> {
        VUSBSEC_W::new(self, 15)
    }
}
/**PWR security configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#PWR:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr::R`](R) reader structure
impl crate::Readable for SECCFGRrs {}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0
impl crate::Resettable for SECCFGRrs {}
