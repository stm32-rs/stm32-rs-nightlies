#[doc = "Register `PLL3CFGR` reader"]
pub type R = crate::R<PLL3CFGRrs>;
#[doc = "Register `PLL3CFGR` writer"]
pub type W = crate::W<PLL3CFGRrs>;
#[doc = "DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL3SRC must be set to '00'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL3SRC {
    #[doc = "0: No clock sent to DIVMx dividers and PLLs"]
    None = 0,
    #[doc = "1: HSI selected as PLL clock"]
    Hsi = 1,
    #[doc = "2: CSI selected as PLL clock"]
    Csi = 2,
    #[doc = "3: HSE selected as PLL clock"]
    Hse = 3,
}
impl From<PLL3SRC> for u8 {
    #[inline(always)]
    fn from(variant: PLL3SRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL3SRC {
    type Ux = u8;
}
#[doc = "Field `PLL3SRC` reader - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL3SRC must be set to '00'."]
pub type PLL3SRC_R = crate::FieldReader<PLL3SRC>;
impl PLL3SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL3SRC {
        match self.bits {
            0 => PLL3SRC::None,
            1 => PLL3SRC::Hsi,
            2 => PLL3SRC::Csi,
            3 => PLL3SRC::Hse,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock sent to DIVMx dividers and PLLs"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PLL3SRC::None
    }
    #[doc = "HSI selected as PLL clock"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == PLL3SRC::Hsi
    }
    #[doc = "CSI selected as PLL clock"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == PLL3SRC::Csi
    }
    #[doc = "HSE selected as PLL clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLL3SRC::Hse
    }
}
#[doc = "Field `PLL3SRC` writer - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL3SRC must be set to '00'."]
pub type PLL3SRC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PLL3SRC>;
impl<'a, REG> PLL3SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock sent to DIVMx dividers and PLLs"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3SRC::None)
    }
    #[doc = "HSI selected as PLL clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3SRC::Hsi)
    }
    #[doc = "CSI selected as PLL clock"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3SRC::Csi)
    }
    #[doc = "HSE selected as PLL clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3SRC::Hse)
    }
}
#[doc = "PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. This bit must be written before enabling the PLL3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL3RGE {
    #[doc = "0: Frequency is between 1 and 2 MHz"]
    Range1 = 0,
    #[doc = "1: Frequency is between 2 and 4 MHz"]
    Range2 = 1,
    #[doc = "2: Frequency is between 4 and 8 MHz"]
    Range4 = 2,
    #[doc = "3: Frequency is between 8 and 16 MHz"]
    Range8 = 3,
}
impl From<PLL3RGE> for u8 {
    #[inline(always)]
    fn from(variant: PLL3RGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL3RGE {
    type Ux = u8;
}
#[doc = "Field `PLL3RGE` reader - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. This bit must be written before enabling the PLL3."]
pub type PLL3RGE_R = crate::FieldReader<PLL3RGE>;
impl PLL3RGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL3RGE {
        match self.bits {
            0 => PLL3RGE::Range1,
            1 => PLL3RGE::Range2,
            2 => PLL3RGE::Range4,
            3 => PLL3RGE::Range8,
            _ => unreachable!(),
        }
    }
    #[doc = "Frequency is between 1 and 2 MHz"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == PLL3RGE::Range1
    }
    #[doc = "Frequency is between 2 and 4 MHz"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == PLL3RGE::Range2
    }
    #[doc = "Frequency is between 4 and 8 MHz"]
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == PLL3RGE::Range4
    }
    #[doc = "Frequency is between 8 and 16 MHz"]
    #[inline(always)]
    pub fn is_range8(&self) -> bool {
        *self == PLL3RGE::Range8
    }
}
#[doc = "Field `PLL3RGE` writer - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. This bit must be written before enabling the PLL3."]
pub type PLL3RGE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PLL3RGE>;
impl<'a, REG> PLL3RGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frequency is between 1 and 2 MHz"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3RGE::Range1)
    }
    #[doc = "Frequency is between 2 and 4 MHz"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3RGE::Range2)
    }
    #[doc = "Frequency is between 4 and 8 MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3RGE::Range4)
    }
    #[doc = "Frequency is between 8 and 16 MHz"]
    #[inline(always)]
    pub fn range8(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3RGE::Range8)
    }
}
#[doc = "PLL3 fractional latch enable Set and reset by software to latch the content of FRACN3 into the sigma-delta modulator. In order to latch the FRACN3 value into the sigma-delta modulator, PLL3FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN3 into the modulator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL3FRACEN {
    #[doc = "0: Reset latch to transfer FRACN to the Sigma-Delta modulator"]
    Reset = 0,
    #[doc = "1: Set latch to transfer FRACN to the Sigma-Delta modulator"]
    Set = 1,
}
impl From<PLL3FRACEN> for bool {
    #[inline(always)]
    fn from(variant: PLL3FRACEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL3FRACEN` reader - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN3 into the sigma-delta modulator. In order to latch the FRACN3 value into the sigma-delta modulator, PLL3FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN3 into the modulator."]
pub type PLL3FRACEN_R = crate::BitReader<PLL3FRACEN>;
impl PLL3FRACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL3FRACEN {
        match self.bits {
            false => PLL3FRACEN::Reset,
            true => PLL3FRACEN::Set,
        }
    }
    #[doc = "Reset latch to transfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PLL3FRACEN::Reset
    }
    #[doc = "Set latch to transfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == PLL3FRACEN::Set
    }
}
#[doc = "Field `PLL3FRACEN` writer - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN3 into the sigma-delta modulator. In order to latch the FRACN3 value into the sigma-delta modulator, PLL3FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN3 into the modulator."]
pub type PLL3FRACEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL3FRACEN>;
impl<'a, REG> PLL3FRACEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset latch to transfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3FRACEN::Reset)
    }
    #[doc = "Set latch to transfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3FRACEN::Set)
    }
}
#[doc = "PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL3VCOSEL {
    #[doc = "0: VCO frequency range 192 to 836 MHz"]
    WideVco = 0,
    #[doc = "1: VCO frequency range 150 to 420 MHz"]
    MediumVco = 1,
}
impl From<PLL3VCOSEL> for bool {
    #[inline(always)]
    fn from(variant: PLL3VCOSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL3VCOSEL` reader - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3."]
pub type PLL3VCOSEL_R = crate::BitReader<PLL3VCOSEL>;
impl PLL3VCOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL3VCOSEL {
        match self.bits {
            false => PLL3VCOSEL::WideVco,
            true => PLL3VCOSEL::MediumVco,
        }
    }
    #[doc = "VCO frequency range 192 to 836 MHz"]
    #[inline(always)]
    pub fn is_wide_vco(&self) -> bool {
        *self == PLL3VCOSEL::WideVco
    }
    #[doc = "VCO frequency range 150 to 420 MHz"]
    #[inline(always)]
    pub fn is_medium_vco(&self) -> bool {
        *self == PLL3VCOSEL::MediumVco
    }
}
#[doc = "Field `PLL3VCOSEL` writer - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3."]
pub type PLL3VCOSEL_W<'a, REG> = crate::BitWriter<'a, REG, PLL3VCOSEL>;
impl<'a, REG> PLL3VCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VCO frequency range 192 to 836 MHz"]
    #[inline(always)]
    pub fn wide_vco(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3VCOSEL::WideVco)
    }
    #[doc = "VCO frequency range 150 to 420 MHz"]
    #[inline(always)]
    pub fn medium_vco(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3VCOSEL::MediumVco)
    }
}
#[doc = "Field `DIVM3` reader - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ON = 1 or PLL3RDY = 1). In order to save power when PLL3 is not used, the value of DIVM2 must be set to 0. ... ..."]
pub type DIVM3_R = crate::FieldReader;
#[doc = "Field `DIVM3` writer - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ON = 1 or PLL3RDY = 1). In order to save power when PLL3 is not used, the value of DIVM2 must be set to 0. ... ..."]
pub type DIVM3_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, when the pll3_p_ck output of the PLL3 is not used, the pll3_p_ck must be disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL3PEN {
    #[doc = "0: Clock output is disabled"]
    Disabled = 0,
    #[doc = "1: Clock output is enabled"]
    Enabled = 1,
}
impl From<PLL3PEN> for bool {
    #[inline(always)]
    fn from(variant: PLL3PEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL3PEN` reader - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, when the pll3_p_ck output of the PLL3 is not used, the pll3_p_ck must be disabled."]
pub type PLL3PEN_R = crate::BitReader<PLL3PEN>;
impl PLL3PEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL3PEN {
        match self.bits {
            false => PLL3PEN::Disabled,
            true => PLL3PEN::Enabled,
        }
    }
    #[doc = "Clock output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL3PEN::Disabled
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL3PEN::Enabled
    }
}
#[doc = "Field `PLL3PEN` writer - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, when the pll3_p_ck output of the PLL3 is not used, the pll3_p_ck must be disabled."]
pub type PLL3PEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL3PEN>;
impl<'a, REG> PLL3PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3PEN::Disabled)
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3PEN::Enabled)
    }
}
#[doc = "Field `PLL3QEN` reader - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, when the pll3_q_ck output of the PLL3 is not used, the pll3_q_ck must be disabled."]
pub use PLL3PEN_R as PLL3QEN_R;
#[doc = "Field `PLL3REN` reader - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll3_r_ck is not used."]
pub use PLL3PEN_R as PLL3REN_R;
#[doc = "Field `PLL3QEN` writer - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, when the pll3_q_ck output of the PLL3 is not used, the pll3_q_ck must be disabled."]
pub use PLL3PEN_W as PLL3QEN_W;
#[doc = "Field `PLL3REN` writer - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll3_r_ck is not used."]
pub use PLL3PEN_W as PLL3REN_W;
impl R {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL3SRC must be set to '00'."]
    #[inline(always)]
    pub fn pll3src(&self) -> PLL3SRC_R {
        PLL3SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. This bit must be written before enabling the PLL3."]
    #[inline(always)]
    pub fn pll3rge(&self) -> PLL3RGE_R {
        PLL3RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN3 into the sigma-delta modulator. In order to latch the FRACN3 value into the sigma-delta modulator, PLL3FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN3 into the modulator."]
    #[inline(always)]
    pub fn pll3fracen(&self) -> PLL3FRACEN_R {
        PLL3FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3."]
    #[inline(always)]
    pub fn pll3vcosel(&self) -> PLL3VCOSEL_R {
        PLL3VCOSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ON = 1 or PLL3RDY = 1). In order to save power when PLL3 is not used, the value of DIVM2 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn divm3(&self) -> DIVM3_R {
        DIVM3_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, when the pll3_p_ck output of the PLL3 is not used, the pll3_p_ck must be disabled."]
    #[inline(always)]
    pub fn pll3pen(&self) -> PLL3PEN_R {
        PLL3PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, when the pll3_q_ck output of the PLL3 is not used, the pll3_q_ck must be disabled."]
    #[inline(always)]
    pub fn pll3qen(&self) -> PLL3QEN_R {
        PLL3QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll3_r_ck is not used."]
    #[inline(always)]
    pub fn pll3ren(&self) -> PLL3REN_R {
        PLL3REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL3SRC must be set to '00'."]
    #[inline(always)]
    #[must_use]
    pub fn pll3src(&mut self) -> PLL3SRC_W<PLL3CFGRrs> {
        PLL3SRC_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. This bit must be written before enabling the PLL3."]
    #[inline(always)]
    #[must_use]
    pub fn pll3rge(&mut self) -> PLL3RGE_W<PLL3CFGRrs> {
        PLL3RGE_W::new(self, 2)
    }
    #[doc = "Bit 4 - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN3 into the sigma-delta modulator. In order to latch the FRACN3 value into the sigma-delta modulator, PLL3FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN3 into the modulator."]
    #[inline(always)]
    #[must_use]
    pub fn pll3fracen(&mut self) -> PLL3FRACEN_W<PLL3CFGRrs> {
        PLL3FRACEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3."]
    #[inline(always)]
    #[must_use]
    pub fn pll3vcosel(&mut self) -> PLL3VCOSEL_W<PLL3CFGRrs> {
        PLL3VCOSEL_W::new(self, 5)
    }
    #[doc = "Bits 8:13 - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ON = 1 or PLL3RDY = 1). In order to save power when PLL3 is not used, the value of DIVM2 must be set to 0. ... ..."]
    #[inline(always)]
    #[must_use]
    pub fn divm3(&mut self) -> DIVM3_W<PLL3CFGRrs> {
        DIVM3_W::new(self, 8)
    }
    #[doc = "Bit 16 - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, when the pll3_p_ck output of the PLL3 is not used, the pll3_p_ck must be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn pll3pen(&mut self) -> PLL3PEN_W<PLL3CFGRrs> {
        PLL3PEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, when the pll3_q_ck output of the PLL3 is not used, the pll3_q_ck must be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn pll3qen(&mut self) -> PLL3QEN_W<PLL3CFGRrs> {
        PLL3QEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll3_r_ck is not used."]
    #[inline(always)]
    #[must_use]
    pub fn pll3ren(&mut self) -> PLL3REN_W<PLL3CFGRrs> {
        PLL3REN_W::new(self, 18)
    }
}
#[doc = "RCC PLL clock source selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll3cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll3cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL3CFGRrs;
impl crate::RegisterSpec for PLL3CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll3cfgr::R`](R) reader structure"]
impl crate::Readable for PLL3CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`pll3cfgr::W`](W) writer structure"]
impl crate::Writable for PLL3CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL3CFGR to value 0"]
impl crate::Resettable for PLL3CFGRrs {
    const RESET_VALUE: u32 = 0;
}
