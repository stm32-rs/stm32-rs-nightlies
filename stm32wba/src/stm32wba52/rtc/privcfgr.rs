///Register `PRIVCFGR` reader
pub type R = crate::R<PRIVCFGRrs>;
///Register `PRIVCFGR` writer
pub type W = crate::W<PRIVCFGRrs>;
///Field `ALRAPRIV` reader - Alarm A and SSR underflow privilege protection
pub type ALRAPRIV_R = crate::BitReader;
///Field `ALRAPRIV` writer - Alarm A and SSR underflow privilege protection
pub type ALRAPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBPRIV` reader - Alarm B privilege protection
pub type ALRBPRIV_R = crate::BitReader;
///Field `ALRBPRIV` writer - Alarm B privilege protection
pub type ALRBPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTPRIV` reader - Wakeup timer privilege protection
pub type WUTPRIV_R = crate::BitReader;
///Field `WUTPRIV` writer - Wakeup timer privilege protection
pub type WUTPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSPRIV` reader - Timestamp privilege protection
pub type TSPRIV_R = crate::BitReader;
///Field `TSPRIV` writer - Timestamp privilege protection
pub type TSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALPRIV` reader - Shift register, Delight saving, calibration and reference clock privilege protection
pub type CALPRIV_R = crate::BitReader;
///Field `CALPRIV` writer - Shift register, Delight saving, calibration and reference clock privilege protection
pub type CALPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INITPRIV` reader - Initialization privilege protection
pub type INITPRIV_R = crate::BitReader;
///Field `INITPRIV` writer - Initialization privilege protection
pub type INITPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV` reader - RTC privilege protection
pub type PRIV_R = crate::BitReader;
///Field `PRIV` writer - RTC privilege protection
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Alarm A and SSR underflow privilege protection
    #[inline(always)]
    pub fn alrapriv(&self) -> ALRAPRIV_R {
        ALRAPRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B privilege protection
    #[inline(always)]
    pub fn alrbpriv(&self) -> ALRBPRIV_R {
        ALRBPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup timer privilege protection
    #[inline(always)]
    pub fn wutpriv(&self) -> WUTPRIV_R {
        WUTPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp privilege protection
    #[inline(always)]
    pub fn tspriv(&self) -> TSPRIV_R {
        TSPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 13 - Shift register, Delight saving, calibration and reference clock privilege protection
    #[inline(always)]
    pub fn calpriv(&self) -> CALPRIV_R {
        CALPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Initialization privilege protection
    #[inline(always)]
    pub fn initpriv(&self) -> INITPRIV_R {
        INITPRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RTC privilege protection
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR")
            .field("alrapriv", &self.alrapriv())
            .field("alrbpriv", &self.alrbpriv())
            .field("wutpriv", &self.wutpriv())
            .field("tspriv", &self.tspriv())
            .field("calpriv", &self.calpriv())
            .field("initpriv", &self.initpriv())
            .field("priv_", &self.priv_())
            .finish()
    }
}
impl W {
    ///Bit 0 - Alarm A and SSR underflow privilege protection
    #[inline(always)]
    pub fn alrapriv(&mut self) -> ALRAPRIV_W<'_, PRIVCFGRrs> {
        ALRAPRIV_W::new(self, 0)
    }
    ///Bit 1 - Alarm B privilege protection
    #[inline(always)]
    pub fn alrbpriv(&mut self) -> ALRBPRIV_W<'_, PRIVCFGRrs> {
        ALRBPRIV_W::new(self, 1)
    }
    ///Bit 2 - Wakeup timer privilege protection
    #[inline(always)]
    pub fn wutpriv(&mut self) -> WUTPRIV_W<'_, PRIVCFGRrs> {
        WUTPRIV_W::new(self, 2)
    }
    ///Bit 3 - Timestamp privilege protection
    #[inline(always)]
    pub fn tspriv(&mut self) -> TSPRIV_W<'_, PRIVCFGRrs> {
        TSPRIV_W::new(self, 3)
    }
    ///Bit 13 - Shift register, Delight saving, calibration and reference clock privilege protection
    #[inline(always)]
    pub fn calpriv(&mut self) -> CALPRIV_W<'_, PRIVCFGRrs> {
        CALPRIV_W::new(self, 13)
    }
    ///Bit 14 - Initialization privilege protection
    #[inline(always)]
    pub fn initpriv(&mut self) -> INITPRIV_W<'_, PRIVCFGRrs> {
        INITPRIV_W::new(self, 14)
    }
    ///Bit 15 - RTC privilege protection
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<'_, PRIVCFGRrs> {
        PRIV_W::new(self, 15)
    }
}
/**RTC privilege mode control register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#RTC:PRIVCFGR)*/
pub struct PRIVCFGRrs;
impl crate::RegisterSpec for PRIVCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr::R`](R) reader structure
impl crate::Readable for PRIVCFGRrs {}
///`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure
impl crate::Writable for PRIVCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR to value 0
impl crate::Resettable for PRIVCFGRrs {}
