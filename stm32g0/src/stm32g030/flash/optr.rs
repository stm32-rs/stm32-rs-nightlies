#[doc = "Register `OPTR` reader"]
pub type R = crate::R<OPTRrs>;
#[doc = "Register `OPTR` writer"]
pub type W = crate::W<OPTRrs>;
#[doc = "Field `RDP` reader - Read protection level"]
pub type RDP_R = crate::FieldReader;
#[doc = "Field `RDP` writer - Read protection level"]
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BOREN` reader - BOR reset Level"]
pub type BOREN_R = crate::BitReader;
#[doc = "Field `BOREN` writer - BOR reset Level"]
pub type BOREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BORF_LEV` reader - These bits contain the VDD supply level threshold that activates the reset"]
pub type BORF_LEV_R = crate::FieldReader;
#[doc = "Field `BORF_LEV` writer - These bits contain the VDD supply level threshold that activates the reset"]
pub type BORF_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BORR_LEV` reader - These bits contain the VDD supply level threshold that releases the reset."]
pub type BORR_LEV_R = crate::FieldReader;
#[doc = "Field `BORR_LEV` writer - These bits contain the VDD supply level threshold that releases the reset."]
pub type BORR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type N_RST_STOP_R = crate::BitReader;
#[doc = "Field `nRST_STOP` writer - nRST_STOP"]
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type N_RST_STDBY_R = crate::BitReader;
#[doc = "Field `nRST_STDBY` writer - nRST_STDBY"]
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRSTS_HDW` reader - nRSTS_HDW"]
pub type N_RSTS_HDW_R = crate::BitReader;
#[doc = "Field `nRSTS_HDW` writer - nRSTS_HDW"]
pub type N_RSTS_HDW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDWG_SW` reader - Independent watchdog selection"]
pub type IDWG_SW_R = crate::BitReader;
#[doc = "Field `IDWG_SW` writer - Independent watchdog selection"]
pub type IDWG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `RAM_PARITY_CHECK` reader - SRAM parity check control"]
pub type RAM_PARITY_CHECK_R = crate::BitReader;
#[doc = "Field `RAM_PARITY_CHECK` writer - SRAM parity check control"]
pub type RAM_PARITY_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT_SEL` reader - nBOOT_SEL"]
pub type N_BOOT_SEL_R = crate::BitReader;
#[doc = "Field `nBOOT_SEL` writer - nBOOT_SEL"]
pub type N_BOOT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT1` reader - Boot configuration"]
pub type N_BOOT1_R = crate::BitReader;
#[doc = "Field `nBOOT1` writer - Boot configuration"]
pub type N_BOOT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT0` reader - nBOOT0 option bit"]
pub type N_BOOT0_R = crate::BitReader;
#[doc = "Field `nBOOT0` writer - nBOOT0 option bit"]
pub type N_BOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRST_MODE` reader - NRST_MODE"]
pub type NRST_MODE_R = crate::FieldReader;
#[doc = "Field `NRST_MODE` writer - NRST_MODE"]
pub type NRST_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRHEN` reader - Internal reset holder enable bit"]
pub type IRHEN_R = crate::BitReader;
#[doc = "Field `IRHEN` writer - Internal reset holder enable bit"]
pub type IRHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - BOR reset Level"]
    #[inline(always)]
    pub fn boren(&self) -> BOREN_R {
        BOREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - These bits contain the VDD supply level threshold that activates the reset"]
    #[inline(always)]
    pub fn borf_lev(&self) -> BORF_LEV_R {
        BORF_LEV_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - These bits contain the VDD supply level threshold that releases the reset."]
    #[inline(always)]
    pub fn borr_lev(&self) -> BORR_LEV_R {
        BORR_LEV_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - nRSTS_HDW"]
    #[inline(always)]
    pub fn n_rsts_hdw(&self) -> N_RSTS_HDW_R {
        N_RSTS_HDW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn idwg_sw(&self) -> IDWG_SW_R {
        IDWG_SW_R::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 22 - SRAM parity check control"]
    #[inline(always)]
    pub fn ram_parity_check(&self) -> RAM_PARITY_CHECK_R {
        RAM_PARITY_CHECK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - nBOOT_SEL"]
    #[inline(always)]
    pub fn n_boot_sel(&self) -> N_BOOT_SEL_R {
        N_BOOT_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Boot configuration"]
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - nBOOT0 option bit"]
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - NRST_MODE"]
    #[inline(always)]
    pub fn nrst_mode(&self) -> NRST_MODE_R {
        NRST_MODE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Internal reset holder enable bit"]
    #[inline(always)]
    pub fn irhen(&self) -> IRHEN_R {
        IRHEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<OPTRrs> {
        RDP_W::new(self, 0)
    }
    #[doc = "Bit 8 - BOR reset Level"]
    #[inline(always)]
    #[must_use]
    pub fn boren(&mut self) -> BOREN_W<OPTRrs> {
        BOREN_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - These bits contain the VDD supply level threshold that activates the reset"]
    #[inline(always)]
    #[must_use]
    pub fn borf_lev(&mut self) -> BORF_LEV_W<OPTRrs> {
        BORF_LEV_W::new(self, 9)
    }
    #[doc = "Bits 11:12 - These bits contain the VDD supply level threshold that releases the reset."]
    #[inline(always)]
    #[must_use]
    pub fn borr_lev(&mut self) -> BORR_LEV_W<OPTRrs> {
        BORR_LEV_W::new(self, 11)
    }
    #[doc = "Bit 13 - nRST_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<OPTRrs> {
        N_RST_STOP_W::new(self, 13)
    }
    #[doc = "Bit 14 - nRST_STDBY"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<OPTRrs> {
        N_RST_STDBY_W::new(self, 14)
    }
    #[doc = "Bit 15 - nRSTS_HDW"]
    #[inline(always)]
    #[must_use]
    pub fn n_rsts_hdw(&mut self) -> N_RSTS_HDW_W<OPTRrs> {
        N_RSTS_HDW_W::new(self, 15)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn idwg_sw(&mut self) -> IDWG_SW_W<OPTRrs> {
        IDWG_SW_W::new(self, 16)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<OPTRrs> {
        IWDG_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<OPTRrs> {
        IWDG_STDBY_W::new(self, 18)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<OPTRrs> {
        WWDG_SW_W::new(self, 19)
    }
    #[doc = "Bit 22 - SRAM parity check control"]
    #[inline(always)]
    #[must_use]
    pub fn ram_parity_check(&mut self) -> RAM_PARITY_CHECK_W<OPTRrs> {
        RAM_PARITY_CHECK_W::new(self, 22)
    }
    #[doc = "Bit 24 - nBOOT_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn n_boot_sel(&mut self) -> N_BOOT_SEL_W<OPTRrs> {
        N_BOOT_SEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - Boot configuration"]
    #[inline(always)]
    #[must_use]
    pub fn n_boot1(&mut self) -> N_BOOT1_W<OPTRrs> {
        N_BOOT1_W::new(self, 25)
    }
    #[doc = "Bit 26 - nBOOT0 option bit"]
    #[inline(always)]
    #[must_use]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<OPTRrs> {
        N_BOOT0_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - NRST_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_mode(&mut self) -> NRST_MODE_W<OPTRrs> {
        NRST_MODE_W::new(self, 27)
    }
    #[doc = "Bit 29 - Internal reset holder enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn irhen(&mut self) -> IRHEN_W<OPTRrs> {
        IRHEN_W::new(self, 29)
    }
}
#[doc = "Flash option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optr::R`](R) reader structure"]
impl crate::Readable for OPTRrs {}
#[doc = "`write(|w| ..)` method takes [`optr::W`](W) writer structure"]
impl crate::Writable for OPTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTR to value 0xf000_0000"]
impl crate::Resettable for OPTRrs {
    const RESET_VALUE: u32 = 0xf000_0000;
}
