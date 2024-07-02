///Register `PWR_SECCFGR` reader
pub type R = crate::R<PWR_SECCFGRrs>;
///Register `PWR_SECCFGR` writer
pub type W = crate::W<PWR_SECCFGRrs>;
///Field `WUP1SEC` reader - WUP1 secure protection
pub type WUP1SEC_R = crate::BitReader;
///Field `WUP1SEC` writer - WUP1 secure protection
pub type WUP1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP2SEC` reader - WUP2 secure protection
pub type WUP2SEC_R = crate::BitReader;
///Field `WUP2SEC` writer - WUP2 secure protection
pub type WUP2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP3SEC` reader - WUP3 secure protection
pub type WUP3SEC_R = crate::BitReader;
///Field `WUP3SEC` writer - WUP3 secure protection
pub type WUP3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP4SEC` reader - WUP4 secure protection
pub type WUP4SEC_R = crate::BitReader;
///Field `WUP4SEC` writer - WUP4 secure protection
pub type WUP4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP5SEC` reader - WUP5 secure protection
pub type WUP5SEC_R = crate::BitReader;
///Field `WUP5SEC` writer - WUP5 secure protection
pub type WUP5SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP6SEC` reader - WUP6 secure protection
pub type WUP6SEC_R = crate::BitReader;
///Field `WUP6SEC` writer - WUP6 secure protection
pub type WUP6SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP7SEC` reader - WUP7 secure protection
pub type WUP7SEC_R = crate::BitReader;
///Field `WUP7SEC` writer - WUP7 secure protection
pub type WUP7SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUP8SEC` reader - WUP8 secure protection
pub type WUP8SEC_R = crate::BitReader;
///Field `WUP8SEC` writer - WUP8 secure protection
pub type WUP8SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPMSEC` reader - Low-power modes secure protection
pub type LPMSEC_R = crate::BitReader;
///Field `LPMSEC` writer - Low-power modes secure protection
pub type LPMSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDMSEC` reader - Voltage detection and monitoring secure protection
pub type VDMSEC_R = crate::BitReader;
///Field `VDMSEC` writer - Voltage detection and monitoring secure protection
pub type VDMSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBSEC` reader - Backup domain secure protection
pub type VBSEC_R = crate::BitReader;
///Field `VBSEC` writer - Backup domain secure protection
pub type VBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APCSEC` reader - Pull-up/pull-down secure protection
pub type APCSEC_R = crate::BitReader;
///Field `APCSEC` writer - Pull-up/pull-down secure protection
pub type APCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - WUP1 secure protection
    #[inline(always)]
    pub fn wup1sec(&self) -> WUP1SEC_R {
        WUP1SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WUP2 secure protection
    #[inline(always)]
    pub fn wup2sec(&self) -> WUP2SEC_R {
        WUP2SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WUP3 secure protection
    #[inline(always)]
    pub fn wup3sec(&self) -> WUP3SEC_R {
        WUP3SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WUP4 secure protection
    #[inline(always)]
    pub fn wup4sec(&self) -> WUP4SEC_R {
        WUP4SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WUP5 secure protection
    #[inline(always)]
    pub fn wup5sec(&self) -> WUP5SEC_R {
        WUP5SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - WUP6 secure protection
    #[inline(always)]
    pub fn wup6sec(&self) -> WUP6SEC_R {
        WUP6SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - WUP7 secure protection
    #[inline(always)]
    pub fn wup7sec(&self) -> WUP7SEC_R {
        WUP7SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - WUP8 secure protection
    #[inline(always)]
    pub fn wup8sec(&self) -> WUP8SEC_R {
        WUP8SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - Low-power modes secure protection
    #[inline(always)]
    pub fn lpmsec(&self) -> LPMSEC_R {
        LPMSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Voltage detection and monitoring secure protection
    #[inline(always)]
    pub fn vdmsec(&self) -> VDMSEC_R {
        VDMSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Backup domain secure protection
    #[inline(always)]
    pub fn vbsec(&self) -> VBSEC_R {
        VBSEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Pull-up/pull-down secure protection
    #[inline(always)]
    pub fn apcsec(&self) -> APCSEC_R {
        APCSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_SECCFGR")
            .field("wup1sec", &self.wup1sec())
            .field("wup2sec", &self.wup2sec())
            .field("wup3sec", &self.wup3sec())
            .field("wup4sec", &self.wup4sec())
            .field("wup5sec", &self.wup5sec())
            .field("wup6sec", &self.wup6sec())
            .field("wup7sec", &self.wup7sec())
            .field("wup8sec", &self.wup8sec())
            .field("lpmsec", &self.lpmsec())
            .field("vdmsec", &self.vdmsec())
            .field("vbsec", &self.vbsec())
            .field("apcsec", &self.apcsec())
            .finish()
    }
}
impl W {
    ///Bit 0 - WUP1 secure protection
    #[inline(always)]
    #[must_use]
    pub fn wup1sec(&mut self) -> WUP1SEC_W<PWR_SECCFGRrs> {
        WUP1SEC_W::new(self, 0)
    }
    ///Bit 1 - WUP2 secure protection
    #[inline(always)]
    #[must_use]
    pub fn wup2sec(&mut self) -> WUP2SEC_W<PWR_SECCFGRrs> {
        WUP2SEC_W::new(self, 1)
    }
    ///Bit 2 - WUP3 secure protection
    #[inline(always)]
    #[must_use]
    pub fn wup3sec(&mut self) -> WUP3SEC_W<PWR_SECCFGRrs> {
        WUP3SEC_W::new(self, 2)
    }
    ///Bit 3 - WUP4 secure protection
    #[inline(always)]
    #[must_use]
    pub fn wup4sec(&mut self) -> WUP4SEC_W<PWR_SECCFGRrs> {
        WUP4SEC_W::new(self, 3)
    }
    ///Bit 4 - WUP5 secure protection
    #[inline(always)]
    #[must_use]
    pub fn wup5sec(&mut self) -> WUP5SEC_W<PWR_SECCFGRrs> {
        WUP5SEC_W::new(self, 4)
    }
    ///Bit 5 - WUP6 secure protection
    #[inline(always)]
    #[must_use]
    pub fn wup6sec(&mut self) -> WUP6SEC_W<PWR_SECCFGRrs> {
        WUP6SEC_W::new(self, 5)
    }
    ///Bit 6 - WUP7 secure protection
    #[inline(always)]
    #[must_use]
    pub fn wup7sec(&mut self) -> WUP7SEC_W<PWR_SECCFGRrs> {
        WUP7SEC_W::new(self, 6)
    }
    ///Bit 7 - WUP8 secure protection
    #[inline(always)]
    #[must_use]
    pub fn wup8sec(&mut self) -> WUP8SEC_W<PWR_SECCFGRrs> {
        WUP8SEC_W::new(self, 7)
    }
    ///Bit 12 - Low-power modes secure protection
    #[inline(always)]
    #[must_use]
    pub fn lpmsec(&mut self) -> LPMSEC_W<PWR_SECCFGRrs> {
        LPMSEC_W::new(self, 12)
    }
    ///Bit 13 - Voltage detection and monitoring secure protection
    #[inline(always)]
    #[must_use]
    pub fn vdmsec(&mut self) -> VDMSEC_W<PWR_SECCFGRrs> {
        VDMSEC_W::new(self, 13)
    }
    ///Bit 14 - Backup domain secure protection
    #[inline(always)]
    #[must_use]
    pub fn vbsec(&mut self) -> VBSEC_W<PWR_SECCFGRrs> {
        VBSEC_W::new(self, 14)
    }
    ///Bit 15 - Pull-up/pull-down secure protection
    #[inline(always)]
    #[must_use]
    pub fn apcsec(&mut self) -> APCSEC_W<PWR_SECCFGRrs> {
        APCSEC_W::new(self, 15)
    }
}
/**PWR security configuration register

You can [`read`](crate::Reg::read) this register and get [`pwr_seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#PWR:PWR_SECCFGR)*/
pub struct PWR_SECCFGRrs;
impl crate::RegisterSpec for PWR_SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_seccfgr::R`](R) reader structure
impl crate::Readable for PWR_SECCFGRrs {}
///`write(|w| ..)` method takes [`pwr_seccfgr::W`](W) writer structure
impl crate::Writable for PWR_SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_SECCFGR to value 0
impl crate::Resettable for PWR_SECCFGRrs {
    const RESET_VALUE: u32 = 0;
}
