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
///Field `IWDG1_HW` reader - IWDG1 control option status bit
pub type IWDG1_HW_R = crate::BitReader;
///Field `IWDG1_HW` writer - IWDG1 control option status bit
pub type IWDG1_HW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_STOP_D1` reader - D1 DStop entry reset option status bit
pub type N_RST_STOP_D1_R = crate::BitReader;
///Field `nRST_STOP_D1` writer - D1 DStop entry reset option status bit
pub type N_RST_STOP_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_STBY_D1` reader - D1 DStandby entry reset option status bit
pub type N_RST_STBY_D1_R = crate::BitReader;
///Field `nRST_STBY_D1` writer - D1 DStandby entry reset option status bit
pub type N_RST_STBY_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDP` reader - Readout protection level option status byte
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Readout protection level option status byte
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FZ_IWDG_STOP` reader - IWDG Stop mode freeze option status bit
pub type FZ_IWDG_STOP_R = crate::BitReader;
///Field `FZ_IWDG_STOP` writer - IWDG Stop mode freeze option status bit
pub type FZ_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FZ_IWDG_SDBY` reader - IWDG Standby mode freeze option status bit
pub type FZ_IWDG_SDBY_R = crate::BitReader;
///Field `FZ_IWDG_SDBY` writer - IWDG Standby mode freeze option status bit
pub type FZ_IWDG_SDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ST_RAM_SIZE` reader - DTCM RAM size option status
pub type ST_RAM_SIZE_R = crate::FieldReader;
///Field `ST_RAM_SIZE` writer - DTCM RAM size option status
pub type ST_RAM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SECURITY` reader - Security enable option status bit
pub type SECURITY_R = crate::BitReader;
///Field `SECURITY` writer - Security enable option status bit
pub type SECURITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSS1` reader - User option bit 1
pub type RSS1_R = crate::BitReader;
///Field `RSS1` writer - User option bit 1
pub type RSS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERSO_OK` reader - Device personalization status bit
pub type PERSO_OK_R = crate::BitReader;
///Field `PERSO_OK` writer - Device personalization status bit
pub type PERSO_OK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IO_HSLV` reader - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)
pub type IO_HSLV_R = crate::BitReader;
///Field `IO_HSLV` writer - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)
pub type IO_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn iwdg1_hw(&self) -> IWDG1_HW_R {
        IWDG1_HW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - D1 DStop entry reset option status bit
    #[inline(always)]
    pub fn n_rst_stop_d1(&self) -> N_RST_STOP_D1_R {
        N_RST_STOP_D1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - D1 DStandby entry reset option status bit
    #[inline(always)]
    pub fn n_rst_stby_d1(&self) -> N_RST_STBY_D1_R {
        N_RST_STBY_D1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Readout protection level option status byte
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 17 - IWDG Stop mode freeze option status bit
    #[inline(always)]
    pub fn fz_iwdg_stop(&self) -> FZ_IWDG_STOP_R {
        FZ_IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IWDG Standby mode freeze option status bit
    #[inline(always)]
    pub fn fz_iwdg_sdby(&self) -> FZ_IWDG_SDBY_R {
        FZ_IWDG_SDBY_R::new(((self.bits >> 18) & 1) != 0)
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
    ///Bit 26 - User option bit 1
    #[inline(always)]
    pub fn rss1(&self) -> RSS1_R {
        RSS1_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Device personalization status bit
    #[inline(always)]
    pub fn perso_ok(&self) -> PERSO_OK_R {
        PERSO_OK_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new(((self.bits >> 29) & 1) != 0)
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
            .field("iwdg1_hw", &self.iwdg1_hw())
            .field("n_rst_stop_d1", &self.n_rst_stop_d1())
            .field("n_rst_stby_d1", &self.n_rst_stby_d1())
            .field("rdp", &self.rdp())
            .field("fz_iwdg_stop", &self.fz_iwdg_stop())
            .field("fz_iwdg_sdby", &self.fz_iwdg_sdby())
            .field("st_ram_size", &self.st_ram_size())
            .field("security", &self.security())
            .field("rss1", &self.rss1())
            .field("perso_ok", &self.perso_ok())
            .field("io_hslv", &self.io_hslv())
            .field("optchangeerr", &self.optchangeerr())
            .field("swap_bank_opt", &self.swap_bank_opt())
            .finish()
    }
}
impl W {
    ///Bit 0 - Option byte change ongoing flag
    #[inline(always)]
    #[must_use]
    pub fn opt_busy(&mut self) -> OPT_BUSY_W<OPTSR_CUR_rs> {
        OPT_BUSY_W::new(self, 0)
    }
    ///Bits 2:3 - Brownout level option status bit
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<OPTSR_CUR_rs> {
        BOR_LEV_W::new(self, 2)
    }
    ///Bit 4 - IWDG1 control option status bit
    #[inline(always)]
    #[must_use]
    pub fn iwdg1_hw(&mut self) -> IWDG1_HW_W<OPTSR_CUR_rs> {
        IWDG1_HW_W::new(self, 4)
    }
    ///Bit 6 - D1 DStop entry reset option status bit
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop_d1(&mut self) -> N_RST_STOP_D1_W<OPTSR_CUR_rs> {
        N_RST_STOP_D1_W::new(self, 6)
    }
    ///Bit 7 - D1 DStandby entry reset option status bit
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stby_d1(&mut self) -> N_RST_STBY_D1_W<OPTSR_CUR_rs> {
        N_RST_STBY_D1_W::new(self, 7)
    }
    ///Bits 8:15 - Readout protection level option status byte
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<OPTSR_CUR_rs> {
        RDP_W::new(self, 8)
    }
    ///Bit 17 - IWDG Stop mode freeze option status bit
    #[inline(always)]
    #[must_use]
    pub fn fz_iwdg_stop(&mut self) -> FZ_IWDG_STOP_W<OPTSR_CUR_rs> {
        FZ_IWDG_STOP_W::new(self, 17)
    }
    ///Bit 18 - IWDG Standby mode freeze option status bit
    #[inline(always)]
    #[must_use]
    pub fn fz_iwdg_sdby(&mut self) -> FZ_IWDG_SDBY_W<OPTSR_CUR_rs> {
        FZ_IWDG_SDBY_W::new(self, 18)
    }
    ///Bits 19:20 - DTCM RAM size option status
    #[inline(always)]
    #[must_use]
    pub fn st_ram_size(&mut self) -> ST_RAM_SIZE_W<OPTSR_CUR_rs> {
        ST_RAM_SIZE_W::new(self, 19)
    }
    ///Bit 21 - Security enable option status bit
    #[inline(always)]
    #[must_use]
    pub fn security(&mut self) -> SECURITY_W<OPTSR_CUR_rs> {
        SECURITY_W::new(self, 21)
    }
    ///Bit 26 - User option bit 1
    #[inline(always)]
    #[must_use]
    pub fn rss1(&mut self) -> RSS1_W<OPTSR_CUR_rs> {
        RSS1_W::new(self, 26)
    }
    ///Bit 28 - Device personalization status bit
    #[inline(always)]
    #[must_use]
    pub fn perso_ok(&mut self) -> PERSO_OK_W<OPTSR_CUR_rs> {
        PERSO_OK_W::new(self, 28)
    }
    ///Bit 29 - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)
    #[inline(always)]
    #[must_use]
    pub fn io_hslv(&mut self) -> IO_HSLV_W<OPTSR_CUR_rs> {
        IO_HSLV_W::new(self, 29)
    }
    ///Bit 30 - Option byte change error flag
    #[inline(always)]
    #[must_use]
    pub fn optchangeerr(&mut self) -> OPTCHANGEERR_W<OPTSR_CUR_rs> {
        OPTCHANGEERR_W::new(self, 30)
    }
    ///Bit 31 - Bank swapping option status bit
    #[inline(always)]
    #[must_use]
    pub fn swap_bank_opt(&mut self) -> SWAP_BANK_OPT_W<OPTSR_CUR_rs> {
        SWAP_BANK_OPT_W::new(self, 31)
    }
}
/**FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_cur_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_cur_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#FLASH:OPTSR_CUR_)*/
pub struct OPTSR_CUR_rs;
impl crate::RegisterSpec for OPTSR_CUR_rs {
    type Ux = u32;
}
///`read()` method returns [`optsr_cur_::R`](R) reader structure
impl crate::Readable for OPTSR_CUR_rs {}
///`write(|w| ..)` method takes [`optsr_cur_::W`](W) writer structure
impl crate::Writable for OPTSR_CUR_rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OPTSR_CUR_ to value 0
impl crate::Resettable for OPTSR_CUR_rs {
    const RESET_VALUE: u32 = 0;
}
