#[doc = "Register `FLASH_OPTR` reader"]
pub type R = crate::R<FLASH_OPTRrs>;
#[doc = "Register `FLASH_OPTR` writer"]
pub type W = crate::W<FLASH_OPTRrs>;
#[doc = "Field `RDP` reader - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details."]
pub type RDP_R = crate::FieldReader;
#[doc = "Field `RDP` writer - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details."]
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BOR_LEV` reader - BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset."]
pub type BOR_LEV_R = crate::FieldReader;
#[doc = "Field `BOR_LEV` writer - BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset."]
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `nRST_STOP` reader - Reset generation in Stop mode"]
pub type N_RST_STOP_R = crate::BitReader;
#[doc = "Field `nRST_STOP` writer - Reset generation in Stop mode"]
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STDBY` reader - Reset generation in Standby mode"]
pub type N_RST_STDBY_R = crate::BitReader;
#[doc = "Field `nRST_STDBY` writer - Reset generation in Standby mode"]
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_SHDW` reader - Reset generation in Shutdown mode"]
pub type N_RST_SHDW_R = crate::BitReader;
#[doc = "Field `nRST_SHDW` writer - Reset generation in Shutdown mode"]
pub type N_RST_SHDW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1345_RST` reader - SRAM1, SRAM3, SRAM4 and SRAM5 erase upon system reset"]
pub type SRAM1345_RST_R = crate::BitReader;
#[doc = "Field `SRAM1345_RST` writer - SRAM1, SRAM3, SRAM4 and SRAM5 erase upon system reset"]
pub type SRAM1345_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_SW` reader - Independent watchdog selection"]
pub type IWDG_SW_R = crate::BitReader;
#[doc = "Field `IWDG_SW` writer - Independent watchdog selection"]
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode"]
pub type IWDG_STOP_R = crate::BitReader;
#[doc = "Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode"]
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode"]
pub type IWDG_STDBY_R = crate::BitReader;
#[doc = "Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode"]
pub type IWDG_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG_SW` reader - Window watchdog selection"]
pub type WWDG_SW_R = crate::BitReader;
#[doc = "Field `WWDG_SW` writer - Window watchdog selection"]
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP_BANK` reader - Swap banks"]
pub type SWAP_BANK_R = crate::BitReader;
#[doc = "Field `SWAP_BANK` writer - Swap banks"]
pub type SWAP_BANK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUALBANK` reader - Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices"]
pub type DUALBANK_R = crate::BitReader;
#[doc = "Field `DUALBANK` writer - Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices"]
pub type DUALBANK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPRAM_ECC` reader - Backup RAM ECC detection and correction enable"]
pub type BKPRAM_ECC_R = crate::BitReader;
#[doc = "Field `BKPRAM_ECC` writer - Backup RAM ECC detection and correction enable"]
pub type BKPRAM_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3_ECC` reader - SRAM3 ECC detection and correction enable"]
pub type SRAM3_ECC_R = crate::BitReader;
#[doc = "Field `SRAM3_ECC` writer - SRAM3 ECC detection and correction enable"]
pub type SRAM3_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2_ECC` reader - SRAM2 ECC detection and correction enable"]
pub type SRAM2_ECC_R = crate::BitReader;
#[doc = "Field `SRAM2_ECC` writer - SRAM2 ECC detection and correction enable"]
pub type SRAM2_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2_RST` reader - SRAM2 erase when system reset"]
pub type SRAM2_RST_R = crate::BitReader;
#[doc = "Field `SRAM2_RST` writer - SRAM2 erase when system reset"]
pub type SRAM2_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nSWBOOT0` reader - Software BOOT0"]
pub type N_SWBOOT0_R = crate::BitReader;
#[doc = "Field `nSWBOOT0` writer - Software BOOT0"]
pub type N_SWBOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT0` reader - nBOOT0 option bit"]
pub type N_BOOT0_R = crate::BitReader;
#[doc = "Field `nBOOT0` writer - nBOOT0 option bit"]
pub type N_BOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA15_PUPEN` reader - PA15 pull-up enable"]
pub type PA15_PUPEN_R = crate::BitReader;
#[doc = "Field `PA15_PUPEN` writer - PA15 pull-up enable"]
pub type PA15_PUPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VDD_HSLV` reader - High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V"]
pub type IO_VDD_HSLV_R = crate::BitReader;
#[doc = "Field `IO_VDD_HSLV` writer - High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V"]
pub type IO_VDD_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VDDIO2_HSLV` reader - High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V."]
pub type IO_VDDIO2_HSLV_R = crate::BitReader;
#[doc = "Field `IO_VDDIO2_HSLV` writer - High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V."]
pub type IO_VDDIO2_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEN` reader - Global TrustZone security enable"]
pub type TZEN_R = crate::BitReader;
#[doc = "Field `TZEN` writer - Global TrustZone security enable"]
pub type TZEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details."]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset."]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Reset generation in Stop mode"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset generation in Standby mode"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reset generation in Shutdown mode"]
    #[inline(always)]
    pub fn n_rst_shdw(&self) -> N_RST_SHDW_R {
        N_RST_SHDW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SRAM1, SRAM3, SRAM4 and SRAM5 erase upon system reset"]
    #[inline(always)]
    pub fn sram1345_rst(&self) -> SRAM1345_RST_R {
        SRAM1345_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Swap banks"]
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices"]
    #[inline(always)]
    pub fn dualbank(&self) -> DUALBANK_R {
        DUALBANK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Backup RAM ECC detection and correction enable"]
    #[inline(always)]
    pub fn bkpram_ecc(&self) -> BKPRAM_ECC_R {
        BKPRAM_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SRAM3 ECC detection and correction enable"]
    #[inline(always)]
    pub fn sram3_ecc(&self) -> SRAM3_ECC_R {
        SRAM3_ECC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SRAM2 ECC detection and correction enable"]
    #[inline(always)]
    pub fn sram2_ecc(&self) -> SRAM2_ECC_R {
        SRAM2_ECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SRAM2 erase when system reset"]
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Software BOOT0"]
    #[inline(always)]
    pub fn n_swboot0(&self) -> N_SWBOOT0_R {
        N_SWBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - nBOOT0 option bit"]
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PA15 pull-up enable"]
    #[inline(always)]
    pub fn pa15_pupen(&self) -> PA15_PUPEN_R {
        PA15_PUPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V"]
    #[inline(always)]
    pub fn io_vdd_hslv(&self) -> IO_VDD_HSLV_R {
        IO_VDD_HSLV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V."]
    #[inline(always)]
    pub fn io_vddio2_hslv(&self) -> IO_VDDIO2_HSLV_R {
        IO_VDDIO2_HSLV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Global TrustZone security enable"]
    #[inline(always)]
    pub fn tzen(&self) -> TZEN_R {
        TZEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details."]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<FLASH_OPTRrs> {
        RDP_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset."]
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<FLASH_OPTRrs> {
        BOR_LEV_W::new(self, 8)
    }
    #[doc = "Bit 12 - Reset generation in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<FLASH_OPTRrs> {
        N_RST_STOP_W::new(self, 12)
    }
    #[doc = "Bit 13 - Reset generation in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<FLASH_OPTRrs> {
        N_RST_STDBY_W::new(self, 13)
    }
    #[doc = "Bit 14 - Reset generation in Shutdown mode"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_shdw(&mut self) -> N_RST_SHDW_W<FLASH_OPTRrs> {
        N_RST_SHDW_W::new(self, 14)
    }
    #[doc = "Bit 15 - SRAM1, SRAM3, SRAM4 and SRAM5 erase upon system reset"]
    #[inline(always)]
    #[must_use]
    pub fn sram1345_rst(&mut self) -> SRAM1345_RST_W<FLASH_OPTRrs> {
        SRAM1345_RST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<FLASH_OPTRrs> {
        IWDG_SW_W::new(self, 16)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<FLASH_OPTRrs> {
        IWDG_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<FLASH_OPTRrs> {
        IWDG_STDBY_W::new(self, 18)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<FLASH_OPTRrs> {
        WWDG_SW_W::new(self, 19)
    }
    #[doc = "Bit 20 - Swap banks"]
    #[inline(always)]
    #[must_use]
    pub fn swap_bank(&mut self) -> SWAP_BANK_W<FLASH_OPTRrs> {
        SWAP_BANK_W::new(self, 20)
    }
    #[doc = "Bit 21 - Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices"]
    #[inline(always)]
    #[must_use]
    pub fn dualbank(&mut self) -> DUALBANK_W<FLASH_OPTRrs> {
        DUALBANK_W::new(self, 21)
    }
    #[doc = "Bit 22 - Backup RAM ECC detection and correction enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpram_ecc(&mut self) -> BKPRAM_ECC_W<FLASH_OPTRrs> {
        BKPRAM_ECC_W::new(self, 22)
    }
    #[doc = "Bit 23 - SRAM3 ECC detection and correction enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram3_ecc(&mut self) -> SRAM3_ECC_W<FLASH_OPTRrs> {
        SRAM3_ECC_W::new(self, 23)
    }
    #[doc = "Bit 24 - SRAM2 ECC detection and correction enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram2_ecc(&mut self) -> SRAM2_ECC_W<FLASH_OPTRrs> {
        SRAM2_ECC_W::new(self, 24)
    }
    #[doc = "Bit 25 - SRAM2 erase when system reset"]
    #[inline(always)]
    #[must_use]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<FLASH_OPTRrs> {
        SRAM2_RST_W::new(self, 25)
    }
    #[doc = "Bit 26 - Software BOOT0"]
    #[inline(always)]
    #[must_use]
    pub fn n_swboot0(&mut self) -> N_SWBOOT0_W<FLASH_OPTRrs> {
        N_SWBOOT0_W::new(self, 26)
    }
    #[doc = "Bit 27 - nBOOT0 option bit"]
    #[inline(always)]
    #[must_use]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<FLASH_OPTRrs> {
        N_BOOT0_W::new(self, 27)
    }
    #[doc = "Bit 28 - PA15 pull-up enable"]
    #[inline(always)]
    #[must_use]
    pub fn pa15_pupen(&mut self) -> PA15_PUPEN_W<FLASH_OPTRrs> {
        PA15_PUPEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V"]
    #[inline(always)]
    #[must_use]
    pub fn io_vdd_hslv(&mut self) -> IO_VDD_HSLV_W<FLASH_OPTRrs> {
        IO_VDD_HSLV_W::new(self, 29)
    }
    #[doc = "Bit 30 - High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V."]
    #[inline(always)]
    #[must_use]
    pub fn io_vddio2_hslv(&mut self) -> IO_VDDIO2_HSLV_W<FLASH_OPTRrs> {
        IO_VDDIO2_HSLV_W::new(self, 30)
    }
    #[doc = "Bit 31 - Global TrustZone security enable"]
    #[inline(always)]
    #[must_use]
    pub fn tzen(&mut self) -> TZEN_W<FLASH_OPTRrs> {
        TZEN_W::new(self, 31)
    }
}
#[doc = "FLASH option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_optr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_optr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_OPTRrs;
impl crate::RegisterSpec for FLASH_OPTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_optr::R`](R) reader structure"]
impl crate::Readable for FLASH_OPTRrs {}
#[doc = "`write(|w| ..)` method takes [`flash_optr::W`](W) writer structure"]
impl crate::Writable for FLASH_OPTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_OPTR to value 0"]
impl crate::Resettable for FLASH_OPTRrs {
    const RESET_VALUE: u32 = 0;
}
