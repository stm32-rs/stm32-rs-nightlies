///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `RXFILTDIS` reader - BMC decoder Rx pre-filter enable
pub type RXFILTDIS_R = crate::BitReader;
///Field `RXFILTDIS` writer - BMC decoder Rx pre-filter enable
pub type RXFILTDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFILT2N3` reader - BMC decoder Rx pre-filter sampling method
pub type RXFILT2N3_R = crate::BitReader;
///Field `RXFILT2N3` writer - BMC decoder Rx pre-filter sampling method
pub type RXFILT2N3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCECLK` reader - Force ClkReq clock request
pub type FORCECLK_R = crate::BitReader;
///Field `FORCECLK` writer - Force ClkReq clock request
pub type FORCECLK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPEN` reader - Wakeup from Stop mode enable
pub type WUPEN_R = crate::BitReader;
///Field `WUPEN` writer - Wakeup from Stop mode enable
pub type WUPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BMC decoder Rx pre-filter enable
    #[inline(always)]
    pub fn rxfiltdis(&self) -> RXFILTDIS_R {
        RXFILTDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BMC decoder Rx pre-filter sampling method
    #[inline(always)]
    pub fn rxfilt2n3(&self) -> RXFILT2N3_R {
        RXFILT2N3_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Force ClkReq clock request
    #[inline(always)]
    pub fn forceclk(&self) -> FORCECLK_R {
        FORCECLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup from Stop mode enable
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("rxfiltdis", &self.rxfiltdis())
            .field("rxfilt2n3", &self.rxfilt2n3())
            .field("forceclk", &self.forceclk())
            .field("wupen", &self.wupen())
            .finish()
    }
}
impl W {
    ///Bit 0 - BMC decoder Rx pre-filter enable
    #[inline(always)]
    pub fn rxfiltdis(&mut self) -> RXFILTDIS_W<CFGR2rs> {
        RXFILTDIS_W::new(self, 0)
    }
    ///Bit 1 - BMC decoder Rx pre-filter sampling method
    #[inline(always)]
    pub fn rxfilt2n3(&mut self) -> RXFILT2N3_W<CFGR2rs> {
        RXFILT2N3_W::new(self, 1)
    }
    ///Bit 2 - Force ClkReq clock request
    #[inline(always)]
    pub fn forceclk(&mut self) -> FORCECLK_W<CFGR2rs> {
        FORCECLK_W::new(self, 2)
    }
    ///Bit 3 - Wakeup from Stop mode enable
    #[inline(always)]
    pub fn wupen(&mut self) -> WUPEN_W<CFGR2rs> {
        WUPEN_W::new(self, 3)
    }
}
/**UCPD configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#UCPD:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
