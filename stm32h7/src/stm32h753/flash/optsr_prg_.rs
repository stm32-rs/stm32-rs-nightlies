///Register `OPTSR_PRG_` reader
pub type R = crate::R<OPTSR_PRG_rs>;
///Register `OPTSR_PRG_` writer
pub type W = crate::W<OPTSR_PRG_rs>;
///Field `BOR_LEV` reader - BOR reset level option configuration bits
pub type BOR_LEV_R = crate::FieldReader;
///Field `BOR_LEV` writer - BOR reset level option configuration bits
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IWDG1_SW` reader - IWDG1 option configuration bit
pub type IWDG1_SW_R = crate::BitReader;
///Field `IWDG1_SW` writer - IWDG1 option configuration bit
pub type IWDG1_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_STOP` reader - Option byte erase after D1 DStop option configuration bit
pub type N_RST_STOP_R = crate::BitReader;
///Field `nRST_STOP` writer - Option byte erase after D1 DStop option configuration bit
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_STDY` reader - Option byte erase after D1 DStandby option configuration bit
pub type N_RST_STDY_R = crate::BitReader;
///Field `nRST_STDY` writer - Option byte erase after D1 DStandby option configuration bit
pub type N_RST_STDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDP` reader - Readout protection level option configuration byte
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Readout protection level option configuration byte
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FZ_IWDG_STOP` reader - IWDG Stop mode freeze option configuration bit
pub type FZ_IWDG_STOP_R = crate::BitReader;
///Field `FZ_IWDG_STOP` writer - IWDG Stop mode freeze option configuration bit
pub type FZ_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FZ_IWDG_SDBY` reader - IWDG Standby mode freeze option configuration bit
pub type FZ_IWDG_SDBY_R = crate::BitReader;
///Field `FZ_IWDG_SDBY` writer - IWDG Standby mode freeze option configuration bit
pub type FZ_IWDG_SDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ST_RAM_SIZE` reader - DTCM size select option configuration bits
pub type ST_RAM_SIZE_R = crate::FieldReader;
///Field `ST_RAM_SIZE` writer - DTCM size select option configuration bits
pub type ST_RAM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SECURITY` reader - Security option configuration bit
pub type SECURITY_R = crate::BitReader;
///Field `SECURITY` writer - Security option configuration bit
pub type SECURITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IO_HSLV` reader - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)
pub type IO_HSLV_R = crate::BitReader;
///Field `IO_HSLV` writer - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)
pub type IO_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn iwdg1_sw(&self) -> IWDG1_SW_R {
        IWDG1_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Option byte erase after D1 DStop option configuration bit
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Option byte erase after D1 DStandby option configuration bit
    #[inline(always)]
    pub fn n_rst_stdy(&self) -> N_RST_STDY_R {
        N_RST_STDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Readout protection level option configuration byte
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 17 - IWDG Stop mode freeze option configuration bit
    #[inline(always)]
    pub fn fz_iwdg_stop(&self) -> FZ_IWDG_STOP_R {
        FZ_IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IWDG Standby mode freeze option configuration bit
    #[inline(always)]
    pub fn fz_iwdg_sdby(&self) -> FZ_IWDG_SDBY_R {
        FZ_IWDG_SDBY_R::new(((self.bits >> 18) & 1) != 0)
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
    ///Bit 29 - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Bank swapping option configuration bit
    #[inline(always)]
    pub fn swap_bank_opt(&self) -> SWAP_BANK_OPT_R {
        SWAP_BANK_OPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTSR_PRG_")
            .field("bor_lev", &self.bor_lev())
            .field("iwdg1_sw", &self.iwdg1_sw())
            .field("n_rst_stop", &self.n_rst_stop())
            .field("n_rst_stdy", &self.n_rst_stdy())
            .field("rdp", &self.rdp())
            .field("fz_iwdg_stop", &self.fz_iwdg_stop())
            .field("fz_iwdg_sdby", &self.fz_iwdg_sdby())
            .field("st_ram_size", &self.st_ram_size())
            .field("security", &self.security())
            .field("io_hslv", &self.io_hslv())
            .field("swap_bank_opt", &self.swap_bank_opt())
            .finish()
    }
}
impl W {
    ///Bits 2:3 - BOR reset level option configuration bits
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<'_, OPTSR_PRG_rs> {
        BOR_LEV_W::new(self, 2)
    }
    ///Bit 4 - IWDG1 option configuration bit
    #[inline(always)]
    pub fn iwdg1_sw(&mut self) -> IWDG1_SW_W<'_, OPTSR_PRG_rs> {
        IWDG1_SW_W::new(self, 4)
    }
    ///Bit 6 - Option byte erase after D1 DStop option configuration bit
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<'_, OPTSR_PRG_rs> {
        N_RST_STOP_W::new(self, 6)
    }
    ///Bit 7 - Option byte erase after D1 DStandby option configuration bit
    #[inline(always)]
    pub fn n_rst_stdy(&mut self) -> N_RST_STDY_W<'_, OPTSR_PRG_rs> {
        N_RST_STDY_W::new(self, 7)
    }
    ///Bits 8:15 - Readout protection level option configuration byte
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, OPTSR_PRG_rs> {
        RDP_W::new(self, 8)
    }
    ///Bit 17 - IWDG Stop mode freeze option configuration bit
    #[inline(always)]
    pub fn fz_iwdg_stop(&mut self) -> FZ_IWDG_STOP_W<'_, OPTSR_PRG_rs> {
        FZ_IWDG_STOP_W::new(self, 17)
    }
    ///Bit 18 - IWDG Standby mode freeze option configuration bit
    #[inline(always)]
    pub fn fz_iwdg_sdby(&mut self) -> FZ_IWDG_SDBY_W<'_, OPTSR_PRG_rs> {
        FZ_IWDG_SDBY_W::new(self, 18)
    }
    ///Bits 19:20 - DTCM size select option configuration bits
    #[inline(always)]
    pub fn st_ram_size(&mut self) -> ST_RAM_SIZE_W<'_, OPTSR_PRG_rs> {
        ST_RAM_SIZE_W::new(self, 19)
    }
    ///Bit 21 - Security option configuration bit
    #[inline(always)]
    pub fn security(&mut self) -> SECURITY_W<'_, OPTSR_PRG_rs> {
        SECURITY_W::new(self, 21)
    }
    ///Bit 29 - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)
    #[inline(always)]
    pub fn io_hslv(&mut self) -> IO_HSLV_W<'_, OPTSR_PRG_rs> {
        IO_HSLV_W::new(self, 29)
    }
    ///Bit 31 - Bank swapping option configuration bit
    #[inline(always)]
    pub fn swap_bank_opt(&mut self) -> SWAP_BANK_OPT_W<'_, OPTSR_PRG_rs> {
        SWAP_BANK_OPT_W::new(self, 31)
    }
}
/**FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_prg_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_prg_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#FLASH:OPTSR_PRG_)*/
pub struct OPTSR_PRG_rs;
impl crate::RegisterSpec for OPTSR_PRG_rs {
    type Ux = u32;
}
///`read()` method returns [`optsr_prg_::R`](R) reader structure
impl crate::Readable for OPTSR_PRG_rs {}
///`write(|w| ..)` method takes [`optsr_prg_::W`](W) writer structure
impl crate::Writable for OPTSR_PRG_rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTSR_PRG_ to value 0
impl crate::Resettable for OPTSR_PRG_rs {}
