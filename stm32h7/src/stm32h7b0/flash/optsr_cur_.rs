///Register `OPTSR_CUR_` reader
pub type R = crate::R<OPTSR_CUR_rs>;
///Register `OPTSR_CUR_` writer
pub type W = crate::W<OPTSR_CUR_rs>;
///Field `OPT_BUSY` reader - Option byte change ongoing flag
pub type OPT_BUSY_R = crate::BitReader;
///Field `OPT_BUSY` writer - Option byte change ongoing flag
pub type OPT_BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOR_LEV` reader - Brownout level option status bit
pub type BOR_LEV_R = crate::FieldReader;
///Field `BOR_LEV` writer - Brownout level option status bit
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IWDG_SW` reader - IWDG1 control option status bit
pub type IWDG_SW_R = crate::BitReader;
///Field `IWDG_SW` writer - IWDG1 control option status bit
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STOP` reader - D1 DStop entry reset option status bit
pub type NRST_STOP_R = crate::BitReader;
///Field `NRST_STOP` writer - D1 DStop entry reset option status bit
pub type NRST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STDY` reader - D1 DStandby entry reset option status bit
pub type NRST_STDY_R = crate::BitReader;
///Field `NRST_STDY` writer - D1 DStandby entry reset option status bit
pub type NRST_STDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDP` reader - Readout protection level option status byte
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Readout protection level option status byte
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `VDDMMC_HSLV` reader - IWDG Stop mode freeze option status bit
pub type VDDMMC_HSLV_R = crate::BitReader;
///Field `VDDMMC_HSLV` writer - IWDG Stop mode freeze option status bit
pub type VDDMMC_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDG_FZ_STOP` reader - IWDG Stop mode freeze option status bit
pub type WDG_FZ_STOP_R = crate::BitReader;
///Field `WDG_FZ_STOP` writer - IWDG Stop mode freeze option status bit
pub type WDG_FZ_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_FZ_SDBY` reader - IWDG Standby mode freeze option status bit
pub type IWDG_FZ_SDBY_R = crate::BitReader;
///Field `IWDG_FZ_SDBY` writer - IWDG Standby mode freeze option status bit
pub type IWDG_FZ_SDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ST_RAM_SIZE` reader - DTCM RAM size option status
pub type ST_RAM_SIZE_R = crate::FieldReader;
///Field `ST_RAM_SIZE` writer - DTCM RAM size option status
pub type ST_RAM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SECURITY` reader - Security enable option status bit
pub type SECURITY_R = crate::BitReader;
///Field `SECURITY` writer - Security enable option status bit
pub type SECURITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO_HSLV` reader - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)
pub type VDDIO_HSLV_R = crate::BitReader;
///Field `VDDIO_HSLV` writer - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)
pub type VDDIO_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTCHANGEERR` reader - Option byte change error flag
pub type OPTCHANGEERR_R = crate::BitReader;
///Field `OPTCHANGEERR` writer - Option byte change error flag
pub type OPTCHANGEERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP_BANK_OPT` reader - Bank swapping option status bit
pub type SWAP_BANK_OPT_R = crate::BitReader;
///Field `SWAP_BANK_OPT` writer - Bank swapping option status bit
pub type SWAP_BANK_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Option byte change ongoing flag
    #[inline(always)]
    pub fn opt_busy(&self) -> OPT_BUSY_R {
        OPT_BUSY_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Brownout level option status bit
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - IWDG1 control option status bit
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - D1 DStop entry reset option status bit
    #[inline(always)]
    pub fn nrst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - D1 DStandby entry reset option status bit
    #[inline(always)]
    pub fn nrst_stdy(&self) -> NRST_STDY_R {
        NRST_STDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Readout protection level option status byte
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - IWDG Stop mode freeze option status bit
    #[inline(always)]
    pub fn vddmmc_hslv(&self) -> VDDMMC_HSLV_R {
        VDDMMC_HSLV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - IWDG Stop mode freeze option status bit
    #[inline(always)]
    pub fn wdg_fz_stop(&self) -> WDG_FZ_STOP_R {
        WDG_FZ_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IWDG Standby mode freeze option status bit
    #[inline(always)]
    pub fn iwdg_fz_sdby(&self) -> IWDG_FZ_SDBY_R {
        IWDG_FZ_SDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:20 - DTCM RAM size option status
    #[inline(always)]
    pub fn st_ram_size(&self) -> ST_RAM_SIZE_R {
        ST_RAM_SIZE_R::new(((self.bits >> 19) & 3) as u8)
    }
    ///Bit 21 - Security enable option status bit
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 29 - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)
    #[inline(always)]
    pub fn vddio_hslv(&self) -> VDDIO_HSLV_R {
        VDDIO_HSLV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Option byte change error flag
    #[inline(always)]
    pub fn optchangeerr(&self) -> OPTCHANGEERR_R {
        OPTCHANGEERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Bank swapping option status bit
    #[inline(always)]
    pub fn swap_bank_opt(&self) -> SWAP_BANK_OPT_R {
        SWAP_BANK_OPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTSR_CUR_")
            .field("opt_busy", &self.opt_busy())
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
            .field("optchangeerr", &self.optchangeerr())
            .field("swap_bank_opt", &self.swap_bank_opt())
            .finish()
    }
}
impl W {
    ///Bit 0 - Option byte change ongoing flag
    #[inline(always)]
    pub fn opt_busy(&mut self) -> OPT_BUSY_W<'_, OPTSR_CUR_rs> {
        OPT_BUSY_W::new(self, 0)
    }
    ///Bits 2:3 - Brownout level option status bit
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<'_, OPTSR_CUR_rs> {
        BOR_LEV_W::new(self, 2)
    }
    ///Bit 4 - IWDG1 control option status bit
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<'_, OPTSR_CUR_rs> {
        IWDG_SW_W::new(self, 4)
    }
    ///Bit 6 - D1 DStop entry reset option status bit
    #[inline(always)]
    pub fn nrst_stop(&mut self) -> NRST_STOP_W<'_, OPTSR_CUR_rs> {
        NRST_STOP_W::new(self, 6)
    }
    ///Bit 7 - D1 DStandby entry reset option status bit
    #[inline(always)]
    pub fn nrst_stdy(&mut self) -> NRST_STDY_W<'_, OPTSR_CUR_rs> {
        NRST_STDY_W::new(self, 7)
    }
    ///Bits 8:15 - Readout protection level option status byte
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, OPTSR_CUR_rs> {
        RDP_W::new(self, 8)
    }
    ///Bit 16 - IWDG Stop mode freeze option status bit
    #[inline(always)]
    pub fn vddmmc_hslv(&mut self) -> VDDMMC_HSLV_W<'_, OPTSR_CUR_rs> {
        VDDMMC_HSLV_W::new(self, 16)
    }
    ///Bit 17 - IWDG Stop mode freeze option status bit
    #[inline(always)]
    pub fn wdg_fz_stop(&mut self) -> WDG_FZ_STOP_W<'_, OPTSR_CUR_rs> {
        WDG_FZ_STOP_W::new(self, 17)
    }
    ///Bit 18 - IWDG Standby mode freeze option status bit
    #[inline(always)]
    pub fn iwdg_fz_sdby(&mut self) -> IWDG_FZ_SDBY_W<'_, OPTSR_CUR_rs> {
        IWDG_FZ_SDBY_W::new(self, 18)
    }
    ///Bits 19:20 - DTCM RAM size option status
    #[inline(always)]
    pub fn st_ram_size(&mut self) -> ST_RAM_SIZE_W<'_, OPTSR_CUR_rs> {
        ST_RAM_SIZE_W::new(self, 19)
    }
    ///Bit 21 - Security enable option status bit
    #[inline(always)]
    pub fn security(&mut self) -> SECURITY_W<'_, OPTSR_CUR_rs> {
        SECURITY_W::new(self, 21)
    }
    ///Bit 29 - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)
    #[inline(always)]
    pub fn vddio_hslv(&mut self) -> VDDIO_HSLV_W<'_, OPTSR_CUR_rs> {
        VDDIO_HSLV_W::new(self, 29)
    }
    ///Bit 30 - Option byte change error flag
    #[inline(always)]
    pub fn optchangeerr(&mut self) -> OPTCHANGEERR_W<'_, OPTSR_CUR_rs> {
        OPTCHANGEERR_W::new(self, 30)
    }
    ///Bit 31 - Bank swapping option status bit
    #[inline(always)]
    pub fn swap_bank_opt(&mut self) -> SWAP_BANK_OPT_W<'_, OPTSR_CUR_rs> {
        SWAP_BANK_OPT_W::new(self, 31)
    }
}
/**FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_cur_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_cur_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OPTSR_CUR_)*/
pub struct OPTSR_CUR_rs;
impl crate::RegisterSpec for OPTSR_CUR_rs {
    type Ux = u32;
}
///`read()` method returns [`optsr_cur_::R`](R) reader structure
impl crate::Readable for OPTSR_CUR_rs {}
///`write(|w| ..)` method takes [`optsr_cur_::W`](W) writer structure
impl crate::Writable for OPTSR_CUR_rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTSR_CUR_ to value 0
impl crate::Resettable for OPTSR_CUR_rs {}
