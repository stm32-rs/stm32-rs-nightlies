///Register `MMC_CONTROL` reader
pub type R = crate::R<MMC_CONTROLrs>;
///Register `MMC_CONTROL` writer
pub type W = crate::W<MMC_CONTROLrs>;
///Field `CNTRST` reader - CNTRST
pub type CNTRST_R = crate::BitReader;
///Field `CNTRST` writer - CNTRST
pub type CNTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTSTOPRO` reader - CNTSTOPRO
pub type CNTSTOPRO_R = crate::BitReader;
///Field `CNTSTOPRO` writer - CNTSTOPRO
pub type CNTSTOPRO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTONRD` reader - RSTONRD
pub type RSTONRD_R = crate::BitReader;
///Field `RSTONRD` writer - RSTONRD
pub type RSTONRD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTFREEZ` reader - CNTFREEZ
pub type CNTFREEZ_R = crate::BitReader;
///Field `CNTFREEZ` writer - CNTFREEZ
pub type CNTFREEZ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTPRST` reader - CNTPRST
pub type CNTPRST_R = crate::BitReader;
///Field `CNTPRST` writer - CNTPRST
pub type CNTPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTPRSTLVL` reader - CNTPRSTLVL
pub type CNTPRSTLVL_R = crate::BitReader;
///Field `CNTPRSTLVL` writer - CNTPRSTLVL
pub type CNTPRSTLVL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCDBC` reader - UCDBC
pub type UCDBC_R = crate::BitReader;
///Field `UCDBC` writer - UCDBC
pub type UCDBC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CNTRST
    #[inline(always)]
    pub fn cntrst(&self) -> CNTRST_R {
        CNTRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CNTSTOPRO
    #[inline(always)]
    pub fn cntstopro(&self) -> CNTSTOPRO_R {
        CNTSTOPRO_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RSTONRD
    #[inline(always)]
    pub fn rstonrd(&self) -> RSTONRD_R {
        RSTONRD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CNTFREEZ
    #[inline(always)]
    pub fn cntfreez(&self) -> CNTFREEZ_R {
        CNTFREEZ_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CNTPRST
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CNTPRSTLVL
    #[inline(always)]
    pub fn cntprstlvl(&self) -> CNTPRSTLVL_R {
        CNTPRSTLVL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - UCDBC
    #[inline(always)]
    pub fn ucdbc(&self) -> UCDBC_R {
        UCDBC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMC_CONTROL")
            .field("cntrst", &self.cntrst())
            .field("cntstopro", &self.cntstopro())
            .field("rstonrd", &self.rstonrd())
            .field("cntfreez", &self.cntfreez())
            .field("cntprst", &self.cntprst())
            .field("cntprstlvl", &self.cntprstlvl())
            .field("ucdbc", &self.ucdbc())
            .finish()
    }
}
impl W {
    ///Bit 0 - CNTRST
    #[inline(always)]
    pub fn cntrst(&mut self) -> CNTRST_W<'_, MMC_CONTROLrs> {
        CNTRST_W::new(self, 0)
    }
    ///Bit 1 - CNTSTOPRO
    #[inline(always)]
    pub fn cntstopro(&mut self) -> CNTSTOPRO_W<'_, MMC_CONTROLrs> {
        CNTSTOPRO_W::new(self, 1)
    }
    ///Bit 2 - RSTONRD
    #[inline(always)]
    pub fn rstonrd(&mut self) -> RSTONRD_W<'_, MMC_CONTROLrs> {
        RSTONRD_W::new(self, 2)
    }
    ///Bit 3 - CNTFREEZ
    #[inline(always)]
    pub fn cntfreez(&mut self) -> CNTFREEZ_W<'_, MMC_CONTROLrs> {
        CNTFREEZ_W::new(self, 3)
    }
    ///Bit 4 - CNTPRST
    #[inline(always)]
    pub fn cntprst(&mut self) -> CNTPRST_W<'_, MMC_CONTROLrs> {
        CNTPRST_W::new(self, 4)
    }
    ///Bit 5 - CNTPRSTLVL
    #[inline(always)]
    pub fn cntprstlvl(&mut self) -> CNTPRSTLVL_W<'_, MMC_CONTROLrs> {
        CNTPRSTLVL_W::new(self, 5)
    }
    ///Bit 8 - UCDBC
    #[inline(always)]
    pub fn ucdbc(&mut self) -> UCDBC_W<'_, MMC_CONTROLrs> {
        UCDBC_W::new(self, 8)
    }
}
/**This register configures the MMC operating mode.

You can [`read`](crate::Reg::read) this register and get [`mmc_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MMC_CONTROL)*/
pub struct MMC_CONTROLrs;
impl crate::RegisterSpec for MMC_CONTROLrs {
    type Ux = u32;
}
///`read()` method returns [`mmc_control::R`](R) reader structure
impl crate::Readable for MMC_CONTROLrs {}
///`write(|w| ..)` method takes [`mmc_control::W`](W) writer structure
impl crate::Writable for MMC_CONTROLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMC_CONTROL to value 0
impl crate::Resettable for MMC_CONTROLrs {}
