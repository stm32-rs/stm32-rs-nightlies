///Register `PLLCFGR` reader
pub type R = crate::R<PLLCFGRrs>;
///Register `PLLCFGR` writer
pub type W = crate::W<PLLCFGRrs>;
/**PLL1 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL1FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1FRACEN {
    ///0: Reset latch to tranfer FRACN to the Sigma-Delta modulator
    Reset = 0,
    ///1: Set latch to tranfer FRACN to the Sigma-Delta modulator
    Set = 1,
}
impl From<PLL1FRACEN> for bool {
    #[inline(always)]
    fn from(variant: PLL1FRACEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL1FRACEN` reader - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL1FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.
pub type PLL1FRACEN_R = crate::BitReader<PLL1FRACEN>;
impl PLL1FRACEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL1FRACEN {
        match self.bits {
            false => PLL1FRACEN::Reset,
            true => PLL1FRACEN::Set,
        }
    }
    ///Reset latch to tranfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PLL1FRACEN::Reset
    }
    ///Set latch to tranfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == PLL1FRACEN::Set
    }
}
///Field `PLL1FRACEN` writer - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL1FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.
pub type PLL1FRACEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL1FRACEN>;
impl<'a, REG> PLL1FRACEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset latch to tranfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1FRACEN::Reset)
    }
    ///Set latch to tranfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1FRACEN::Set)
    }
}
/**PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref1_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref1_ck</sub> must be between 1 and 2 MHz)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1VCOSEL {
    ///0: VCO frequency range 192 to 836 MHz
    WideVco = 0,
    ///1: VCO frequency range 150 to 420 MHz
    MediumVco = 1,
}
impl From<PLL1VCOSEL> for bool {
    #[inline(always)]
    fn from(variant: PLL1VCOSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL1VCOSEL` reader - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref1_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref1_ck</sub> must be between 1 and 2 MHz)
pub type PLL1VCOSEL_R = crate::BitReader<PLL1VCOSEL>;
impl PLL1VCOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL1VCOSEL {
        match self.bits {
            false => PLL1VCOSEL::WideVco,
            true => PLL1VCOSEL::MediumVco,
        }
    }
    ///VCO frequency range 192 to 836 MHz
    #[inline(always)]
    pub fn is_wide_vco(&self) -> bool {
        *self == PLL1VCOSEL::WideVco
    }
    ///VCO frequency range 150 to 420 MHz
    #[inline(always)]
    pub fn is_medium_vco(&self) -> bool {
        *self == PLL1VCOSEL::MediumVco
    }
}
///Field `PLL1VCOSEL` writer - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref1_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref1_ck</sub> must be between 1 and 2 MHz)
pub type PLL1VCOSEL_W<'a, REG> = crate::BitWriter<'a, REG, PLL1VCOSEL>;
impl<'a, REG> PLL1VCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VCO frequency range 192 to 836 MHz
    #[inline(always)]
    pub fn wide_vco(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1VCOSEL::WideVco)
    }
    ///VCO frequency range 150 to 420 MHz
    #[inline(always)]
    pub fn medium_vco(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1VCOSEL::MediumVco)
    }
}
///Field `PLL1SSCGEN` reader - PLL1 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL1, in order to reduce the amount of EMI peaks.
pub type PLL1SSCGEN_R = crate::BitReader;
///Field `PLL1SSCGEN` writer - PLL1 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL1, in order to reduce the amount of EMI peaks.
pub type PLL1SSCGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1RGE {
    ///0: Frequency is between 1 and 2 MHz
    Range1 = 0,
    ///1: Frequency is between 2 and 4 MHz
    Range2 = 1,
    ///2: Frequency is between 4 and 8 MHz
    Range4 = 2,
    ///3: Frequency is between 8 and 16 MHz
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
impl crate::IsEnum for PLL1RGE {}
///Field `PLL1RGE` reader - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1.
pub type PLL1RGE_R = crate::FieldReader<PLL1RGE>;
impl PLL1RGE_R {
    ///Get enumerated values variant
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
    ///Frequency is between 1 and 2 MHz
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == PLL1RGE::Range1
    }
    ///Frequency is between 2 and 4 MHz
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == PLL1RGE::Range2
    }
    ///Frequency is between 4 and 8 MHz
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == PLL1RGE::Range4
    }
    ///Frequency is between 8 and 16 MHz
    #[inline(always)]
    pub fn is_range8(&self) -> bool {
        *self == PLL1RGE::Range8
    }
}
///Field `PLL1RGE` writer - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1.
pub type PLL1RGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL1RGE, crate::Safe>;
impl<'a, REG> PLL1RGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Frequency is between 1 and 2 MHz
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE::Range1)
    }
    ///Frequency is between 2 and 4 MHz
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE::Range2)
    }
    ///Frequency is between 4 and 8 MHz
    #[inline(always)]
    pub fn range4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE::Range4)
    }
    ///Frequency is between 8 and 16 MHz
    #[inline(always)]
    pub fn range8(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE::Range8)
    }
}
/**PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. The hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock (SW=3). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIVP1EN {
    ///0: Clock ouput is disabled
    Disabled = 0,
    ///1: Clock output is enabled
    Enabled = 1,
}
impl From<DIVP1EN> for bool {
    #[inline(always)]
    fn from(variant: DIVP1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `DIVP1EN` reader - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. The hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock (SW=3). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled.
pub type DIVP1EN_R = crate::BitReader<DIVP1EN>;
impl DIVP1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIVP1EN {
        match self.bits {
            false => DIVP1EN::Disabled,
            true => DIVP1EN::Enabled,
        }
    }
    ///Clock ouput is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIVP1EN::Disabled
    }
    ///Clock output is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIVP1EN::Enabled
    }
}
///Field `DIVP1EN` writer - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. The hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock (SW=3). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled.
pub type DIVP1EN_W<'a, REG> = crate::BitWriter<'a, REG, DIVP1EN>;
impl<'a, REG> DIVP1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock ouput is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1EN::Disabled)
    }
    ///Clock output is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1EN::Enabled)
    }
}
///Field `DIVQ1EN` reader - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. The hardware prevents writing this bit if FMCCKP = 1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled.
pub use DIVP1EN_R as DIVQ1EN_R;
///Field `DIVR1EN` reader - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1DIVREN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used.
pub use DIVP1EN_R as DIVR1EN_R;
///Field `DIVS1EN` reader - PLL1 DIVS divider output enable Set and reset by software to enable the pll1_s_ck output of the PLL1. To save power, PLL1DIVSEN must be set to 0 when the pll1_s_ck is not used.
pub use DIVP1EN_R as DIVS1EN_R;
///Field `DIVT1EN` reader - PLL1 DIVT divider output enable Set and reset by software to enable the pll1_t_ck output of the PLL1. To save power, PLL1DIVTEN must be set to 0 when the pll1_t_ck is not used.
pub use DIVP1EN_R as DIVT1EN_R;
///Field `DIVQ1EN` writer - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. The hardware prevents writing this bit if FMCCKP = 1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled.
pub use DIVP1EN_W as DIVQ1EN_W;
///Field `DIVR1EN` writer - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1DIVREN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used.
pub use DIVP1EN_W as DIVR1EN_W;
///Field `DIVS1EN` writer - PLL1 DIVS divider output enable Set and reset by software to enable the pll1_s_ck output of the PLL1. To save power, PLL1DIVSEN must be set to 0 when the pll1_s_ck is not used.
pub use DIVP1EN_W as DIVS1EN_W;
///Field `DIVT1EN` writer - PLL1 DIVT divider output enable Set and reset by software to enable the pll1_t_ck output of the PLL1. To save power, PLL1DIVTEN must be set to 0 when the pll1_t_ck is not used.
pub use DIVP1EN_W as DIVT1EN_W;
///Field `PLL2FRACEN` reader - PLL2 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL2FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.
pub use PLL1FRACEN_R as PLL2FRACEN_R;
///Field `PLL2FRACEN` writer - PLL2 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL2FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.
pub use PLL1FRACEN_W as PLL2FRACEN_W;
///Field `PLL2VCOSEL` reader - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref2_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref2_ck</sub> must be between 1 and 2 MHz)
pub use PLL1VCOSEL_R as PLL2VCOSEL_R;
///Field `PLL2VCOSEL` writer - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref2_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref2_ck</sub> must be between 1 and 2 MHz)
pub use PLL1VCOSEL_W as PLL2VCOSEL_W;
///Field `PLL2SSCGEN` reader - PLL2 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL2, in order to reduce the amount of EMI peaks.
pub type PLL2SSCGEN_R = crate::BitReader;
///Field `PLL2SSCGEN` writer - PLL2 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL2, in order to reduce the amount of EMI peaks.
pub type PLL2SSCGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVP2EN` reader - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2DIVPEN and DIVP bits must be set to 0 when the pll2_p_ck is not used.
pub use DIVP1EN_R as DIVP2EN_R;
///Field `DIVQ2EN` reader - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL3DIVQEN and DIVQ bits must be set to 0 when the pll2_q_ck is not used.
pub use DIVP1EN_R as DIVQ2EN_R;
///Field `DIVR2EN` reader - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. The hardware prevents writing this bit if FMCCKP = 1. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
pub use DIVP1EN_R as DIVR2EN_R;
///Field `DIVS2EN` reader - PLL2 DIVS divider output enable Set and reset by software to enable the pll2_s_ck output of the PLL2. To save power, PLL2DIVSEN must be set to 0 when the pll2_s_ck is not used. The hardware prevents writing this bit if XSPICKP = 1.
pub use DIVP1EN_R as DIVS2EN_R;
///Field `DIVT2EN` reader - PLL2 DIVT divider output enable Set and reset by software to enable the pll2_t_ck output of the PLL2. To save power, PLL2DIVTEN must be set to 0 when the pll2_t_ck is not used. The hardware prevents writing this bit if XSPICKP = 1.
pub use DIVP1EN_R as DIVT2EN_R;
///Field `DIVP2EN` writer - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2DIVPEN and DIVP bits must be set to 0 when the pll2_p_ck is not used.
pub use DIVP1EN_W as DIVP2EN_W;
///Field `DIVQ2EN` writer - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL3DIVQEN and DIVQ bits must be set to 0 when the pll2_q_ck is not used.
pub use DIVP1EN_W as DIVQ2EN_W;
///Field `DIVR2EN` writer - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. The hardware prevents writing this bit if FMCCKP = 1. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
pub use DIVP1EN_W as DIVR2EN_W;
///Field `DIVS2EN` writer - PLL2 DIVS divider output enable Set and reset by software to enable the pll2_s_ck output of the PLL2. To save power, PLL2DIVSEN must be set to 0 when the pll2_s_ck is not used. The hardware prevents writing this bit if XSPICKP = 1.
pub use DIVP1EN_W as DIVS2EN_W;
///Field `DIVT2EN` writer - PLL2 DIVT divider output enable Set and reset by software to enable the pll2_t_ck output of the PLL2. To save power, PLL2DIVTEN must be set to 0 when the pll2_t_ck is not used. The hardware prevents writing this bit if XSPICKP = 1.
pub use DIVP1EN_W as DIVT2EN_W;
///Field `PLL3FRACEN` reader - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL3FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.
pub use PLL1FRACEN_R as PLL3FRACEN_R;
///Field `PLL3FRACEN` writer - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL3FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.
pub use PLL1FRACEN_W as PLL3FRACEN_W;
///Field `PLL2RGE` reader - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2.
pub use PLL1RGE_R as PLL2RGE_R;
///Field `PLL2RGE` writer - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2.
pub use PLL1RGE_W as PLL2RGE_W;
///Field `PLL3VCOSEL` reader - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref2_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref2_ck</sub> must be between 1 and 2 MHz)
pub use PLL1VCOSEL_R as PLL3VCOSEL_R;
///Field `PLL3VCOSEL` writer - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref2_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref2_ck</sub> must be between 1 and 2 MHz)
pub use PLL1VCOSEL_W as PLL3VCOSEL_W;
///Field `PLL3SSCGEN` reader - PLL3 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL3, in order to reduce the amount of EMI peaks.
pub type PLL3SSCGEN_R = crate::BitReader;
///Field `PLL3SSCGEN` writer - PLL3 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL3, in order to reduce the amount of EMI peaks.
pub type PLL3SSCGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVP3EN` reader - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
pub use DIVP1EN_R as DIVP3EN_R;
///Field `DIVQ3EN` reader - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
pub use DIVP1EN_R as DIVQ3EN_R;
///Field `DIVR3EN` reader - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
pub use DIVP1EN_R as DIVR3EN_R;
///Field `DIVS3EN` reader - PLL3 DIVS divider output enable Set and reset by software to enable the pll3_s_ck output of the PLL3. To save power, PLL3DIVSEN must be set to 0 when the pll3_s_ck is not used.
pub use DIVP1EN_R as DIVS3EN_R;
///Field `DIVT3EN` reader - PLL3 DIVT divider output enable Set and reset by software to enable the pll3_t_ck output of the PLL3. To save power, PLL1DIVTEN must be set to 0 when the pll3_t_ck is not used.
pub use DIVP1EN_R as DIVT3EN_R;
///Field `DIVP3EN` writer - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
pub use DIVP1EN_W as DIVP3EN_W;
///Field `DIVQ3EN` writer - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
pub use DIVP1EN_W as DIVQ3EN_W;
///Field `DIVR3EN` writer - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
pub use DIVP1EN_W as DIVR3EN_W;
///Field `DIVS3EN` writer - PLL3 DIVS divider output enable Set and reset by software to enable the pll3_s_ck output of the PLL3. To save power, PLL3DIVSEN must be set to 0 when the pll3_s_ck is not used.
pub use DIVP1EN_W as DIVS3EN_W;
///Field `DIVT3EN` writer - PLL3 DIVT divider output enable Set and reset by software to enable the pll3_t_ck output of the PLL3. To save power, PLL1DIVTEN must be set to 0 when the pll3_t_ck is not used.
pub use DIVP1EN_W as DIVT3EN_W;
///Field `PLL3RGE` reader - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. These bits must be written before enabling the PLL3.
pub use PLL1RGE_R as PLL3RGE_R;
///Field `PLL3RGE` writer - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. These bits must be written before enabling the PLL3.
pub use PLL1RGE_W as PLL3RGE_W;
impl R {
    ///Bit 0 - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL1FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.
    #[inline(always)]
    pub fn pll1fracen(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref1_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref1_ck</sub> must be between 1 and 2 MHz)
    #[inline(always)]
    pub fn pll1vcosel(&self) -> PLL1VCOSEL_R {
        PLL1VCOSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PLL1 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL1, in order to reduce the amount of EMI peaks.
    #[inline(always)]
    pub fn pll1sscgen(&self) -> PLL1SSCGEN_R {
        PLL1SSCGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1.
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. The hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock (SW=3). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled.
    #[inline(always)]
    pub fn divp1en(&self) -> DIVP1EN_R {
        DIVP1EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. The hardware prevents writing this bit if FMCCKP = 1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled.
    #[inline(always)]
    pub fn divq1en(&self) -> DIVQ1EN_R {
        DIVQ1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1DIVREN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used.
    #[inline(always)]
    pub fn divr1en(&self) -> DIVR1EN_R {
        DIVR1EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL1 DIVS divider output enable Set and reset by software to enable the pll1_s_ck output of the PLL1. To save power, PLL1DIVSEN must be set to 0 when the pll1_s_ck is not used.
    #[inline(always)]
    pub fn divs1en(&self) -> DIVS1EN_R {
        DIVS1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL1 DIVT divider output enable Set and reset by software to enable the pll1_t_ck output of the PLL1. To save power, PLL1DIVTEN must be set to 0 when the pll1_t_ck is not used.
    #[inline(always)]
    pub fn divt1en(&self) -> DIVT1EN_R {
        DIVT1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - PLL2 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL2FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.
    #[inline(always)]
    pub fn pll2fracen(&self) -> PLL2FRACEN_R {
        PLL2FRACEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref2_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref2_ck</sub> must be between 1 and 2 MHz)
    #[inline(always)]
    pub fn pll2vcosel(&self) -> PLL2VCOSEL_R {
        PLL2VCOSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - PLL2 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL2, in order to reduce the amount of EMI peaks.
    #[inline(always)]
    pub fn pll2sscgen(&self) -> PLL2SSCGEN_R {
        PLL2SSCGEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2.
    #[inline(always)]
    pub fn pll2rge(&self) -> PLL2RGE_R {
        PLL2RGE_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2DIVPEN and DIVP bits must be set to 0 when the pll2_p_ck is not used.
    #[inline(always)]
    pub fn divp2en(&self) -> DIVP2EN_R {
        DIVP2EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL3DIVQEN and DIVQ bits must be set to 0 when the pll2_q_ck is not used.
    #[inline(always)]
    pub fn divq2en(&self) -> DIVQ2EN_R {
        DIVQ2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. The hardware prevents writing this bit if FMCCKP = 1. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
    #[inline(always)]
    pub fn divr2en(&self) -> DIVR2EN_R {
        DIVR2EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PLL2 DIVS divider output enable Set and reset by software to enable the pll2_s_ck output of the PLL2. To save power, PLL2DIVSEN must be set to 0 when the pll2_s_ck is not used. The hardware prevents writing this bit if XSPICKP = 1.
    #[inline(always)]
    pub fn divs2en(&self) -> DIVS2EN_R {
        DIVS2EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - PLL2 DIVT divider output enable Set and reset by software to enable the pll2_t_ck output of the PLL2. To save power, PLL2DIVTEN must be set to 0 when the pll2_t_ck is not used. The hardware prevents writing this bit if XSPICKP = 1.
    #[inline(always)]
    pub fn divt2en(&self) -> DIVT2EN_R {
        DIVT2EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL3FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.
    #[inline(always)]
    pub fn pll3fracen(&self) -> PLL3FRACEN_R {
        PLL3FRACEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref2_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref2_ck</sub> must be between 1 and 2 MHz)
    #[inline(always)]
    pub fn pll3vcosel(&self) -> PLL3VCOSEL_R {
        PLL3VCOSEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - PLL3 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL3, in order to reduce the amount of EMI peaks.
    #[inline(always)]
    pub fn pll3sscgen(&self) -> PLL3SSCGEN_R {
        PLL3SSCGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. These bits must be written before enabling the PLL3.
    #[inline(always)]
    pub fn pll3rge(&self) -> PLL3RGE_R {
        PLL3RGE_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 27 - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
    #[inline(always)]
    pub fn divp3en(&self) -> DIVP3EN_R {
        DIVP3EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
    #[inline(always)]
    pub fn divq3en(&self) -> DIVQ3EN_R {
        DIVQ3EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
    #[inline(always)]
    pub fn divr3en(&self) -> DIVR3EN_R {
        DIVR3EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - PLL3 DIVS divider output enable Set and reset by software to enable the pll3_s_ck output of the PLL3. To save power, PLL3DIVSEN must be set to 0 when the pll3_s_ck is not used.
    #[inline(always)]
    pub fn divs3en(&self) -> DIVS3EN_R {
        DIVS3EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - PLL3 DIVT divider output enable Set and reset by software to enable the pll3_t_ck output of the PLL3. To save power, PLL1DIVTEN must be set to 0 when the pll3_t_ck is not used.
    #[inline(always)]
    pub fn divt3en(&self) -> DIVT3EN_R {
        DIVT3EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCFGR")
            .field("pll1fracen", &self.pll1fracen())
            .field("pll1vcosel", &self.pll1vcosel())
            .field("pll1sscgen", &self.pll1sscgen())
            .field("pll1rge", &self.pll1rge())
            .field("divp1en", &self.divp1en())
            .field("divq1en", &self.divq1en())
            .field("divr1en", &self.divr1en())
            .field("divs1en", &self.divs1en())
            .field("divt1en", &self.divt1en())
            .field("pll2fracen", &self.pll2fracen())
            .field("pll2vcosel", &self.pll2vcosel())
            .field("pll2sscgen", &self.pll2sscgen())
            .field("pll2rge", &self.pll2rge())
            .field("divp2en", &self.divp2en())
            .field("divq2en", &self.divq2en())
            .field("divr2en", &self.divr2en())
            .field("divs2en", &self.divs2en())
            .field("divt2en", &self.divt2en())
            .field("pll3fracen", &self.pll3fracen())
            .field("pll3vcosel", &self.pll3vcosel())
            .field("pll3sscgen", &self.pll3sscgen())
            .field("pll3rge", &self.pll3rge())
            .field("divp3en", &self.divp3en())
            .field("divq3en", &self.divq3en())
            .field("divr3en", &self.divr3en())
            .field("divs3en", &self.divs3en())
            .field("divt3en", &self.divt3en())
            .finish()
    }
}
impl W {
    ///Bit 0 - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL1FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.
    #[inline(always)]
    pub fn pll1fracen(&mut self) -> PLL1FRACEN_W<PLLCFGRrs> {
        PLL1FRACEN_W::new(self, 0)
    }
    ///Bit 1 - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref1_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref1_ck</sub> must be between 1 and 2 MHz)
    #[inline(always)]
    pub fn pll1vcosel(&mut self) -> PLL1VCOSEL_W<PLLCFGRrs> {
        PLL1VCOSEL_W::new(self, 1)
    }
    ///Bit 2 - PLL1 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL1, in order to reduce the amount of EMI peaks.
    #[inline(always)]
    pub fn pll1sscgen(&mut self) -> PLL1SSCGEN_W<PLLCFGRrs> {
        PLL1SSCGEN_W::new(self, 2)
    }
    ///Bits 3:4 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1.
    #[inline(always)]
    pub fn pll1rge(&mut self) -> PLL1RGE_W<PLLCFGRrs> {
        PLL1RGE_W::new(self, 3)
    }
    ///Bit 5 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. The hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock (SW=3). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled.
    #[inline(always)]
    pub fn divp1en(&mut self) -> DIVP1EN_W<PLLCFGRrs> {
        DIVP1EN_W::new(self, 5)
    }
    ///Bit 6 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. The hardware prevents writing this bit if FMCCKP = 1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled.
    #[inline(always)]
    pub fn divq1en(&mut self) -> DIVQ1EN_W<PLLCFGRrs> {
        DIVQ1EN_W::new(self, 6)
    }
    ///Bit 7 - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1DIVREN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used.
    #[inline(always)]
    pub fn divr1en(&mut self) -> DIVR1EN_W<PLLCFGRrs> {
        DIVR1EN_W::new(self, 7)
    }
    ///Bit 8 - PLL1 DIVS divider output enable Set and reset by software to enable the pll1_s_ck output of the PLL1. To save power, PLL1DIVSEN must be set to 0 when the pll1_s_ck is not used.
    #[inline(always)]
    pub fn divs1en(&mut self) -> DIVS1EN_W<PLLCFGRrs> {
        DIVS1EN_W::new(self, 8)
    }
    ///Bit 9 - PLL1 DIVT divider output enable Set and reset by software to enable the pll1_t_ck output of the PLL1. To save power, PLL1DIVTEN must be set to 0 when the pll1_t_ck is not used.
    #[inline(always)]
    pub fn divt1en(&mut self) -> DIVT1EN_W<PLLCFGRrs> {
        DIVT1EN_W::new(self, 9)
    }
    ///Bit 11 - PLL2 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL2FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.
    #[inline(always)]
    pub fn pll2fracen(&mut self) -> PLL2FRACEN_W<PLLCFGRrs> {
        PLL2FRACEN_W::new(self, 11)
    }
    ///Bit 12 - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref2_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref2_ck</sub> must be between 1 and 2 MHz)
    #[inline(always)]
    pub fn pll2vcosel(&mut self) -> PLL2VCOSEL_W<PLLCFGRrs> {
        PLL2VCOSEL_W::new(self, 12)
    }
    ///Bit 13 - PLL2 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL2, in order to reduce the amount of EMI peaks.
    #[inline(always)]
    pub fn pll2sscgen(&mut self) -> PLL2SSCGEN_W<PLLCFGRrs> {
        PLL2SSCGEN_W::new(self, 13)
    }
    ///Bits 14:15 - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2.
    #[inline(always)]
    pub fn pll2rge(&mut self) -> PLL2RGE_W<PLLCFGRrs> {
        PLL2RGE_W::new(self, 14)
    }
    ///Bit 16 - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2DIVPEN and DIVP bits must be set to 0 when the pll2_p_ck is not used.
    #[inline(always)]
    pub fn divp2en(&mut self) -> DIVP2EN_W<PLLCFGRrs> {
        DIVP2EN_W::new(self, 16)
    }
    ///Bit 17 - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL3DIVQEN and DIVQ bits must be set to 0 when the pll2_q_ck is not used.
    #[inline(always)]
    pub fn divq2en(&mut self) -> DIVQ2EN_W<PLLCFGRrs> {
        DIVQ2EN_W::new(self, 17)
    }
    ///Bit 18 - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. The hardware prevents writing this bit if FMCCKP = 1. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
    #[inline(always)]
    pub fn divr2en(&mut self) -> DIVR2EN_W<PLLCFGRrs> {
        DIVR2EN_W::new(self, 18)
    }
    ///Bit 19 - PLL2 DIVS divider output enable Set and reset by software to enable the pll2_s_ck output of the PLL2. To save power, PLL2DIVSEN must be set to 0 when the pll2_s_ck is not used. The hardware prevents writing this bit if XSPICKP = 1.
    #[inline(always)]
    pub fn divs2en(&mut self) -> DIVS2EN_W<PLLCFGRrs> {
        DIVS2EN_W::new(self, 19)
    }
    ///Bit 20 - PLL2 DIVT divider output enable Set and reset by software to enable the pll2_t_ck output of the PLL2. To save power, PLL2DIVTEN must be set to 0 when the pll2_t_ck is not used. The hardware prevents writing this bit if XSPICKP = 1.
    #[inline(always)]
    pub fn divt2en(&mut self) -> DIVT2EN_W<PLLCFGRrs> {
        DIVT2EN_W::new(self, 20)
    }
    ///Bit 22 - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN into the sigma-delta modulator. In order to latch the FRACN value into the sigma-delta modulator, PLL3FRACLE must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN into the modulator. Refer to PLL initialization procedure on page 444 for additional information.
    #[inline(always)]
    pub fn pll3fracen(&mut self) -> PLL3FRACEN_W<PLLCFGRrs> {
        PLL3FRACEN_W::new(self, 22)
    }
    ///Bit 23 - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3. It allows the application to select the VCO range: VCOH: working from 400 to 1600 MHz (F<sub>ref2_ck</sub> must be between 2 and 16 MHz) VCOL: working from 150 to 420 MHz (F<sub>ref2_ck</sub> must be between 1 and 2 MHz)
    #[inline(always)]
    pub fn pll3vcosel(&mut self) -> PLL3VCOSEL_W<PLLCFGRrs> {
        PLL3VCOSEL_W::new(self, 23)
    }
    ///Bit 24 - PLL3 SSCG enable Set and reset by software to enable the Spread Spectrum Clock Generator of PLL3, in order to reduce the amount of EMI peaks.
    #[inline(always)]
    pub fn pll3sscgen(&mut self) -> PLL3SSCGEN_W<PLLCFGRrs> {
        PLL3SSCGEN_W::new(self, 24)
    }
    ///Bits 25:26 - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. These bits must be written before enabling the PLL3.
    #[inline(always)]
    pub fn pll3rge(&mut self) -> PLL3RGE_W<PLLCFGRrs> {
        PLL3RGE_W::new(self, 25)
    }
    ///Bit 27 - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
    #[inline(always)]
    pub fn divp3en(&mut self) -> DIVP3EN_W<PLLCFGRrs> {
        DIVP3EN_W::new(self, 27)
    }
    ///Bit 28 - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
    #[inline(always)]
    pub fn divq3en(&mut self) -> DIVQ3EN_W<PLLCFGRrs> {
        DIVQ3EN_W::new(self, 28)
    }
    ///Bit 29 - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3DIVREN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used.
    #[inline(always)]
    pub fn divr3en(&mut self) -> DIVR3EN_W<PLLCFGRrs> {
        DIVR3EN_W::new(self, 29)
    }
    ///Bit 30 - PLL3 DIVS divider output enable Set and reset by software to enable the pll3_s_ck output of the PLL3. To save power, PLL3DIVSEN must be set to 0 when the pll3_s_ck is not used.
    #[inline(always)]
    pub fn divs3en(&mut self) -> DIVS3EN_W<PLLCFGRrs> {
        DIVS3EN_W::new(self, 30)
    }
    ///Bit 31 - PLL3 DIVT divider output enable Set and reset by software to enable the pll3_t_ck output of the PLL3. To save power, PLL1DIVTEN must be set to 0 when the pll3_t_ck is not used.
    #[inline(always)]
    pub fn divt3en(&mut self) -> DIVT3EN_W<PLLCFGRrs> {
        DIVT3EN_W::new(self, 31)
    }
}
/**RCC PLLs configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:PLLCFGR)*/
pub struct PLLCFGRrs;
impl crate::RegisterSpec for PLLCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pllcfgr::R`](R) reader structure
impl crate::Readable for PLLCFGRrs {}
///`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure
impl crate::Writable for PLLCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLCFGR to value 0
impl crate::Resettable for PLLCFGRrs {}
