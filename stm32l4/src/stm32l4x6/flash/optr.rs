#[doc = "Register `OPTR` reader"]
pub type R = crate::R<OPTRrs>;
#[doc = "Register `OPTR` writer"]
pub type W = crate::W<OPTRrs>;
#[doc = "Field `RDP` reader - Read protection level"]
pub type RDP_R = crate::FieldReader;
#[doc = "Field `RDP` writer - Read protection level"]
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BOR_LEV` reader - BOR reset Level"]
pub type BOR_LEV_R = crate::FieldReader;
#[doc = "Field `BOR_LEV` writer - BOR reset Level"]
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type N_RST_STOP_R = crate::BitReader;
#[doc = "Field `nRST_STOP` writer - nRST_STOP"]
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type N_RST_STDBY_R = crate::BitReader;
#[doc = "Field `nRST_STDBY` writer - nRST_STDBY"]
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `BFB2` reader - Dual-bank boot"]
pub type BFB2_R = crate::BitReader;
#[doc = "Field `BFB2` writer - Dual-bank boot"]
pub type BFB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUALBANK` reader - Dual-Bank on 512 KB or 256 KB Flash memory devices"]
pub type DUALBANK_R = crate::BitReader;
#[doc = "Field `DUALBANK` writer - Dual-Bank on 512 KB or 256 KB Flash memory devices"]
pub type DUALBANK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT1` reader - Boot configuration"]
pub type N_BOOT1_R = crate::BitReader;
#[doc = "Field `nBOOT1` writer - Boot configuration"]
pub type N_BOOT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2_PE` reader - SRAM2 parity check enable"]
pub type SRAM2_PE_R = crate::BitReader;
#[doc = "Field `SRAM2_PE` writer - SRAM2 parity check enable"]
pub type SRAM2_PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2_RST` reader - SRAM2 Erase when system reset"]
pub type SRAM2_RST_R = crate::BitReader;
#[doc = "Field `SRAM2_RST` writer - SRAM2 Erase when system reset"]
pub type SRAM2_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nSWBOOT0` reader - Software BOOT0"]
pub type N_SWBOOT0_R = crate::BitReader;
#[doc = "Field `nSWBOOT0` writer - Software BOOT0"]
pub type N_SWBOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT0` reader - nBOOT0 option bit"]
pub type N_BOOT0_R = crate::BitReader;
#[doc = "Field `nBOOT0` writer - nBOOT0 option bit"]
pub type N_BOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - BOR reset Level"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 13) & 1) != 0)
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
    #[doc = "Bit 20 - Dual-bank boot"]
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Dual-Bank on 512 KB or 256 KB Flash memory devices"]
    #[inline(always)]
    pub fn dualbank(&self) -> DUALBANK_R {
        DUALBANK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Boot configuration"]
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SRAM2 parity check enable"]
    #[inline(always)]
    pub fn sram2_pe(&self) -> SRAM2_PE_R {
        SRAM2_PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SRAM2 Erase when system reset"]
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
}
impl W {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<OPTRrs> {
        RDP_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - BOR reset Level"]
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<OPTRrs> {
        BOR_LEV_W::new(self, 8)
    }
    #[doc = "Bit 12 - nRST_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<OPTRrs> {
        N_RST_STOP_W::new(self, 12)
    }
    #[doc = "Bit 13 - nRST_STDBY"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<OPTRrs> {
        N_RST_STDBY_W::new(self, 13)
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
    #[doc = "Bit 20 - Dual-bank boot"]
    #[inline(always)]
    #[must_use]
    pub fn bfb2(&mut self) -> BFB2_W<OPTRrs> {
        BFB2_W::new(self, 20)
    }
    #[doc = "Bit 21 - Dual-Bank on 512 KB or 256 KB Flash memory devices"]
    #[inline(always)]
    #[must_use]
    pub fn dualbank(&mut self) -> DUALBANK_W<OPTRrs> {
        DUALBANK_W::new(self, 21)
    }
    #[doc = "Bit 23 - Boot configuration"]
    #[inline(always)]
    #[must_use]
    pub fn n_boot1(&mut self) -> N_BOOT1_W<OPTRrs> {
        N_BOOT1_W::new(self, 23)
    }
    #[doc = "Bit 24 - SRAM2 parity check enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram2_pe(&mut self) -> SRAM2_PE_W<OPTRrs> {
        SRAM2_PE_W::new(self, 24)
    }
    #[doc = "Bit 25 - SRAM2 Erase when system reset"]
    #[inline(always)]
    #[must_use]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<OPTRrs> {
        SRAM2_RST_W::new(self, 25)
    }
    #[doc = "Bit 26 - Software BOOT0"]
    #[inline(always)]
    #[must_use]
    pub fn n_swboot0(&mut self) -> N_SWBOOT0_W<OPTRrs> {
        N_SWBOOT0_W::new(self, 26)
    }
    #[doc = "Bit 27 - nBOOT0 option bit"]
    #[inline(always)]
    #[must_use]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<OPTRrs> {
        N_BOOT0_W::new(self, 27)
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
