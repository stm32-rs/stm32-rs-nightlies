///Register `OPTR` reader
pub type R = crate::R<OPTRrs>;
///Register `OPTR` writer
pub type W = crate::W<OPTRrs>;
///Field `RDP` reader - Read protection level
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Read protection level
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BOR_LEV` reader - BOR reset Level
pub type BOR_LEV_R = crate::FieldReader;
///Field `BOR_LEV` writer - BOR reset Level
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `nRST_STOP` reader - nRST_STOP
pub type N_RST_STOP_R = crate::BitReader;
///Field `nRST_STOP` writer - nRST_STOP
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_STDBY` reader - nRST_STDBY
pub type N_RST_STDBY_R = crate::BitReader;
///Field `nRST_STDBY` writer - nRST_STDBY
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDWG_SW` reader - Independent watchdog selection
pub type IDWG_SW_R = crate::BitReader;
///Field `IDWG_SW` writer - Independent watchdog selection
pub type IDWG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader;
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_R = crate::BitReader;
///Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDG_SW` reader - Window watchdog selection
pub type WWDG_SW_R = crate::BitReader;
///Field `WWDG_SW` writer - Window watchdog selection
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BFB2` reader - Dual-bank boot
pub type BFB2_R = crate::BitReader;
///Field `BFB2` writer - Dual-bank boot
pub type BFB2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DUALBANK` reader - Dual-Bank on 512 KB or 256 KB Flash memory devices
pub type DUALBANK_R = crate::BitReader;
///Field `DUALBANK` writer - Dual-Bank on 512 KB or 256 KB Flash memory devices
pub type DUALBANK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nBOOT1` reader - Boot configuration
pub type N_BOOT1_R = crate::BitReader;
///Field `nBOOT1` writer - Boot configuration
pub type N_BOOT1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2_PE` reader - SRAM2 parity check enable
pub type SRAM2_PE_R = crate::BitReader;
///Field `SRAM2_PE` writer - SRAM2 parity check enable
pub type SRAM2_PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2_RST` reader - SRAM2 Erase when system reset
pub type SRAM2_RST_R = crate::BitReader;
///Field `SRAM2_RST` writer - SRAM2 Erase when system reset
pub type SRAM2_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nSWBOOT0` reader - Software BOOT0
pub type N_SWBOOT0_R = crate::BitReader;
///Field `nSWBOOT0` writer - Software BOOT0
pub type N_SWBOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nBOOT0` reader - nBOOT0 option bit
pub type N_BOOT0_R = crate::BitReader;
///Field `nBOOT0` writer - nBOOT0 option bit
pub type N_BOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 12 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn idwg_sw(&self) -> IDWG_SW_R {
        IDWG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Dual-bank boot
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Dual-Bank on 512 KB or 256 KB Flash memory devices
    #[inline(always)]
    pub fn dualbank(&self) -> DUALBANK_R {
        DUALBANK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SRAM2 parity check enable
    #[inline(always)]
    pub fn sram2_pe(&self) -> SRAM2_PE_R {
        SRAM2_PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SRAM2 Erase when system reset
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Software BOOT0
    #[inline(always)]
    pub fn n_swboot0(&self) -> N_SWBOOT0_R {
        N_SWBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTR")
            .field("rdp", &self.rdp())
            .field("bor_lev", &self.bor_lev())
            .field("n_rst_stop", &self.n_rst_stop())
            .field("n_rst_stdby", &self.n_rst_stdby())
            .field("idwg_sw", &self.idwg_sw())
            .field("iwdg_stop", &self.iwdg_stop())
            .field("iwdg_stdby", &self.iwdg_stdby())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("bfb2", &self.bfb2())
            .field("dualbank", &self.dualbank())
            .field("n_boot1", &self.n_boot1())
            .field("sram2_pe", &self.sram2_pe())
            .field("sram2_rst", &self.sram2_rst())
            .field("n_swboot0", &self.n_swboot0())
            .field("n_boot0", &self.n_boot0())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, OPTRrs> {
        RDP_W::new(self, 0)
    }
    ///Bits 8:10 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<'_, OPTRrs> {
        BOR_LEV_W::new(self, 8)
    }
    ///Bit 12 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<'_, OPTRrs> {
        N_RST_STOP_W::new(self, 12)
    }
    ///Bit 13 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<'_, OPTRrs> {
        N_RST_STDBY_W::new(self, 13)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn idwg_sw(&mut self) -> IDWG_SW_W<'_, OPTRrs> {
        IDWG_SW_W::new(self, 16)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<'_, OPTRrs> {
        IWDG_STOP_W::new(self, 17)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<'_, OPTRrs> {
        IWDG_STDBY_W::new(self, 18)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<'_, OPTRrs> {
        WWDG_SW_W::new(self, 19)
    }
    ///Bit 20 - Dual-bank boot
    #[inline(always)]
    pub fn bfb2(&mut self) -> BFB2_W<'_, OPTRrs> {
        BFB2_W::new(self, 20)
    }
    ///Bit 21 - Dual-Bank on 512 KB or 256 KB Flash memory devices
    #[inline(always)]
    pub fn dualbank(&mut self) -> DUALBANK_W<'_, OPTRrs> {
        DUALBANK_W::new(self, 21)
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&mut self) -> N_BOOT1_W<'_, OPTRrs> {
        N_BOOT1_W::new(self, 23)
    }
    ///Bit 24 - SRAM2 parity check enable
    #[inline(always)]
    pub fn sram2_pe(&mut self) -> SRAM2_PE_W<'_, OPTRrs> {
        SRAM2_PE_W::new(self, 24)
    }
    ///Bit 25 - SRAM2 Erase when system reset
    #[inline(always)]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<'_, OPTRrs> {
        SRAM2_RST_W::new(self, 25)
    }
    ///Bit 26 - Software BOOT0
    #[inline(always)]
    pub fn n_swboot0(&mut self) -> N_SWBOOT0_W<'_, OPTRrs> {
        N_SWBOOT0_W::new(self, 26)
    }
    ///Bit 27 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<'_, OPTRrs> {
        N_BOOT0_W::new(self, 27)
    }
}
/**Flash option register

You can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x1.html#FLASH:OPTR)*/
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
///`read()` method returns [`optr::R`](R) reader structure
impl crate::Readable for OPTRrs {}
///`write(|w| ..)` method takes [`optr::W`](W) writer structure
impl crate::Writable for OPTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTR to value 0xf000_0000
impl crate::Resettable for OPTRrs {
    const RESET_VALUE: u32 = 0xf000_0000;
}
