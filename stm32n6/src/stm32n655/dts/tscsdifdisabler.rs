///Register `TSCSDIFDISABLER` reader
pub type R = crate::R<TSCSDIFDISABLERrs>;
///Register `TSCSDIFDISABLER` writer
pub type W = crate::W<TSCSDIFDISABLERrs>;
///Field `TS0_SDIF_DISABLE` reader - TS0 serial data interface (SDIF) disable bit
pub type TS0_SDIF_DISABLE_R = crate::BitReader;
///Field `TS0_SDIF_DISABLE` writer - TS0 serial data interface (SDIF) disable bit
pub type TS0_SDIF_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_SDIF_DISABLE` reader - TS1 serial data interface (SDIF) disable bit
pub type TS1_SDIF_DISABLE_R = crate::BitReader;
///Field `TS1_SDIF_DISABLE` writer - TS1 serial data interface (SDIF) disable bit
pub type TS1_SDIF_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TS0 serial data interface (SDIF) disable bit
    #[inline(always)]
    pub fn ts0_sdif_disable(&self) -> TS0_SDIF_DISABLE_R {
        TS0_SDIF_DISABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TS1 serial data interface (SDIF) disable bit
    #[inline(always)]
    pub fn ts1_sdif_disable(&self) -> TS1_SDIF_DISABLE_R {
        TS1_SDIF_DISABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCSDIFDISABLER")
            .field("ts0_sdif_disable", &self.ts0_sdif_disable())
            .field("ts1_sdif_disable", &self.ts1_sdif_disable())
            .finish()
    }
}
impl W {
    ///Bit 0 - TS0 serial data interface (SDIF) disable bit
    #[inline(always)]
    pub fn ts0_sdif_disable(&mut self) -> TS0_SDIF_DISABLE_W<'_, TSCSDIFDISABLERrs> {
        TS0_SDIF_DISABLE_W::new(self, 0)
    }
    ///Bit 1 - TS1 serial data interface (SDIF) disable bit
    #[inline(always)]
    pub fn ts1_sdif_disable(&mut self) -> TS1_SDIF_DISABLE_W<'_, TSCSDIFDISABLERrs> {
        TS1_SDIF_DISABLE_W::new(self, 1)
    }
}
/**DTS TSC SDIF interface disable register

You can [`read`](crate::Reg::read) this register and get [`tscsdifdisabler::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscsdifdisabler::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DTS:TSCSDIFDISABLER)*/
pub struct TSCSDIFDISABLERrs;
impl crate::RegisterSpec for TSCSDIFDISABLERrs {
    type Ux = u32;
}
///`read()` method returns [`tscsdifdisabler::R`](R) reader structure
impl crate::Readable for TSCSDIFDISABLERrs {}
///`write(|w| ..)` method takes [`tscsdifdisabler::W`](W) writer structure
impl crate::Writable for TSCSDIFDISABLERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSCSDIFDISABLER to value 0
impl crate::Resettable for TSCSDIFDISABLERrs {}
