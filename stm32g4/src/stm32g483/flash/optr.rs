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
///Field `nRST_SHDW` reader - nRST_SHDW
pub type N_RST_SHDW_R = crate::BitReader;
///Field `nRST_SHDW` writer - nRST_SHDW
pub type N_RST_SHDW_W<'a, REG> = crate::BitWriter<'a, REG>;
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
/**Bank to boot from

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFB2 {
    ///0: Boot from memory bank 1
    Disabled = 0,
    ///1: Boot from memory bank 2
    Enabled = 1,
}
impl From<BFB2> for bool {
    #[inline(always)]
    fn from(variant: BFB2) -> Self {
        variant as u8 != 0
    }
}
///Field `BFB2` reader - Bank to boot from
pub type BFB2_R = crate::BitReader<BFB2>;
impl BFB2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BFB2 {
        match self.bits {
            false => BFB2::Disabled,
            true => BFB2::Enabled,
        }
    }
    ///Boot from memory bank 1
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BFB2::Disabled
    }
    ///Boot from memory bank 2
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BFB2::Enabled
    }
}
///Field `BFB2` writer - Bank to boot from
pub type BFB2_W<'a, REG> = crate::BitWriter<'a, REG, BFB2>;
impl<'a, REG> BFB2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Boot from memory bank 1
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BFB2::Disabled)
    }
    ///Boot from memory bank 2
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BFB2::Enabled)
    }
}
/**Single or dual bank mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBANK {
    ///0: Single-bank mode with 128 bits data read width
    SingleBankMode = 0,
    ///1: Dual-bank mode with 64 bits data
    DualBankMode = 1,
}
impl From<DBANK> for bool {
    #[inline(always)]
    fn from(variant: DBANK) -> Self {
        variant as u8 != 0
    }
}
///Field `DBANK` reader - Single or dual bank mode
pub type DBANK_R = crate::BitReader<DBANK>;
impl DBANK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBANK {
        match self.bits {
            false => DBANK::SingleBankMode,
            true => DBANK::DualBankMode,
        }
    }
    ///Single-bank mode with 128 bits data read width
    #[inline(always)]
    pub fn is_single_bank_mode(&self) -> bool {
        *self == DBANK::SingleBankMode
    }
    ///Dual-bank mode with 64 bits data
    #[inline(always)]
    pub fn is_dual_bank_mode(&self) -> bool {
        *self == DBANK::DualBankMode
    }
}
///Field `DBANK` writer - Single or dual bank mode
pub type DBANK_W<'a, REG> = crate::BitWriter<'a, REG, DBANK>;
impl<'a, REG> DBANK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single-bank mode with 128 bits data read width
    #[inline(always)]
    pub fn single_bank_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DBANK::SingleBankMode)
    }
    ///Dual-bank mode with 64 bits data
    #[inline(always)]
    pub fn dual_bank_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DBANK::DualBankMode)
    }
}
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
///Field `nSWBOOT0` reader - nSWBOOT0
pub type N_SWBOOT0_R = crate::BitReader;
///Field `nSWBOOT0` writer - nSWBOOT0
pub type N_SWBOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nBOOT0` reader - nBOOT0
pub type N_BOOT0_R = crate::BitReader;
///Field `nBOOT0` writer - nBOOT0
pub type N_BOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_MODE` reader - NRST_MODE
pub type NRST_MODE_R = crate::FieldReader;
///Field `NRST_MODE` writer - NRST_MODE
pub type NRST_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IRHEN` reader - IRHEN
pub type IRHEN_R = crate::BitReader;
///Field `IRHEN` writer - IRHEN
pub type IRHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 14 - nRST_SHDW
    #[inline(always)]
    pub fn n_rst_shdw(&self) -> N_RST_SHDW_R {
        N_RST_SHDW_R::new(((self.bits >> 14) & 1) != 0)
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
    ///Bit 20 - Bank to boot from
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Single or dual bank mode
    #[inline(always)]
    pub fn dbank(&self) -> DBANK_R {
        DBANK_R::new(((self.bits >> 22) & 1) != 0)
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
    ///Bit 26 - nSWBOOT0
    #[inline(always)]
    pub fn n_swboot0(&self) -> N_SWBOOT0_R {
        N_SWBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - nBOOT0
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:29 - NRST_MODE
    #[inline(always)]
    pub fn nrst_mode(&self) -> NRST_MODE_R {
        NRST_MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - IRHEN
    #[inline(always)]
    pub fn irhen(&self) -> IRHEN_R {
        IRHEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTR")
            .field("rdp", &self.rdp())
            .field("bor_lev", &self.bor_lev())
            .field("n_rst_stop", &self.n_rst_stop())
            .field("n_rst_stdby", &self.n_rst_stdby())
            .field("n_rst_shdw", &self.n_rst_shdw())
            .field("idwg_sw", &self.idwg_sw())
            .field("iwdg_stop", &self.iwdg_stop())
            .field("iwdg_stdby", &self.iwdg_stdby())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("n_boot1", &self.n_boot1())
            .field("sram2_pe", &self.sram2_pe())
            .field("sram2_rst", &self.sram2_rst())
            .field("n_swboot0", &self.n_swboot0())
            .field("n_boot0", &self.n_boot0())
            .field("nrst_mode", &self.nrst_mode())
            .field("irhen", &self.irhen())
            .field("dbank", &self.dbank())
            .field("bfb2", &self.bfb2())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<OPTRrs> {
        RDP_W::new(self, 0)
    }
    ///Bits 8:10 - BOR reset Level
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<OPTRrs> {
        BOR_LEV_W::new(self, 8)
    }
    ///Bit 12 - nRST_STOP
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<OPTRrs> {
        N_RST_STOP_W::new(self, 12)
    }
    ///Bit 13 - nRST_STDBY
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<OPTRrs> {
        N_RST_STDBY_W::new(self, 13)
    }
    ///Bit 14 - nRST_SHDW
    #[inline(always)]
    #[must_use]
    pub fn n_rst_shdw(&mut self) -> N_RST_SHDW_W<OPTRrs> {
        N_RST_SHDW_W::new(self, 14)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    #[must_use]
    pub fn idwg_sw(&mut self) -> IDWG_SW_W<OPTRrs> {
        IDWG_SW_W::new(self, 16)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<OPTRrs> {
        IWDG_STOP_W::new(self, 17)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<OPTRrs> {
        IWDG_STDBY_W::new(self, 18)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    #[must_use]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<OPTRrs> {
        WWDG_SW_W::new(self, 19)
    }
    ///Bit 20 - Bank to boot from
    #[inline(always)]
    #[must_use]
    pub fn bfb2(&mut self) -> BFB2_W<OPTRrs> {
        BFB2_W::new(self, 20)
    }
    ///Bit 22 - Single or dual bank mode
    #[inline(always)]
    #[must_use]
    pub fn dbank(&mut self) -> DBANK_W<OPTRrs> {
        DBANK_W::new(self, 22)
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    #[must_use]
    pub fn n_boot1(&mut self) -> N_BOOT1_W<OPTRrs> {
        N_BOOT1_W::new(self, 23)
    }
    ///Bit 24 - SRAM2 parity check enable
    #[inline(always)]
    #[must_use]
    pub fn sram2_pe(&mut self) -> SRAM2_PE_W<OPTRrs> {
        SRAM2_PE_W::new(self, 24)
    }
    ///Bit 25 - SRAM2 Erase when system reset
    #[inline(always)]
    #[must_use]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<OPTRrs> {
        SRAM2_RST_W::new(self, 25)
    }
    ///Bit 26 - nSWBOOT0
    #[inline(always)]
    #[must_use]
    pub fn n_swboot0(&mut self) -> N_SWBOOT0_W<OPTRrs> {
        N_SWBOOT0_W::new(self, 26)
    }
    ///Bit 27 - nBOOT0
    #[inline(always)]
    #[must_use]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<OPTRrs> {
        N_BOOT0_W::new(self, 27)
    }
    ///Bits 28:29 - NRST_MODE
    #[inline(always)]
    #[must_use]
    pub fn nrst_mode(&mut self) -> NRST_MODE_W<OPTRrs> {
        NRST_MODE_W::new(self, 28)
    }
    ///Bit 30 - IRHEN
    #[inline(always)]
    #[must_use]
    pub fn irhen(&mut self) -> IRHEN_W<OPTRrs> {
        IRHEN_W::new(self, 30)
    }
}
/**Flash option register

You can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483xx.html#FLASH:OPTR)*/
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
///`read()` method returns [`optr::R`](R) reader structure
impl crate::Readable for OPTRrs {}
///`write(|w| ..)` method takes [`optr::W`](W) writer structure
impl crate::Writable for OPTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OPTR to value 0xffef_f8aa
impl crate::Resettable for OPTRrs {
    const RESET_VALUE: u32 = 0xffef_f8aa;
}
