///Register `OPTSR_PRG` reader
pub type R = crate::R<OPTSR_PRGrs>;
///Register `OPTSR_PRG` writer
pub type W = crate::W<OPTSR_PRGrs>;
///Field `BOR_LEV` reader - BOR reset level option configuration bits
pub type BOR_LEV_R = crate::FieldReader;
///Field `BOR_LEV` writer - BOR reset level option configuration bits
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IWDG_SW` reader - IWDG1 option configuration bit
pub type IWDG_SW_R = crate::BitReader;
///Field `IWDG_SW` writer - IWDG1 option configuration bit
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STOP` reader - Option byte erase after D1 DStop option configuration bit
pub type NRST_STOP_R = crate::BitReader;
///Field `NRST_STOP` writer - Option byte erase after D1 DStop option configuration bit
pub type NRST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STDY` reader - Option byte erase after D1 DStandby option configuration bit
pub type NRST_STDY_R = crate::BitReader;
///Field `NRST_STDY` writer - Option byte erase after D1 DStandby option configuration bit
pub type NRST_STDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDP` reader - Readout protection level option configuration byte
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Readout protection level option configuration byte
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `VDDMMC_HSLV` reader - VDDMMC_HSLV
pub type VDDMMC_HSLV_R = crate::BitReader;
///Field `VDDMMC_HSLV` writer - VDDMMC_HSLV
pub type VDDMMC_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDG_FZ_STOP` reader - IWDG Stop mode freeze option configuration bit
pub type WDG_FZ_STOP_R = crate::BitReader;
///Field `WDG_FZ_STOP` writer - IWDG Stop mode freeze option configuration bit
pub type WDG_FZ_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_FZ_SDBY` reader - IWDG Standby mode freeze option configuration bit
pub type IWDG_FZ_SDBY_R = crate::BitReader;
///Field `IWDG_FZ_SDBY` writer - IWDG Standby mode freeze option configuration bit
pub type IWDG_FZ_SDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ST_RAM_SIZE` reader - DTCM size select option configuration bits
pub type ST_RAM_SIZE_R = crate::FieldReader;
///Field `ST_RAM_SIZE` writer - DTCM size select option configuration bits
pub type ST_RAM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SECURITY` reader - Security option configuration bit
pub type SECURITY_R = crate::BitReader;
///Field `SECURITY` writer - Security option configuration bit
pub type SECURITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO_HSLV` reader - VDDIO_HSLV
pub type VDDIO_HSLV_R = crate::BitReader;
///Field `VDDIO_HSLV` writer - VDDIO_HSLV
pub type VDDIO_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP_BANK_OPT` reader - Bank swapping option configuration bit
pub type SWAP_BANK_OPT_R = crate::BitReader;
///Field `SWAP_BANK_OPT` writer - Bank swapping option configuration bit
pub type SWAP_BANK_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 2:3 - BOR reset level option configuration bits
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - IWDG1 option configuration bit
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Option byte erase after D1 DStop option configuration bit
    #[inline(always)]
    pub fn nrst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Option byte erase after D1 DStandby option configuration bit
    #[inline(always)]
    pub fn nrst_stdy(&self) -> NRST_STDY_R {
        NRST_STDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Readout protection level option configuration byte
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - VDDMMC_HSLV
    #[inline(always)]
    pub fn vddmmc_hslv(&self) -> VDDMMC_HSLV_R {
        VDDMMC_HSLV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - IWDG Stop mode freeze option configuration bit
    #[inline(always)]
    pub fn wdg_fz_stop(&self) -> WDG_FZ_STOP_R {
        WDG_FZ_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IWDG Standby mode freeze option configuration bit
    #[inline(always)]
    pub fn iwdg_fz_sdby(&self) -> IWDG_FZ_SDBY_R {
        IWDG_FZ_SDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:20 - DTCM size select option configuration bits
    #[inline(always)]
    pub fn st_ram_size(&self) -> ST_RAM_SIZE_R {
        ST_RAM_SIZE_R::new(((self.bits >> 19) & 3) as u8)
    }
    ///Bit 21 - Security option configuration bit
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 29 - VDDIO_HSLV
    #[inline(always)]
    pub fn vddio_hslv(&self) -> VDDIO_HSLV_R {
        VDDIO_HSLV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Bank swapping option configuration bit
    #[inline(always)]
    pub fn swap_bank_opt(&self) -> SWAP_BANK_OPT_R {
        SWAP_BANK_OPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTSR_PRG")
            .field("bor_lev", &self.bor_lev())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("nrst_stop", &self.nrst_stop())
            .field("nrst_stdy", &self.nrst_stdy())
            .field("rdp", &self.rdp())
            .field("vddmmc_hslv", &self.vddmmc_hslv())
            .field("wdg_fz_stop", &self.wdg_fz_stop())
            .field("iwdg_fz_sdby", &self.iwdg_fz_sdby())
            .field("st_ram_size", &self.st_ram_size())
            .field("security", &self.security())
            .field("vddio_hslv", &self.vddio_hslv())
            .field("swap_bank_opt", &self.swap_bank_opt())
            .finish()
    }
}
impl W {
    ///Bits 2:3 - BOR reset level option configuration bits
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<'_, OPTSR_PRGrs> {
        BOR_LEV_W::new(self, 2)
    }
    ///Bit 4 - IWDG1 option configuration bit
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<'_, OPTSR_PRGrs> {
        IWDG_SW_W::new(self, 4)
    }
    ///Bit 6 - Option byte erase after D1 DStop option configuration bit
    #[inline(always)]
    pub fn nrst_stop(&mut self) -> NRST_STOP_W<'_, OPTSR_PRGrs> {
        NRST_STOP_W::new(self, 6)
    }
    ///Bit 7 - Option byte erase after D1 DStandby option configuration bit
    #[inline(always)]
    pub fn nrst_stdy(&mut self) -> NRST_STDY_W<'_, OPTSR_PRGrs> {
        NRST_STDY_W::new(self, 7)
    }
    ///Bits 8:15 - Readout protection level option configuration byte
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, OPTSR_PRGrs> {
        RDP_W::new(self, 8)
    }
    ///Bit 16 - VDDMMC_HSLV
    #[inline(always)]
    pub fn vddmmc_hslv(&mut self) -> VDDMMC_HSLV_W<'_, OPTSR_PRGrs> {
        VDDMMC_HSLV_W::new(self, 16)
    }
    ///Bit 17 - IWDG Stop mode freeze option configuration bit
    #[inline(always)]
    pub fn wdg_fz_stop(&mut self) -> WDG_FZ_STOP_W<'_, OPTSR_PRGrs> {
        WDG_FZ_STOP_W::new(self, 17)
    }
    ///Bit 18 - IWDG Standby mode freeze option configuration bit
    #[inline(always)]
    pub fn iwdg_fz_sdby(&mut self) -> IWDG_FZ_SDBY_W<'_, OPTSR_PRGrs> {
        IWDG_FZ_SDBY_W::new(self, 18)
    }
    ///Bits 19:20 - DTCM size select option configuration bits
    #[inline(always)]
    pub fn st_ram_size(&mut self) -> ST_RAM_SIZE_W<'_, OPTSR_PRGrs> {
        ST_RAM_SIZE_W::new(self, 19)
    }
    ///Bit 21 - Security option configuration bit
    #[inline(always)]
    pub fn security(&mut self) -> SECURITY_W<'_, OPTSR_PRGrs> {
        SECURITY_W::new(self, 21)
    }
    ///Bit 29 - VDDIO_HSLV
    #[inline(always)]
    pub fn vddio_hslv(&mut self) -> VDDIO_HSLV_W<'_, OPTSR_PRGrs> {
        VDDIO_HSLV_W::new(self, 29)
    }
    ///Bit 31 - Bank swapping option configuration bit
    #[inline(always)]
    pub fn swap_bank_opt(&mut self) -> SWAP_BANK_OPT_W<'_, OPTSR_PRGrs> {
        SWAP_BANK_OPT_W::new(self, 31)
    }
}
/**FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OPTSR_PRG)*/
pub struct OPTSR_PRGrs;
impl crate::RegisterSpec for OPTSR_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`optsr_prg::R`](R) reader structure
impl crate::Readable for OPTSR_PRGrs {}
///`write(|w| ..)` method takes [`optsr_prg::W`](W) writer structure
impl crate::Writable for OPTSR_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTSR_PRG to value 0
impl crate::Resettable for OPTSR_PRGrs {}
