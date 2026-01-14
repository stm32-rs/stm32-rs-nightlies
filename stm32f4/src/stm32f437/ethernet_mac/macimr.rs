///Register `MACIMR` reader
pub type R = crate::R<MACIMRrs>;
///Register `MACIMR` writer
pub type W = crate::W<MACIMRrs>;
///Field `PMTIM` reader - PMTIM
pub type PMTIM_R = crate::BitReader;
///Field `PMTIM` writer - PMTIM
pub type PMTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSTIM` reader - TSTIM
pub type TSTIM_R = crate::BitReader;
///Field `TSTIM` writer - TSTIM
pub type TSTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - PMTIM
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 9 - TSTIM
    #[inline(always)]
    pub fn tstim(&self) -> TSTIM_R {
        TSTIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACIMR")
            .field("pmtim", &self.pmtim())
            .field("tstim", &self.tstim())
            .finish()
    }
}
impl W {
    ///Bit 3 - PMTIM
    #[inline(always)]
    pub fn pmtim(&mut self) -> PMTIM_W<'_, MACIMRrs> {
        PMTIM_W::new(self, 3)
    }
    ///Bit 9 - TSTIM
    #[inline(always)]
    pub fn tstim(&mut self) -> TSTIM_W<'_, MACIMRrs> {
        TSTIM_W::new(self, 9)
    }
}
/**Ethernet MAC interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`macimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#Ethernet_MAC:MACIMR)*/
pub struct MACIMRrs;
impl crate::RegisterSpec for MACIMRrs {
    type Ux = u32;
}
///`read()` method returns [`macimr::R`](R) reader structure
impl crate::Readable for MACIMRrs {}
///`write(|w| ..)` method takes [`macimr::W`](W) writer structure
impl crate::Writable for MACIMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACIMR to value 0
impl crate::Resettable for MACIMRrs {}
