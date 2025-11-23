///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `ALRASEC` reader - Alarm A and SSR underflow protection
pub type ALRASEC_R = crate::BitReader;
///Field `ALRASEC` writer - Alarm A and SSR underflow protection
pub type ALRASEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBSEC` reader - Alarm B protection
pub type ALRBSEC_R = crate::BitReader;
///Field `ALRBSEC` writer - Alarm B protection
pub type ALRBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTSEC` reader - Wakeup timer protection
pub type WUTSEC_R = crate::BitReader;
///Field `WUTSEC` writer - Wakeup timer protection
pub type WUTSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSSEC` reader - Timestamp protection
pub type TSSEC_R = crate::BitReader;
///Field `TSSEC` writer - Timestamp protection
pub type TSSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALSEC` reader - Shift register, daylight saving, calibration and reference clock protection
pub type CALSEC_R = crate::BitReader;
///Field `CALSEC` writer - Shift register, daylight saving, calibration and reference clock protection
pub type CALSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INITSEC` reader - Initialization protection
pub type INITSEC_R = crate::BitReader;
///Field `INITSEC` writer - Initialization protection
pub type INITSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC` reader - RTC global protection
pub type SEC_R = crate::BitReader;
///Field `SEC` writer - RTC global protection
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Alarm A and SSR underflow protection
    #[inline(always)]
    pub fn alrasec(&self) -> ALRASEC_R {
        ALRASEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B protection
    #[inline(always)]
    pub fn alrbsec(&self) -> ALRBSEC_R {
        ALRBSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup timer protection
    #[inline(always)]
    pub fn wutsec(&self) -> WUTSEC_R {
        WUTSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp protection
    #[inline(always)]
    pub fn tssec(&self) -> TSSEC_R {
        TSSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 13 - Shift register, daylight saving, calibration and reference clock protection
    #[inline(always)]
    pub fn calsec(&self) -> CALSEC_R {
        CALSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Initialization protection
    #[inline(always)]
    pub fn initsec(&self) -> INITSEC_R {
        INITSEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RTC global protection
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("alrasec", &self.alrasec())
            .field("alrbsec", &self.alrbsec())
            .field("wutsec", &self.wutsec())
            .field("tssec", &self.tssec())
            .field("calsec", &self.calsec())
            .field("initsec", &self.initsec())
            .field("sec", &self.sec())
            .finish()
    }
}
impl W {
    ///Bit 0 - Alarm A and SSR underflow protection
    #[inline(always)]
    pub fn alrasec(&mut self) -> ALRASEC_W<'_, SECCFGRrs> {
        ALRASEC_W::new(self, 0)
    }
    ///Bit 1 - Alarm B protection
    #[inline(always)]
    pub fn alrbsec(&mut self) -> ALRBSEC_W<'_, SECCFGRrs> {
        ALRBSEC_W::new(self, 1)
    }
    ///Bit 2 - Wakeup timer protection
    #[inline(always)]
    pub fn wutsec(&mut self) -> WUTSEC_W<'_, SECCFGRrs> {
        WUTSEC_W::new(self, 2)
    }
    ///Bit 3 - Timestamp protection
    #[inline(always)]
    pub fn tssec(&mut self) -> TSSEC_W<'_, SECCFGRrs> {
        TSSEC_W::new(self, 3)
    }
    ///Bit 13 - Shift register, daylight saving, calibration and reference clock protection
    #[inline(always)]
    pub fn calsec(&mut self) -> CALSEC_W<'_, SECCFGRrs> {
        CALSEC_W::new(self, 13)
    }
    ///Bit 14 - Initialization protection
    #[inline(always)]
    pub fn initsec(&mut self) -> INITSEC_W<'_, SECCFGRrs> {
        INITSEC_W::new(self, 14)
    }
    ///Bit 15 - RTC global protection
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<'_, SECCFGRrs> {
        SEC_W::new(self, 15)
    }
}
/**RTC secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#RTC:SECCFGR)*/
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
