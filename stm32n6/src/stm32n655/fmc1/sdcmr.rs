///Register `SDCMR` reader
pub type R = crate::R<SDCMRrs>;
///Register `SDCMR` writer
pub type W = crate::W<SDCMRrs>;
///Field `MODE` writer - Command mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DS2` writer - Command targeting SDRAM device 2
pub type DS2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DS1` writer - Command targeting SDRAM device 1
pub type DS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRFS` reader - Number of Refresh commands
pub type NRFS_R = crate::FieldReader;
///Field `NRFS` writer - Number of Refresh commands
pub type NRFS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MRD` reader - Mode register definition
pub type MRD_R = crate::FieldReader<u16>;
///Field `MRD` writer - Mode register definition
pub type MRD_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 5:8 - Number of Refresh commands
    #[inline(always)]
    pub fn nrfs(&self) -> NRFS_R {
        NRFS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bits 9:22 - Mode register definition
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 9) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDCMR")
            .field("nrfs", &self.nrfs())
            .field("mrd", &self.mrd())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Command mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, SDCMRrs> {
        MODE_W::new(self, 0)
    }
    ///Bit 3 - Command targeting SDRAM device 2
    #[inline(always)]
    pub fn ds2(&mut self) -> DS2_W<'_, SDCMRrs> {
        DS2_W::new(self, 3)
    }
    ///Bit 4 - Command targeting SDRAM device 1
    #[inline(always)]
    pub fn ds1(&mut self) -> DS1_W<'_, SDCMRrs> {
        DS1_W::new(self, 4)
    }
    ///Bits 5:8 - Number of Refresh commands
    #[inline(always)]
    pub fn nrfs(&mut self) -> NRFS_W<'_, SDCMRrs> {
        NRFS_W::new(self, 5)
    }
    ///Bits 9:22 - Mode register definition
    #[inline(always)]
    pub fn mrd(&mut self) -> MRD_W<'_, SDCMRrs> {
        MRD_W::new(self, 9)
    }
}
/**SDRAM command mode register

You can [`read`](crate::Reg::read) this register and get [`sdcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#FMC1:SDCMR)*/
pub struct SDCMRrs;
impl crate::RegisterSpec for SDCMRrs {
    type Ux = u32;
}
///`read()` method returns [`sdcmr::R`](R) reader structure
impl crate::Readable for SDCMRrs {}
///`write(|w| ..)` method takes [`sdcmr::W`](W) writer structure
impl crate::Writable for SDCMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDCMR to value 0
impl crate::Resettable for SDCMRrs {}
