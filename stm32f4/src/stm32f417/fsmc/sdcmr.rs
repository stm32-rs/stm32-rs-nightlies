///Register `SDCMR` reader
pub type R = crate::R<SDCMRrs>;
///Register `SDCMR` writer
pub type W = crate::W<SDCMRrs>;
///Field `MODE` writer - Command mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CTB2` writer - Command target bank 2
pub type CTB2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTB1` writer - Command target bank 1
pub type CTB1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRFS` reader - Number of Auto-refresh
pub type NRFS_R = crate::FieldReader;
///Field `NRFS` writer - Number of Auto-refresh
pub type NRFS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MRD` reader - Mode Register definition
pub type MRD_R = crate::FieldReader<u16>;
///Field `MRD` writer - Mode Register definition
pub type MRD_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 5:8 - Number of Auto-refresh
    #[inline(always)]
    pub fn nrfs(&self) -> NRFS_R {
        NRFS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bits 9:21 - Mode Register definition
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 9) & 0x1fff) as u16)
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
    ///Bit 3 - Command target bank 2
    #[inline(always)]
    pub fn ctb2(&mut self) -> CTB2_W<'_, SDCMRrs> {
        CTB2_W::new(self, 3)
    }
    ///Bit 4 - Command target bank 1
    #[inline(always)]
    pub fn ctb1(&mut self) -> CTB1_W<'_, SDCMRrs> {
        CTB1_W::new(self, 4)
    }
    ///Bits 5:8 - Number of Auto-refresh
    #[inline(always)]
    pub fn nrfs(&mut self) -> NRFS_W<'_, SDCMRrs> {
        NRFS_W::new(self, 5)
    }
    ///Bits 9:21 - Mode Register definition
    #[inline(always)]
    pub fn mrd(&mut self) -> MRD_W<'_, SDCMRrs> {
        MRD_W::new(self, 9)
    }
}
/**SDRAM Command Mode register

You can [`read`](crate::Reg::read) this register and get [`sdcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#FSMC:SDCMR)*/
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
