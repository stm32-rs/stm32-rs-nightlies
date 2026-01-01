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
///Field `nRST_STOP` reader - None
pub type N_RST_STOP_R = crate::BitReader;
///Field `nRST_STOP` writer - None
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_STDBY` reader - None
pub type N_RST_STDBY_R = crate::BitReader;
///Field `nRST_STDBY` writer - None
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_SHDW` reader - None
pub type N_RST_SHDW_R = crate::BitReader;
///Field `nRST_SHDW` writer - None
pub type N_RST_SHDW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_SW` reader - Independent watchdog selection
pub type IWDG_SW_R = crate::BitReader;
///Field `IWDG_SW` writer - Independent watchdog selection
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STOP` reader - None
pub type IWDG_STOP_R = crate::BitReader;
///Field `IWDG_STOP` writer - None
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWGD_STDBY` reader - None
pub type IWGD_STDBY_R = crate::BitReader;
///Field `IWGD_STDBY` writer - None
pub type IWGD_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDG_SW` reader - Window watchdog selection
pub type WWDG_SW_R = crate::BitReader;
///Field `WWDG_SW` writer - Window watchdog selection
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Dual-bank boot

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
///Field `BFB2` reader - Dual-bank boot
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
///Field `BFB2` writer - Dual-bank boot
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
/**This bit can only be written when PCROPA/B is disabled.

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
///Field `DBANK` reader - This bit can only be written when PCROPA/B is disabled.
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
///Field `DBANK` writer - This bit can only be written when PCROPA/B is disabled.
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
///Field `SRAM_PE` reader - SRAM1 and CCM SRAM parity check enable
pub type SRAM_PE_R = crate::BitReader;
///Field `SRAM_PE` writer - SRAM1 and CCM SRAM parity check enable
pub type SRAM_PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCMSRAM_RST` reader - None
pub type CCMSRAM_RST_R = crate::BitReader;
///Field `CCMSRAM_RST` writer - None
pub type CCMSRAM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nSWBOOT0` reader - Software BOOT0
pub type N_SWBOOT0_R = crate::BitReader;
///Field `nSWBOOT0` writer - Software BOOT0
pub type N_SWBOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nBOOT0` reader - nBOOT0 option bit
pub type N_BOOT0_R = crate::BitReader;
///Field `nBOOT0` writer - nBOOT0 option bit
pub type N_BOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_MODE` reader - None
pub type NRST_MODE_R = crate::FieldReader;
///Field `NRST_MODE` writer - None
pub type NRST_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IRHEN` reader - Internal reset holder enable bit
pub type IRHEN_R = crate::BitReader;
///Field `IRHEN` writer - Internal reset holder enable bit
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
    ///Bit 12 - None
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - None
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - None
    #[inline(always)]
    pub fn n_rst_shdw(&self) -> N_RST_SHDW_R {
        N_RST_SHDW_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - None
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - None
    #[inline(always)]
    pub fn iwgd_stdby(&self) -> IWGD_STDBY_R {
        IWGD_STDBY_R::new(((self.bits >> 18) & 1) != 0)
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
    ///Bit 22 - This bit can only be written when PCROPA/B is disabled.
    #[inline(always)]
    pub fn dbank(&self) -> DBANK_R {
        DBANK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SRAM1 and CCM SRAM parity check enable
    #[inline(always)]
    pub fn sram_pe(&self) -> SRAM_PE_R {
        SRAM_PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - None
    #[inline(always)]
    pub fn ccmsram_rst(&self) -> CCMSRAM_RST_R {
        CCMSRAM_RST_R::new(((self.bits >> 25) & 1) != 0)
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
    ///Bits 28:29 - None
    #[inline(always)]
    pub fn nrst_mode(&self) -> NRST_MODE_R {
        NRST_MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - Internal reset holder enable bit
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
            .field("iwdg_sw", &self.iwdg_sw())
            .field("iwdg_stop", &self.iwdg_stop())
            .field("iwgd_stdby", &self.iwgd_stdby())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("bfb2", &self.bfb2())
            .field("dbank", &self.dbank())
            .field("n_boot1", &self.n_boot1())
            .field("sram_pe", &self.sram_pe())
            .field("ccmsram_rst", &self.ccmsram_rst())
            .field("n_swboot0", &self.n_swboot0())
            .field("n_boot0", &self.n_boot0())
            .field("nrst_mode", &self.nrst_mode())
            .field("irhen", &self.irhen())
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
    ///Bit 12 - None
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<'_, OPTRrs> {
        N_RST_STOP_W::new(self, 12)
    }
    ///Bit 13 - None
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<'_, OPTRrs> {
        N_RST_STDBY_W::new(self, 13)
    }
    ///Bit 14 - None
    #[inline(always)]
    pub fn n_rst_shdw(&mut self) -> N_RST_SHDW_W<'_, OPTRrs> {
        N_RST_SHDW_W::new(self, 14)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<'_, OPTRrs> {
        IWDG_SW_W::new(self, 16)
    }
    ///Bit 17 - None
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<'_, OPTRrs> {
        IWDG_STOP_W::new(self, 17)
    }
    ///Bit 18 - None
    #[inline(always)]
    pub fn iwgd_stdby(&mut self) -> IWGD_STDBY_W<'_, OPTRrs> {
        IWGD_STDBY_W::new(self, 18)
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
    ///Bit 22 - This bit can only be written when PCROPA/B is disabled.
    #[inline(always)]
    pub fn dbank(&mut self) -> DBANK_W<'_, OPTRrs> {
        DBANK_W::new(self, 22)
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&mut self) -> N_BOOT1_W<'_, OPTRrs> {
        N_BOOT1_W::new(self, 23)
    }
    ///Bit 24 - SRAM1 and CCM SRAM parity check enable
    #[inline(always)]
    pub fn sram_pe(&mut self) -> SRAM_PE_W<'_, OPTRrs> {
        SRAM_PE_W::new(self, 24)
    }
    ///Bit 25 - None
    #[inline(always)]
    pub fn ccmsram_rst(&mut self) -> CCMSRAM_RST_W<'_, OPTRrs> {
        CCMSRAM_RST_W::new(self, 25)
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
    ///Bits 28:29 - None
    #[inline(always)]
    pub fn nrst_mode(&mut self) -> NRST_MODE_W<'_, OPTRrs> {
        NRST_MODE_W::new(self, 28)
    }
    ///Bit 30 - Internal reset holder enable bit
    #[inline(always)]
    pub fn irhen(&mut self) -> IRHEN_W<'_, OPTRrs> {
        IRHEN_W::new(self, 30)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#FLASH:OPTR)*/
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
///`reset()` method sets OPTR to value 0xffef_f8aa
impl crate::Resettable for OPTRrs {
    const RESET_VALUE: u32 = 0xffef_f8aa;
}
