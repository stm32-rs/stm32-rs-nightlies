#[doc = "Register `OPTSR_CUR` reader"]
pub type R = crate::R<OPTSR_CURrs>;
#[doc = "Register `OPTSR_CUR` writer"]
pub type W = crate::W<OPTSR_CURrs>;
#[doc = "Field `OPT_BUSY` reader - Option byte change ongoing flag"]
pub type OPT_BUSY_R = crate::BitReader;
#[doc = "Field `OPT_BUSY` writer - Option byte change ongoing flag"]
pub type OPT_BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOR_LEV` reader - Brownout level option status bit"]
pub type BOR_LEV_R = crate::FieldReader;
#[doc = "Field `BOR_LEV` writer - Brownout level option status bit"]
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IWDG_SW` reader - IWDG control mode option status bit"]
pub type IWDG_SW_R = crate::BitReader;
#[doc = "Field `IWDG_SW` writer - IWDG control mode option status bit"]
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG2_SW` reader - IWDG2 control mode option status bit"]
pub type IWDG2_SW_R = crate::BitReader;
#[doc = "Field `IWDG2_SW` writer - IWDG2 control mode option status bit"]
pub type IWDG2_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRST_STOP_D1` reader - D1 domain DStop entry reset option status bit"]
pub type NRST_STOP_D1_R = crate::BitReader;
#[doc = "Field `NRST_STOP_D1` writer - D1 domain DStop entry reset option status bit"]
pub type NRST_STOP_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_STDY_D1` reader - D1 domain DStandby entry reset option status bit"]
pub type RST_STDY_D1_R = crate::BitReader;
#[doc = "Field `RST_STDY_D1` writer - D1 domain DStandby entry reset option status bit"]
pub type RST_STDY_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP` reader - Readout protection level option status byte"]
pub type RDP_R = crate::FieldReader;
#[doc = "Field `RDP` writer - Readout protection level option status byte"]
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IWDG_FZ_STOP` reader - IWDG Stop mode freeze option status bit"]
pub type IWDG_FZ_STOP_R = crate::BitReader;
#[doc = "Field `IWDG_FZ_STOP` writer - IWDG Stop mode freeze option status bit"]
pub type IWDG_FZ_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_FZ_SDBY` reader - IWDG Standby mode freeze option status bit"]
pub type IWDG_FZ_SDBY_R = crate::BitReader;
#[doc = "Field `IWDG_FZ_SDBY` writer - IWDG Standby mode freeze option status bit"]
pub type IWDG_FZ_SDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST_RAM_SIZE` reader - ST RAM size option status"]
pub type ST_RAM_SIZE_R = crate::FieldReader;
#[doc = "Field `ST_RAM_SIZE` writer - ST RAM size option status"]
pub type ST_RAM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SECURITY` reader - Security enable option status bit"]
pub type SECURITY_R = crate::BitReader;
#[doc = "Field `SECURITY` writer - Security enable option status bit"]
pub type SECURITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_CM4` reader - Arm Cortex"]
pub type BOOT_CM4_R = crate::BitReader;
#[doc = "Field `BOOT_CM4` writer - Arm Cortex"]
pub type BOOT_CM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_CM7` reader - Arm Cortex"]
pub type BOOT_CM7_R = crate::BitReader;
#[doc = "Field `BOOT_CM7` writer - Arm Cortex"]
pub type BOOT_CM7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRST_STOP_D2` reader - D2 domain DStop entry reset option status bit"]
pub type NRST_STOP_D2_R = crate::BitReader;
#[doc = "Field `NRST_STOP_D2` writer - D2 domain DStop entry reset option status bit"]
pub type NRST_STOP_D2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRST_STBY_D2` reader - D2 domain DStandby entry reset option status bit"]
pub type NRST_STBY_D2_R = crate::BitReader;
#[doc = "Field `NRST_STBY_D2` writer - D2 domain DStandby entry reset option status bit"]
pub type NRST_STBY_D2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_HSLV` reader - I"]
pub type IO_HSLV_R = crate::BitReader;
#[doc = "Field `IO_HSLV` writer - I"]
pub type IO_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTCHANGEERR` reader - Option byte change error flag"]
pub type OPTCHANGEERR_R = crate::BitReader;
#[doc = "Field `OPTCHANGEERR` writer - Option byte change error flag"]
pub type OPTCHANGEERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP_BANK_OPT` reader - Bank swapping option status bit"]
pub type SWAP_BANK_OPT_R = crate::BitReader;
#[doc = "Field `SWAP_BANK_OPT` writer - Bank swapping option status bit"]
pub type SWAP_BANK_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Option byte change ongoing flag"]
    #[inline(always)]
    pub fn opt_busy(&self) -> OPT_BUSY_R {
        OPT_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Brownout level option status bit"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - IWDG control mode option status bit"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IWDG2 control mode option status bit"]
    #[inline(always)]
    pub fn iwdg2_sw(&self) -> IWDG2_SW_R {
        IWDG2_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - D1 domain DStop entry reset option status bit"]
    #[inline(always)]
    pub fn nrst_stop_d1(&self) -> NRST_STOP_D1_R {
        NRST_STOP_D1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - D1 domain DStandby entry reset option status bit"]
    #[inline(always)]
    pub fn rst_stdy_d1(&self) -> RST_STDY_D1_R {
        RST_STDY_D1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Readout protection level option status byte"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option status bit"]
    #[inline(always)]
    pub fn iwdg_fz_stop(&self) -> IWDG_FZ_STOP_R {
        IWDG_FZ_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option status bit"]
    #[inline(always)]
    pub fn iwdg_fz_sdby(&self) -> IWDG_FZ_SDBY_R {
        IWDG_FZ_SDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - ST RAM size option status"]
    #[inline(always)]
    pub fn st_ram_size(&self) -> ST_RAM_SIZE_R {
        ST_RAM_SIZE_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - Security enable option status bit"]
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Arm Cortex"]
    #[inline(always)]
    pub fn boot_cm4(&self) -> BOOT_CM4_R {
        BOOT_CM4_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Arm Cortex"]
    #[inline(always)]
    pub fn boot_cm7(&self) -> BOOT_CM7_R {
        BOOT_CM7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - D2 domain DStop entry reset option status bit"]
    #[inline(always)]
    pub fn nrst_stop_d2(&self) -> NRST_STOP_D2_R {
        NRST_STOP_D2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - D2 domain DStandby entry reset option status bit"]
    #[inline(always)]
    pub fn nrst_stby_d2(&self) -> NRST_STBY_D2_R {
        NRST_STBY_D2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - I"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Option byte change error flag"]
    #[inline(always)]
    pub fn optchangeerr(&self) -> OPTCHANGEERR_R {
        OPTCHANGEERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank swapping option status bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&self) -> SWAP_BANK_OPT_R {
        SWAP_BANK_OPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option byte change ongoing flag"]
    #[inline(always)]
    #[must_use]
    pub fn opt_busy(&mut self) -> OPT_BUSY_W<OPTSR_CURrs> {
        OPT_BUSY_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Brownout level option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<OPTSR_CURrs> {
        BOR_LEV_W::new(self, 2)
    }
    #[doc = "Bit 4 - IWDG control mode option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<OPTSR_CURrs> {
        IWDG_SW_W::new(self, 4)
    }
    #[doc = "Bit 5 - IWDG2 control mode option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg2_sw(&mut self) -> IWDG2_SW_W<OPTSR_CURrs> {
        IWDG2_SW_W::new(self, 5)
    }
    #[doc = "Bit 6 - D1 domain DStop entry reset option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_stop_d1(&mut self) -> NRST_STOP_D1_W<OPTSR_CURrs> {
        NRST_STOP_D1_W::new(self, 6)
    }
    #[doc = "Bit 7 - D1 domain DStandby entry reset option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn rst_stdy_d1(&mut self) -> RST_STDY_D1_W<OPTSR_CURrs> {
        RST_STDY_D1_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Readout protection level option status byte"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<OPTSR_CURrs> {
        RDP_W::new(self, 8)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_fz_stop(&mut self) -> IWDG_FZ_STOP_W<OPTSR_CURrs> {
        IWDG_FZ_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_fz_sdby(&mut self) -> IWDG_FZ_SDBY_W<OPTSR_CURrs> {
        IWDG_FZ_SDBY_W::new(self, 18)
    }
    #[doc = "Bits 19:20 - ST RAM size option status"]
    #[inline(always)]
    #[must_use]
    pub fn st_ram_size(&mut self) -> ST_RAM_SIZE_W<OPTSR_CURrs> {
        ST_RAM_SIZE_W::new(self, 19)
    }
    #[doc = "Bit 21 - Security enable option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn security(&mut self) -> SECURITY_W<OPTSR_CURrs> {
        SECURITY_W::new(self, 21)
    }
    #[doc = "Bit 22 - Arm Cortex"]
    #[inline(always)]
    #[must_use]
    pub fn boot_cm4(&mut self) -> BOOT_CM4_W<OPTSR_CURrs> {
        BOOT_CM4_W::new(self, 22)
    }
    #[doc = "Bit 23 - Arm Cortex"]
    #[inline(always)]
    #[must_use]
    pub fn boot_cm7(&mut self) -> BOOT_CM7_W<OPTSR_CURrs> {
        BOOT_CM7_W::new(self, 23)
    }
    #[doc = "Bit 24 - D2 domain DStop entry reset option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_stop_d2(&mut self) -> NRST_STOP_D2_W<OPTSR_CURrs> {
        NRST_STOP_D2_W::new(self, 24)
    }
    #[doc = "Bit 25 - D2 domain DStandby entry reset option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_stby_d2(&mut self) -> NRST_STBY_D2_W<OPTSR_CURrs> {
        NRST_STBY_D2_W::new(self, 25)
    }
    #[doc = "Bit 29 - I"]
    #[inline(always)]
    #[must_use]
    pub fn io_hslv(&mut self) -> IO_HSLV_W<OPTSR_CURrs> {
        IO_HSLV_W::new(self, 29)
    }
    #[doc = "Bit 30 - Option byte change error flag"]
    #[inline(always)]
    #[must_use]
    pub fn optchangeerr(&mut self) -> OPTCHANGEERR_W<OPTSR_CURrs> {
        OPTCHANGEERR_W::new(self, 30)
    }
    #[doc = "Bit 31 - Bank swapping option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn swap_bank_opt(&mut self) -> SWAP_BANK_OPT_W<OPTSR_CURrs> {
        SWAP_BANK_OPT_W::new(self, 31)
    }
}
#[doc = "FLASH option status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr_cur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr_cur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTSR_CURrs;
impl crate::RegisterSpec for OPTSR_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optsr_cur::R`](R) reader structure"]
impl crate::Readable for OPTSR_CURrs {}
#[doc = "`write(|w| ..)` method takes [`optsr_cur::W`](W) writer structure"]
impl crate::Writable for OPTSR_CURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTSR_CUR to value 0"]
impl crate::Resettable for OPTSR_CURrs {
    const RESET_VALUE: u32 = 0;
}
