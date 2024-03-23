#[doc = "Register `PLL1CFGR` reader"]
pub type R = crate::R<PLL1CFGRrs>;
#[doc = "Register `PLL1CFGR` writer"]
pub type W = crate::W<PLL1CFGRrs>;
#[doc = "DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1SRC {
    #[doc = "0: No clock sent to DIVMx dividers and PLLs"]
    None = 0,
    #[doc = "1: HSI selected as PLL clock"]
    Hsi = 1,
    #[doc = "2: CSI selected as PLL clock"]
    Csi = 2,
    #[doc = "3: HSE selected as PLL clock"]
    Hse = 3,
}
impl From<PLL1SRC> for u8 {
    #[inline(always)]
    fn from(variant: PLL1SRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL1SRC {
    type Ux = u8;
}
#[doc = "Field `PLL1SRC` reader - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
pub type PLL1SRC_R = crate::FieldReader<PLL1SRC>;
impl PLL1SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1SRC {
        match self.bits {
            0 => PLL1SRC::None,
            1 => PLL1SRC::Hsi,
            2 => PLL1SRC::Csi,
            3 => PLL1SRC::Hse,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock sent to DIVMx dividers and PLLs"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PLL1SRC::None
    }
    #[doc = "HSI selected as PLL clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == PLL1SRC::Hsi
    }
    #[doc = "CSI selected as PLL clock"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == PLL1SRC::Csi
    }
    #[doc = "HSE selected as PLL clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLL1SRC::Hse
    }
}
#[doc = "Field `PLL1SRC` writer - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
pub type PLL1SRC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PLL1SRC>;
impl<'a, REG> PLL1SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock sent to DIVMx dividers and PLLs"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC::None)
    }
    #[doc = "HSI selected as PLL clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC::Hsi)
    }
    #[doc = "CSI selected as PLL clock"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC::Csi)
    }
    #[doc = "HSE selected as PLL clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC::Hse)
    }
}
#[doc = "PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1RGE {
    #[doc = "0: Frequency is between 1 and 2 MHz"]
    Range1 = 0,
    #[doc = "1: Frequency is between 2 and 4 MHz"]
    Range2 = 1,
    #[doc = "2: Frequency is between 4 and 8 MHz"]
    Range4 = 2,
    #[doc = "3: Frequency is between 8 and 16 MHz"]
    Range8 = 3,
}
impl From<PLL1RGE> for u8 {
    #[inline(always)]
    fn from(variant: PLL1RGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL1RGE {
    type Ux = u8;
}
#[doc = "Field `PLL1RGE` reader - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1RGE_R = crate::FieldReader<PLL1RGE>;
impl PLL1RGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1RGE {
        match self.bits {
            0 => PLL1RGE::Range1,
            1 => PLL1RGE::Range2,
            2 => PLL1RGE::Range4,
            3 => PLL1RGE::Range8,
            _ => unreachable!(),
        }
    }
    #[doc = "Frequency is between 1 and 2 MHz"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == PLL1RGE::Range1
    }
    #[doc = "Frequency is between 2 and 4 MHz"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == PLL1RGE::Range2
    }
    #[doc = "Frequency is between 4 and 8 MHz"]
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == PLL1RGE::Range4
    }
    #[doc = "Frequency is between 8 and 16 MHz"]
    #[inline(always)]
    pub fn is_range8(&self) -> bool {
        *self == PLL1RGE::Range8
    }
}
#[doc = "Field `PLL1RGE` writer - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1RGE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PLL1RGE>;
impl<'a, REG> PLL1RGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frequency is between 1 and 2 MHz"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE::Range1)
    }
    #[doc = "Frequency is between 2 and 4 MHz"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE::Range2)
    }
    #[doc = "Frequency is between 4 and 8 MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE::Range4)
    }
    #[doc = "Frequency is between 8 and 16 MHz"]
    #[inline(always)]
    pub fn range8(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE::Range8)
    }
}
#[doc = "PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1FRACEN {
    #[doc = "0: Reset latch to transfer FRACN to the Sigma-Delta modulator"]
    Reset = 0,
    #[doc = "1: Set latch to transfer FRACN to the Sigma-Delta modulator"]
    Set = 1,
}
impl From<PLL1FRACEN> for bool {
    #[inline(always)]
    fn from(variant: PLL1FRACEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1FRACEN` reader - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
pub type PLL1FRACEN_R = crate::BitReader<PLL1FRACEN>;
impl PLL1FRACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1FRACEN {
        match self.bits {
            false => PLL1FRACEN::Reset,
            true => PLL1FRACEN::Set,
        }
    }
    #[doc = "Reset latch to transfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PLL1FRACEN::Reset
    }
    #[doc = "Set latch to transfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == PLL1FRACEN::Set
    }
}
#[doc = "Field `PLL1FRACEN` writer - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
pub type PLL1FRACEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL1FRACEN>;
impl<'a, REG> PLL1FRACEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset latch to transfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1FRACEN::Reset)
    }
    #[doc = "Set latch to transfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1FRACEN::Set)
    }
}
#[doc = "PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1VCOSEL {
    #[doc = "0: VCO frequency range 192 to 836 MHz"]
    WideVco = 0,
    #[doc = "1: VCO frequency range 150 to 420 MHz"]
    MediumVco = 1,
}
impl From<PLL1VCOSEL> for bool {
    #[inline(always)]
    fn from(variant: PLL1VCOSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1VCOSEL` reader - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1VCOSEL_R = crate::BitReader<PLL1VCOSEL>;
impl PLL1VCOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1VCOSEL {
        match self.bits {
            false => PLL1VCOSEL::WideVco,
            true => PLL1VCOSEL::MediumVco,
        }
    }
    #[doc = "VCO frequency range 192 to 836 MHz"]
    #[inline(always)]
    pub fn is_wide_vco(&self) -> bool {
        *self == PLL1VCOSEL::WideVco
    }
    #[doc = "VCO frequency range 150 to 420 MHz"]
    #[inline(always)]
    pub fn is_medium_vco(&self) -> bool {
        *self == PLL1VCOSEL::MediumVco
    }
}
#[doc = "Field `PLL1VCOSEL` writer - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1VCOSEL_W<'a, REG> = crate::BitWriter<'a, REG, PLL1VCOSEL>;
impl<'a, REG> PLL1VCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VCO frequency range 192 to 836 MHz"]
    #[inline(always)]
    pub fn wide_vco(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1VCOSEL::WideVco)
    }
    #[doc = "VCO frequency range 150 to 420 MHz"]
    #[inline(always)]
    pub fn medium_vco(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1VCOSEL::MediumVco)
    }
}
#[doc = "Field `PLL1M` reader - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
pub type PLL1M_R = crate::FieldReader;
#[doc = "Field `PLL1M` writer - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
pub type PLL1M_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1PEN {
    #[doc = "0: Clock output is disabled"]
    Disabled = 0,
    #[doc = "1: Clock output is enabled"]
    Enabled = 1,
}
impl From<PLL1PEN> for bool {
    #[inline(always)]
    fn from(variant: PLL1PEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1PEN` reader - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
pub type PLL1PEN_R = crate::BitReader<PLL1PEN>;
impl PLL1PEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1PEN {
        match self.bits {
            false => PLL1PEN::Disabled,
            true => PLL1PEN::Enabled,
        }
    }
    #[doc = "Clock output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL1PEN::Disabled
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL1PEN::Enabled
    }
}
#[doc = "Field `PLL1PEN` writer - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
pub type PLL1PEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL1PEN>;
impl<'a, REG> PLL1PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1PEN::Disabled)
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1PEN::Enabled)
    }
}
#[doc = "Field `PLL1QEN` reader - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub use PLL1PEN_R as PLL1QEN_R;
#[doc = "Field `PLL1REN` reader - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub use PLL1PEN_R as PLL1REN_R;
#[doc = "Field `PLL1QEN` writer - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub use PLL1PEN_W as PLL1QEN_W;
#[doc = "Field `PLL1REN` writer - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub use PLL1PEN_W as PLL1REN_W;
impl R {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
    #[inline(always)]
    pub fn pll1src(&self) -> PLL1SRC_R {
        PLL1SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
    #[inline(always)]
    pub fn pll1fracen(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    pub fn pll1vcosel(&self) -> PLL1VCOSEL_R {
        PLL1VCOSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn pll1m(&self) -> PLL1M_R {
        PLL1M_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
    #[inline(always)]
    pub fn pll1pen(&self) -> PLL1PEN_R {
        PLL1PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    pub fn pll1qen(&self) -> PLL1QEN_R {
        PLL1QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    pub fn pll1ren(&self) -> PLL1REN_R {
        PLL1REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
    #[inline(always)]
    #[must_use]
    pub fn pll1src(&mut self) -> PLL1SRC_W<PLL1CFGRrs> {
        PLL1SRC_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    #[must_use]
    pub fn pll1rge(&mut self) -> PLL1RGE_W<PLL1CFGRrs> {
        PLL1RGE_W::new(self, 2)
    }
    #[doc = "Bit 4 - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
    #[inline(always)]
    #[must_use]
    pub fn pll1fracen(&mut self) -> PLL1FRACEN_W<PLL1CFGRrs> {
        PLL1FRACEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    #[must_use]
    pub fn pll1vcosel(&mut self) -> PLL1VCOSEL_W<PLL1CFGRrs> {
        PLL1VCOSEL_W::new(self, 5)
    }
    #[doc = "Bits 8:13 - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll1m(&mut self) -> PLL1M_W<PLL1CFGRrs> {
        PLL1M_W::new(self, 8)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn pll1pen(&mut self) -> PLL1PEN_W<PLL1CFGRrs> {
        PLL1PEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn pll1qen(&mut self) -> PLL1QEN_W<PLL1CFGRrs> {
        PLL1QEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn pll1ren(&mut self) -> PLL1REN_W<PLL1CFGRrs> {
        PLL1REN_W::new(self, 18)
    }
}
#[doc = "RCC PLL clock source selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL1CFGRrs;
impl crate::RegisterSpec for PLL1CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1cfgr::R`](R) reader structure"]
impl crate::Readable for PLL1CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`pll1cfgr::W`](W) writer structure"]
impl crate::Writable for PLL1CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL1CFGR to value 0"]
impl crate::Resettable for PLL1CFGRrs {
    const RESET_VALUE: u32 = 0;
}
