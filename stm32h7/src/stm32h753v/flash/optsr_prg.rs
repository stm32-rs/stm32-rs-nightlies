#[doc = "Register `OPTSR_PRG` reader"]
pub type R = crate::R<OPTSR_PRGrs>;
#[doc = "Register `OPTSR_PRG` writer"]
pub type W = crate::W<OPTSR_PRGrs>;
#[doc = "Field `BOR_LEV` reader - BOR reset level option configuration bits"]
pub type BOR_LEV_R = crate::FieldReader;
#[doc = "Field `BOR_LEV` writer - BOR reset level option configuration bits"]
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IWDG1_HW` reader - IWDG1 option configuration bit"]
pub type IWDG1_HW_R = crate::BitReader;
#[doc = "Field `IWDG1_HW` writer - IWDG1 option configuration bit"]
pub type IWDG1_HW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STOP_D1` reader - Option byte erase after D1 DStop option configuration bit"]
pub type N_RST_STOP_D1_R = crate::BitReader;
#[doc = "Field `nRST_STOP_D1` writer - Option byte erase after D1 DStop option configuration bit"]
pub type N_RST_STOP_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STBY_D1` reader - Option byte erase after D1 DStandby option configuration bit"]
pub type N_RST_STBY_D1_R = crate::BitReader;
#[doc = "Field `nRST_STBY_D1` writer - Option byte erase after D1 DStandby option configuration bit"]
pub type N_RST_STBY_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP` reader - Readout protection level option configuration byte"]
pub type RDP_R = crate::FieldReader;
#[doc = "Field `RDP` writer - Readout protection level option configuration byte"]
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FZ_IWDG_STOP` reader - IWDG Stop mode freeze option configuration bit"]
pub type FZ_IWDG_STOP_R = crate::BitReader;
#[doc = "Field `FZ_IWDG_STOP` writer - IWDG Stop mode freeze option configuration bit"]
pub type FZ_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FZ_IWDG_SDBY` reader - IWDG Standby mode freeze option configuration bit"]
pub type FZ_IWDG_SDBY_R = crate::BitReader;
#[doc = "Field `FZ_IWDG_SDBY` writer - IWDG Standby mode freeze option configuration bit"]
pub type FZ_IWDG_SDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST_RAM_SIZE` reader - DTCM size select option configuration bits"]
pub type ST_RAM_SIZE_R = crate::FieldReader;
#[doc = "Field `ST_RAM_SIZE` writer - DTCM size select option configuration bits"]
pub type ST_RAM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SECURITY` reader - Security option configuration bit"]
pub type SECURITY_R = crate::BitReader;
#[doc = "Field `SECURITY` writer - Security option configuration bit"]
pub type SECURITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSS1` reader - User option configuration bit 1"]
pub type RSS1_R = crate::BitReader;
#[doc = "Field `RSS1` writer - User option configuration bit 1"]
pub type RSS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSS2` reader - User option configuration bit 2"]
pub type RSS2_R = crate::BitReader;
#[doc = "Field `RSS2` writer - User option configuration bit 2"]
pub type RSS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_HSLV` reader - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)"]
pub type IO_HSLV_R = crate::BitReader;
#[doc = "Field `IO_HSLV` writer - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)"]
pub type IO_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP_BANK_OPT` reader - Bank swapping option configuration bit"]
pub type SWAP_BANK_OPT_R = crate::BitReader;
#[doc = "Field `SWAP_BANK_OPT` writer - Bank swapping option configuration bit"]
pub type SWAP_BANK_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:3 - BOR reset level option configuration bits"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - IWDG1 option configuration bit"]
    #[inline(always)]
    pub fn iwdg1_hw(&self) -> IWDG1_HW_R {
        IWDG1_HW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Option byte erase after D1 DStop option configuration bit"]
    #[inline(always)]
    pub fn n_rst_stop_d1(&self) -> N_RST_STOP_D1_R {
        N_RST_STOP_D1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Option byte erase after D1 DStandby option configuration bit"]
    #[inline(always)]
    pub fn n_rst_stby_d1(&self) -> N_RST_STBY_D1_R {
        N_RST_STBY_D1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Readout protection level option configuration byte"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option configuration bit"]
    #[inline(always)]
    pub fn fz_iwdg_stop(&self) -> FZ_IWDG_STOP_R {
        FZ_IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option configuration bit"]
    #[inline(always)]
    pub fn fz_iwdg_sdby(&self) -> FZ_IWDG_SDBY_R {
        FZ_IWDG_SDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - DTCM size select option configuration bits"]
    #[inline(always)]
    pub fn st_ram_size(&self) -> ST_RAM_SIZE_R {
        ST_RAM_SIZE_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - Security option configuration bit"]
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - User option configuration bit 1"]
    #[inline(always)]
    pub fn rss1(&self) -> RSS1_R {
        RSS1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - User option configuration bit 2"]
    #[inline(always)]
    pub fn rss2(&self) -> RSS2_R {
        RSS2_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank swapping option configuration bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&self) -> SWAP_BANK_OPT_R {
        SWAP_BANK_OPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - BOR reset level option configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<OPTSR_PRGrs> {
        BOR_LEV_W::new(self, 2)
    }
    #[doc = "Bit 4 - IWDG1 option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg1_hw(&mut self) -> IWDG1_HW_W<OPTSR_PRGrs> {
        IWDG1_HW_W::new(self, 4)
    }
    #[doc = "Bit 6 - Option byte erase after D1 DStop option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop_d1(&mut self) -> N_RST_STOP_D1_W<OPTSR_PRGrs> {
        N_RST_STOP_D1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Option byte erase after D1 DStandby option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stby_d1(&mut self) -> N_RST_STBY_D1_W<OPTSR_PRGrs> {
        N_RST_STBY_D1_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Readout protection level option configuration byte"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<OPTSR_PRGrs> {
        RDP_W::new(self, 8)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn fz_iwdg_stop(&mut self) -> FZ_IWDG_STOP_W<OPTSR_PRGrs> {
        FZ_IWDG_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn fz_iwdg_sdby(&mut self) -> FZ_IWDG_SDBY_W<OPTSR_PRGrs> {
        FZ_IWDG_SDBY_W::new(self, 18)
    }
    #[doc = "Bits 19:20 - DTCM size select option configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn st_ram_size(&mut self) -> ST_RAM_SIZE_W<OPTSR_PRGrs> {
        ST_RAM_SIZE_W::new(self, 19)
    }
    #[doc = "Bit 21 - Security option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn security(&mut self) -> SECURITY_W<OPTSR_PRGrs> {
        SECURITY_W::new(self, 21)
    }
    #[doc = "Bit 26 - User option configuration bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rss1(&mut self) -> RSS1_W<OPTSR_PRGrs> {
        RSS1_W::new(self, 26)
    }
    #[doc = "Bit 27 - User option configuration bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rss2(&mut self) -> RSS2_W<OPTSR_PRGrs> {
        RSS2_W::new(self, 27)
    }
    #[doc = "Bit 29 - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)"]
    #[inline(always)]
    #[must_use]
    pub fn io_hslv(&mut self) -> IO_HSLV_W<OPTSR_PRGrs> {
        IO_HSLV_W::new(self, 29)
    }
    #[doc = "Bit 31 - Bank swapping option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn swap_bank_opt(&mut self) -> SWAP_BANK_OPT_W<OPTSR_PRGrs> {
        SWAP_BANK_OPT_W::new(self, 31)
    }
}
#[doc = "FLASH option status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr_prg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr_prg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTSR_PRGrs;
impl crate::RegisterSpec for OPTSR_PRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optsr_prg::R`](R) reader structure"]
impl crate::Readable for OPTSR_PRGrs {}
#[doc = "`write(|w| ..)` method takes [`optsr_prg::W`](W) writer structure"]
impl crate::Writable for OPTSR_PRGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTSR_PRG to value 0"]
impl crate::Resettable for OPTSR_PRGrs {
    const RESET_VALUE: u32 = 0;
}
