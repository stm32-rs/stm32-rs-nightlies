///Register `OPTSR_PRG` reader
pub type R = crate::R<OPTSR_PRGrs>;
///Register `OPTSR_PRG` writer
pub type W = crate::W<OPTSR_PRGrs>;
///Field `BOR_LEV` reader - Brownout level option configuration bit
pub type BOR_LEV_R = crate::FieldReader;
///Field `BOR_LEV` writer - Brownout level option configuration bit
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IWDG_SW` reader - IWDG control mode option configuration bit
pub type IWDG_SW_R = crate::BitReader;
///Field `IWDG_SW` writer - IWDG control mode option configuration bit
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG2_SW` reader - IWDG2 control mode option configuration bit
pub type IWDG2_SW_R = crate::BitReader;
///Field `IWDG2_SW` writer - IWDG2 control mode option configuration bit
pub type IWDG2_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STOP_D1` reader - D1 domain DStop entry reset option configuration bit
pub type NRST_STOP_D1_R = crate::BitReader;
///Field `NRST_STOP_D1` writer - D1 domain DStop entry reset option configuration bit
pub type NRST_STOP_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STDY_D1` reader - D1 domain DStandby entry reset option configuration bit
pub type NRST_STDY_D1_R = crate::BitReader;
///Field `NRST_STDY_D1` writer - D1 domain DStandby entry reset option configuration bit
pub type NRST_STDY_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDP` reader - Readout protection level option configuration bits
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Readout protection level option configuration bits
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IWDG_FZ_STOP` reader - IWDG Stop mode freeze option configuration bit
pub type IWDG_FZ_STOP_R = crate::BitReader;
///Field `IWDG_FZ_STOP` writer - IWDG Stop mode freeze option configuration bit
pub type IWDG_FZ_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_FZ_SDBY` reader - IWDG Standby mode freeze option configuration bit
pub type IWDG_FZ_SDBY_R = crate::BitReader;
///Field `IWDG_FZ_SDBY` writer - IWDG Standby mode freeze option configuration bit
pub type IWDG_FZ_SDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ST_RAM_SIZE` reader - ST RAM size option configuration bits
pub type ST_RAM_SIZE_R = crate::FieldReader;
///Field `ST_RAM_SIZE` writer - ST RAM size option configuration bits
pub type ST_RAM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SECURITY` reader - Security enable option configuration bit
pub type SECURITY_R = crate::BitReader;
///Field `SECURITY` writer - Security enable option configuration bit
pub type SECURITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOT_CM4` reader - Arm Cortex
pub type BOOT_CM4_R = crate::BitReader;
///Field `BOOT_CM4` writer - Arm Cortex
pub type BOOT_CM4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOT_CM7` reader - Arm Cortex
pub type BOOT_CM7_R = crate::BitReader;
///Field `BOOT_CM7` writer - Arm Cortex
pub type BOOT_CM7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STOP_D2` reader - D2 domain DStop entry reset option configuration bit
pub type NRST_STOP_D2_R = crate::BitReader;
///Field `NRST_STOP_D2` writer - D2 domain DStop entry reset option configuration bit
pub type NRST_STOP_D2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STBY_D2` reader - D2 domain DStandby entry reset option configuration bit
pub type NRST_STBY_D2_R = crate::BitReader;
///Field `NRST_STBY_D2` writer - D2 domain DStandby entry reset option configuration bit
pub type NRST_STBY_D2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IO_HSLV` reader - I
pub type IO_HSLV_R = crate::BitReader;
///Field `IO_HSLV` writer - I
pub type IO_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP_BANK_OPT` reader - Bank swapping option configuration bit
pub type SWAP_BANK_OPT_R = crate::BitReader;
///Field `SWAP_BANK_OPT` writer - Bank swapping option configuration bit
pub type SWAP_BANK_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 2:3 - Brownout level option configuration bit
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - IWDG control mode option configuration bit
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IWDG2 control mode option configuration bit
    #[inline(always)]
    pub fn iwdg2_sw(&self) -> IWDG2_SW_R {
        IWDG2_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - D1 domain DStop entry reset option configuration bit
    #[inline(always)]
    pub fn nrst_stop_d1(&self) -> NRST_STOP_D1_R {
        NRST_STOP_D1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - D1 domain DStandby entry reset option configuration bit
    #[inline(always)]
    pub fn nrst_stdy_d1(&self) -> NRST_STDY_D1_R {
        NRST_STDY_D1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Readout protection level option configuration bits
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 17 - IWDG Stop mode freeze option configuration bit
    #[inline(always)]
    pub fn iwdg_fz_stop(&self) -> IWDG_FZ_STOP_R {
        IWDG_FZ_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IWDG Standby mode freeze option configuration bit
    #[inline(always)]
    pub fn iwdg_fz_sdby(&self) -> IWDG_FZ_SDBY_R {
        IWDG_FZ_SDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:20 - ST RAM size option configuration bits
    #[inline(always)]
    pub fn st_ram_size(&self) -> ST_RAM_SIZE_R {
        ST_RAM_SIZE_R::new(((self.bits >> 19) & 3) as u8)
    }
    ///Bit 21 - Security enable option configuration bit
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Arm Cortex
    #[inline(always)]
    pub fn boot_cm4(&self) -> BOOT_CM4_R {
        BOOT_CM4_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Arm Cortex
    #[inline(always)]
    pub fn boot_cm7(&self) -> BOOT_CM7_R {
        BOOT_CM7_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - D2 domain DStop entry reset option configuration bit
    #[inline(always)]
    pub fn nrst_stop_d2(&self) -> NRST_STOP_D2_R {
        NRST_STOP_D2_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - D2 domain DStandby entry reset option configuration bit
    #[inline(always)]
    pub fn nrst_stby_d2(&self) -> NRST_STBY_D2_R {
        NRST_STBY_D2_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 29 - I
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
        f.debug_struct("OPTSR_PRG")
            .field("swap_bank_opt", &self.swap_bank_opt())
            .field("io_hslv", &self.io_hslv())
            .field("nrst_stby_d2", &self.nrst_stby_d2())
            .field("nrst_stop_d2", &self.nrst_stop_d2())
            .field("boot_cm7", &self.boot_cm7())
            .field("boot_cm4", &self.boot_cm4())
            .field("security", &self.security())
            .field("st_ram_size", &self.st_ram_size())
            .field("iwdg_fz_sdby", &self.iwdg_fz_sdby())
            .field("iwdg_fz_stop", &self.iwdg_fz_stop())
            .field("rdp", &self.rdp())
            .field("nrst_stdy_d1", &self.nrst_stdy_d1())
            .field("nrst_stop_d1", &self.nrst_stop_d1())
            .field("iwdg2_sw", &self.iwdg2_sw())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("bor_lev", &self.bor_lev())
            .finish()
    }
}
impl W {
    ///Bits 2:3 - Brownout level option configuration bit
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<'_, OPTSR_PRGrs> {
        BOR_LEV_W::new(self, 2)
    }
    ///Bit 4 - IWDG control mode option configuration bit
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<'_, OPTSR_PRGrs> {
        IWDG_SW_W::new(self, 4)
    }
    ///Bit 5 - IWDG2 control mode option configuration bit
    #[inline(always)]
    pub fn iwdg2_sw(&mut self) -> IWDG2_SW_W<'_, OPTSR_PRGrs> {
        IWDG2_SW_W::new(self, 5)
    }
    ///Bit 6 - D1 domain DStop entry reset option configuration bit
    #[inline(always)]
    pub fn nrst_stop_d1(&mut self) -> NRST_STOP_D1_W<'_, OPTSR_PRGrs> {
        NRST_STOP_D1_W::new(self, 6)
    }
    ///Bit 7 - D1 domain DStandby entry reset option configuration bit
    #[inline(always)]
    pub fn nrst_stdy_d1(&mut self) -> NRST_STDY_D1_W<'_, OPTSR_PRGrs> {
        NRST_STDY_D1_W::new(self, 7)
    }
    ///Bits 8:15 - Readout protection level option configuration bits
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, OPTSR_PRGrs> {
        RDP_W::new(self, 8)
    }
    ///Bit 17 - IWDG Stop mode freeze option configuration bit
    #[inline(always)]
    pub fn iwdg_fz_stop(&mut self) -> IWDG_FZ_STOP_W<'_, OPTSR_PRGrs> {
        IWDG_FZ_STOP_W::new(self, 17)
    }
    ///Bit 18 - IWDG Standby mode freeze option configuration bit
    #[inline(always)]
    pub fn iwdg_fz_sdby(&mut self) -> IWDG_FZ_SDBY_W<'_, OPTSR_PRGrs> {
        IWDG_FZ_SDBY_W::new(self, 18)
    }
    ///Bits 19:20 - ST RAM size option configuration bits
    #[inline(always)]
    pub fn st_ram_size(&mut self) -> ST_RAM_SIZE_W<'_, OPTSR_PRGrs> {
        ST_RAM_SIZE_W::new(self, 19)
    }
    ///Bit 21 - Security enable option configuration bit
    #[inline(always)]
    pub fn security(&mut self) -> SECURITY_W<'_, OPTSR_PRGrs> {
        SECURITY_W::new(self, 21)
    }
    ///Bit 22 - Arm Cortex
    #[inline(always)]
    pub fn boot_cm4(&mut self) -> BOOT_CM4_W<'_, OPTSR_PRGrs> {
        BOOT_CM4_W::new(self, 22)
    }
    ///Bit 23 - Arm Cortex
    #[inline(always)]
    pub fn boot_cm7(&mut self) -> BOOT_CM7_W<'_, OPTSR_PRGrs> {
        BOOT_CM7_W::new(self, 23)
    }
    ///Bit 24 - D2 domain DStop entry reset option configuration bit
    #[inline(always)]
    pub fn nrst_stop_d2(&mut self) -> NRST_STOP_D2_W<'_, OPTSR_PRGrs> {
        NRST_STOP_D2_W::new(self, 24)
    }
    ///Bit 25 - D2 domain DStandby entry reset option configuration bit
    #[inline(always)]
    pub fn nrst_stby_d2(&mut self) -> NRST_STBY_D2_W<'_, OPTSR_PRGrs> {
        NRST_STBY_D2_W::new(self, 25)
    }
    ///Bit 29 - I
    #[inline(always)]
    pub fn io_hslv(&mut self) -> IO_HSLV_W<'_, OPTSR_PRGrs> {
        IO_HSLV_W::new(self, 29)
    }
    ///Bit 31 - Bank swapping option configuration bit
    #[inline(always)]
    pub fn swap_bank_opt(&mut self) -> SWAP_BANK_OPT_W<'_, OPTSR_PRGrs> {
        SWAP_BANK_OPT_W::new(self, 31)
    }
}
/**FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#FLASH:OPTSR_PRG)*/
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
