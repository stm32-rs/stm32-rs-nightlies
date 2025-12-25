///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `WUP1SEC` reader - WKUP1 pin security
pub type WUP1SEC_R = crate::BitReader;
///Field `WUP1SEC` writer - WKUP1 pin security
pub type WUP1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP2SEC` reader - WKUP2 pin security
pub type WUP2SEC_R = crate::BitReader;
///Field `WUP2SEC` writer - WKUP2 pin security
pub type WUP2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP3SEC` reader - WKUP3 pin security
pub type WUP3SEC_R = crate::BitReader;
///Field `WUP3SEC` writer - WKUP3 pin security
pub type WUP3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP4SEC` reader - WKUP4 pin security
pub type WUP4SEC_R = crate::BitReader;
///Field `WUP4SEC` writer - WKUP4 pin security
pub type WUP4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP5SEC` reader - WKUP5 pin security
pub type WUP5SEC_R = crate::BitReader;
///Field `WUP5SEC` writer - WKUP5 pin security
pub type WUP5SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPMSEC` reader - LPMSEC
pub type LPMSEC_R = crate::BitReader;
///Field `LPMSEC` writer - LPMSEC
pub type LPMSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDMSEC` reader - VDMSEC
pub type VDMSEC_R = crate::BitReader;
///Field `VDMSEC` writer - VDMSEC
pub type VDMSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBSEC` reader - VBSEC
pub type VBSEC_R = crate::BitReader;
///Field `VBSEC` writer - VBSEC
pub type VBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APCSEC` reader - APCSEC
pub type APCSEC_R = crate::BitReader;
///Field `APCSEC` writer - APCSEC
pub type APCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - WKUP1 pin security
    #[inline(always)]
    pub fn wup1sec(&self) -> WUP1SEC_R {
        WUP1SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WKUP2 pin security
    #[inline(always)]
    pub fn wup2sec(&self) -> WUP2SEC_R {
        WUP2SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WKUP3 pin security
    #[inline(always)]
    pub fn wup3sec(&self) -> WUP3SEC_R {
        WUP3SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WKUP4 pin security
    #[inline(always)]
    pub fn wup4sec(&self) -> WUP4SEC_R {
        WUP4SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WKUP5 pin security
    #[inline(always)]
    pub fn wup5sec(&self) -> WUP5SEC_R {
        WUP5SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - LPMSEC
    #[inline(always)]
    pub fn lpmsec(&self) -> LPMSEC_R {
        LPMSEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - VDMSEC
    #[inline(always)]
    pub fn vdmsec(&self) -> VDMSEC_R {
        VDMSEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - VBSEC
    #[inline(always)]
    pub fn vbsec(&self) -> VBSEC_R {
        VBSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - APCSEC
    #[inline(always)]
    pub fn apcsec(&self) -> APCSEC_R {
        APCSEC_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("apcsec", &self.apcsec())
            .field("vbsec", &self.vbsec())
            .field("vdmsec", &self.vdmsec())
            .field("lpmsec", &self.lpmsec())
            .field("wup5sec", &self.wup5sec())
            .field("wup4sec", &self.wup4sec())
            .field("wup3sec", &self.wup3sec())
            .field("wup2sec", &self.wup2sec())
            .field("wup1sec", &self.wup1sec())
            .finish()
    }
}
impl W {
    ///Bit 0 - WKUP1 pin security
    #[inline(always)]
    pub fn wup1sec(&mut self) -> WUP1SEC_W<'_, SECCFGRrs> {
        WUP1SEC_W::new(self, 0)
    }
    ///Bit 1 - WKUP2 pin security
    #[inline(always)]
    pub fn wup2sec(&mut self) -> WUP2SEC_W<'_, SECCFGRrs> {
        WUP2SEC_W::new(self, 1)
    }
    ///Bit 2 - WKUP3 pin security
    #[inline(always)]
    pub fn wup3sec(&mut self) -> WUP3SEC_W<'_, SECCFGRrs> {
        WUP3SEC_W::new(self, 2)
    }
    ///Bit 3 - WKUP4 pin security
    #[inline(always)]
    pub fn wup4sec(&mut self) -> WUP4SEC_W<'_, SECCFGRrs> {
        WUP4SEC_W::new(self, 3)
    }
    ///Bit 4 - WKUP5 pin security
    #[inline(always)]
    pub fn wup5sec(&mut self) -> WUP5SEC_W<'_, SECCFGRrs> {
        WUP5SEC_W::new(self, 4)
    }
    ///Bit 8 - LPMSEC
    #[inline(always)]
    pub fn lpmsec(&mut self) -> LPMSEC_W<'_, SECCFGRrs> {
        LPMSEC_W::new(self, 8)
    }
    ///Bit 9 - VDMSEC
    #[inline(always)]
    pub fn vdmsec(&mut self) -> VDMSEC_W<'_, SECCFGRrs> {
        VDMSEC_W::new(self, 9)
    }
    ///Bit 10 - VBSEC
    #[inline(always)]
    pub fn vbsec(&mut self) -> VBSEC_W<'_, SECCFGRrs> {
        VBSEC_W::new(self, 10)
    }
    ///Bit 11 - APCSEC
    #[inline(always)]
    pub fn apcsec(&mut self) -> APCSEC_W<'_, SECCFGRrs> {
        APCSEC_W::new(self, 11)
    }
}
/**Power secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#PWR:SECCFGR)*/
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
