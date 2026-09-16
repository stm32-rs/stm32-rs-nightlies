///Register `PLLCFGR` reader
pub type R = crate::R<PLLCFGRrs>;
///Register `PLLCFGR` writer
pub type W = crate::W<PLLCFGRrs>;
/**PLL1 fractional latch enable

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
///Field `PLL1FRACEN` reader - PLL1 fractional latch enable
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
///Field `PLL1FRACEN` writer - PLL1 fractional latch enable
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
/**PLL1 VCO selection

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
///Field `PLL1VCOSEL` reader - PLL1 VCO selection
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
///Field `PLL1VCOSEL` writer - PLL1 VCO selection
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
///Field `PLL1SSCGEN` reader - PLL1 SSCG enable
pub type PLL1SSCGEN_R = crate::BitReader;
///Field `PLL1SSCGEN` writer - PLL1 SSCG enable
pub type PLL1SSCGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**PLL1 input frequency range

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
///Field `PLL1RGE` reader - PLL1 input frequency range
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
///Field `PLL1RGE` writer - PLL1 input frequency range
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
/**PLL1 DIVP divider output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1PEN {
    ///0: Clock ouput is disabled
    Disabled = 0,
    ///1: Clock output is enabled
    Enabled = 1,
}
impl From<PLL1PEN> for bool {
    #[inline(always)]
    fn from(variant: PLL1PEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL1PEN` reader - PLL1 DIVP divider output enable
pub type PLL1PEN_R = crate::BitReader<PLL1PEN>;
impl PLL1PEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL1PEN {
        match self.bits {
            false => PLL1PEN::Disabled,
            true => PLL1PEN::Enabled,
        }
    }
    ///Clock ouput is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL1PEN::Disabled
    }
    ///Clock output is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL1PEN::Enabled
    }
}
///Field `PLL1PEN` writer - PLL1 DIVP divider output enable
pub type PLL1PEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL1PEN>;
impl<'a, REG> PLL1PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock ouput is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1PEN::Disabled)
    }
    ///Clock output is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1PEN::Enabled)
    }
}
///Field `PLL1QEN` reader - PLL1 DIVQ divider output enable
pub use PLL1PEN_R as PLL1QEN_R;
///Field `PLL1SEN` reader - PLL1 DIVS divider output enable
pub use PLL1PEN_R as PLL1SEN_R;
///Field `PLL1QEN` writer - PLL1 DIVQ divider output enable
pub use PLL1PEN_W as PLL1QEN_W;
///Field `PLL1SEN` writer - PLL1 DIVS divider output enable
pub use PLL1PEN_W as PLL1SEN_W;
///Field `PLL2FRACLEN` reader - PLL2 fractional latch enable
pub type PLL2FRACLEN_R = crate::BitReader;
///Field `PLL2FRACLEN` writer - PLL2 fractional latch enable
pub type PLL2FRACLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2VCOSEL` reader - PLL2 VCO selection
pub use PLL1VCOSEL_R as PLL2VCOSEL_R;
///Field `PLL2VCOSEL` writer - PLL2 VCO selection
pub use PLL1VCOSEL_W as PLL2VCOSEL_W;
///Field `PLL2SSCGEN` reader - PLL2 SSCG enable
pub type PLL2SSCGEN_R = crate::BitReader;
///Field `PLL2SSCGEN` writer - PLL2 SSCG enable
pub type PLL2SSCGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3FRACEN` reader - PLL3 fractional latch enable
pub use PLL1FRACEN_R as PLL3FRACEN_R;
///Field `PLL3FRACEN` writer - PLL3 fractional latch enable
pub use PLL1FRACEN_W as PLL3FRACEN_W;
///Field `PLL2PEN` reader - PLL2 DIVP divider output enable
pub use PLL1PEN_R as PLL2PEN_R;
///Field `PLL2QEN` reader - PLL2 DIVQ divider output enable
pub use PLL1PEN_R as PLL2QEN_R;
///Field `PLL2REN` reader - PLL2 DIVR divider output enable
pub use PLL1PEN_R as PLL2REN_R;
///Field `PLL2SEN` reader - PLL2 DIVS divider output enable
pub use PLL1PEN_R as PLL2SEN_R;
///Field `PLL2TEN` reader - PLL2 DIVT divider output enable
pub use PLL1PEN_R as PLL2TEN_R;
///Field `PLL2PEN` writer - PLL2 DIVP divider output enable
pub use PLL1PEN_W as PLL2PEN_W;
///Field `PLL2QEN` writer - PLL2 DIVQ divider output enable
pub use PLL1PEN_W as PLL2QEN_W;
///Field `PLL2REN` writer - PLL2 DIVR divider output enable
pub use PLL1PEN_W as PLL2REN_W;
///Field `PLL2SEN` writer - PLL2 DIVS divider output enable
pub use PLL1PEN_W as PLL2SEN_W;
///Field `PLL2TEN` writer - PLL2 DIVT divider output enable
pub use PLL1PEN_W as PLL2TEN_W;
///Field `PLL2RGE` reader - PLL2 input frequency range
pub use PLL1RGE_R as PLL2RGE_R;
///Field `PLL2RGE` writer - PLL2 input frequency range
pub use PLL1RGE_W as PLL2RGE_W;
///Field `PLL3VCOSEL` reader - PLL3 VCO selection
pub use PLL1VCOSEL_R as PLL3VCOSEL_R;
///Field `PLL3VCOSEL` writer - PLL3 VCO selection
pub use PLL1VCOSEL_W as PLL3VCOSEL_W;
///Field `PLL3SSCGEN` reader - PLL3 SSCG enable
pub type PLL3SSCGEN_R = crate::BitReader;
///Field `PLL3SSCGEN` writer - PLL3 SSCG enable
pub type PLL3SSCGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3PEN` reader - PLL3 DIVP divider output enable
pub use PLL1PEN_R as PLL3PEN_R;
///Field `PLL3QEN` reader - PLL3 DIVQ divider output enable
pub use PLL1PEN_R as PLL3QEN_R;
///Field `PLL3REN` reader - PLL3 DIVR divider output enable
pub use PLL1PEN_R as PLL3REN_R;
///Field `PLL3SEN` reader - PLL3 DIVS divider output enable
pub use PLL1PEN_R as PLL3SEN_R;
///Field `PLL3PEN` writer - PLL3 DIVP divider output enable
pub use PLL1PEN_W as PLL3PEN_W;
///Field `PLL3QEN` writer - PLL3 DIVQ divider output enable
pub use PLL1PEN_W as PLL3QEN_W;
///Field `PLL3REN` writer - PLL3 DIVR divider output enable
pub use PLL1PEN_W as PLL3REN_W;
///Field `PLL3SEN` writer - PLL3 DIVS divider output enable
pub use PLL1PEN_W as PLL3SEN_W;
///Field `PLL3RGE` reader - PLL3 input frequency range
pub use PLL1RGE_R as PLL3RGE_R;
///Field `PLL3RGE` writer - PLL3 input frequency range
pub use PLL1RGE_W as PLL3RGE_W;
impl R {
    ///Bit 0 - PLL1 fractional latch enable
    #[inline(always)]
    pub fn pll1fracen(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PLL1 VCO selection
    #[inline(always)]
    pub fn pll1vcosel(&self) -> PLL1VCOSEL_R {
        PLL1VCOSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PLL1 SSCG enable
    #[inline(always)]
    pub fn pll1sscgen(&self) -> PLL1SSCGEN_R {
        PLL1SSCGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - PLL1 input frequency range
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - PLL1 DIVP divider output enable
    #[inline(always)]
    pub fn pll1pen(&self) -> PLL1PEN_R {
        PLL1PEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLL1 DIVQ divider output enable
    #[inline(always)]
    pub fn pll1qen(&self) -> PLL1QEN_R {
        PLL1QEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - PLL1 DIVS divider output enable
    #[inline(always)]
    pub fn pll1sen(&self) -> PLL1SEN_R {
        PLL1SEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - PLL2 fractional latch enable
    #[inline(always)]
    pub fn pll2fraclen(&self) -> PLL2FRACLEN_R {
        PLL2FRACLEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PLL2 VCO selection
    #[inline(always)]
    pub fn pll2vcosel(&self) -> PLL2VCOSEL_R {
        PLL2VCOSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - PLL2 SSCG enable
    #[inline(always)]
    pub fn pll2sscgen(&self) -> PLL2SSCGEN_R {
        PLL2SSCGEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - PLL2 input frequency range
    #[inline(always)]
    pub fn pll2rge(&self) -> PLL2RGE_R {
        PLL2RGE_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - PLL2 DIVP divider output enable
    #[inline(always)]
    pub fn pll2pen(&self) -> PLL2PEN_R {
        PLL2PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL2 DIVQ divider output enable
    #[inline(always)]
    pub fn pll2qen(&self) -> PLL2QEN_R {
        PLL2QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL2 DIVR divider output enable
    #[inline(always)]
    pub fn pll2ren(&self) -> PLL2REN_R {
        PLL2REN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PLL2 DIVS divider output enable
    #[inline(always)]
    pub fn pll2sen(&self) -> PLL2SEN_R {
        PLL2SEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - PLL2 DIVT divider output enable
    #[inline(always)]
    pub fn pll2ten(&self) -> PLL2TEN_R {
        PLL2TEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - PLL3 fractional latch enable
    #[inline(always)]
    pub fn pll3fracen(&self) -> PLL3FRACEN_R {
        PLL3FRACEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - PLL3 VCO selection
    #[inline(always)]
    pub fn pll3vcosel(&self) -> PLL3VCOSEL_R {
        PLL3VCOSEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - PLL3 SSCG enable
    #[inline(always)]
    pub fn pll3sscgen(&self) -> PLL3SSCGEN_R {
        PLL3SSCGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - PLL3 input frequency range
    #[inline(always)]
    pub fn pll3rge(&self) -> PLL3RGE_R {
        PLL3RGE_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 27 - PLL3 DIVP divider output enable
    #[inline(always)]
    pub fn pll3pen(&self) -> PLL3PEN_R {
        PLL3PEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PLL3 DIVQ divider output enable
    #[inline(always)]
    pub fn pll3qen(&self) -> PLL3QEN_R {
        PLL3QEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - PLL3 DIVR divider output enable
    #[inline(always)]
    pub fn pll3ren(&self) -> PLL3REN_R {
        PLL3REN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - PLL3 DIVS divider output enable
    #[inline(always)]
    pub fn pll3sen(&self) -> PLL3SEN_R {
        PLL3SEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCFGR")
            .field("pll1fracen", &self.pll1fracen())
            .field("pll1vcosel", &self.pll1vcosel())
            .field("pll1sscgen", &self.pll1sscgen())
            .field("pll1rge", &self.pll1rge())
            .field("pll1pen", &self.pll1pen())
            .field("pll1qen", &self.pll1qen())
            .field("pll1sen", &self.pll1sen())
            .field("pll2fraclen", &self.pll2fraclen())
            .field("pll2vcosel", &self.pll2vcosel())
            .field("pll2sscgen", &self.pll2sscgen())
            .field("pll2rge", &self.pll2rge())
            .field("pll2pen", &self.pll2pen())
            .field("pll2qen", &self.pll2qen())
            .field("pll2ren", &self.pll2ren())
            .field("pll2sen", &self.pll2sen())
            .field("pll2ten", &self.pll2ten())
            .field("pll3fracen", &self.pll3fracen())
            .field("pll3vcosel", &self.pll3vcosel())
            .field("pll3sscgen", &self.pll3sscgen())
            .field("pll3rge", &self.pll3rge())
            .field("pll3pen", &self.pll3pen())
            .field("pll3qen", &self.pll3qen())
            .field("pll3ren", &self.pll3ren())
            .field("pll3sen", &self.pll3sen())
            .finish()
    }
}
impl W {
    ///Bit 0 - PLL1 fractional latch enable
    #[inline(always)]
    pub fn pll1fracen(&mut self) -> PLL1FRACEN_W<'_, PLLCFGRrs> {
        PLL1FRACEN_W::new(self, 0)
    }
    ///Bit 1 - PLL1 VCO selection
    #[inline(always)]
    pub fn pll1vcosel(&mut self) -> PLL1VCOSEL_W<'_, PLLCFGRrs> {
        PLL1VCOSEL_W::new(self, 1)
    }
    ///Bit 2 - PLL1 SSCG enable
    #[inline(always)]
    pub fn pll1sscgen(&mut self) -> PLL1SSCGEN_W<'_, PLLCFGRrs> {
        PLL1SSCGEN_W::new(self, 2)
    }
    ///Bits 3:4 - PLL1 input frequency range
    #[inline(always)]
    pub fn pll1rge(&mut self) -> PLL1RGE_W<'_, PLLCFGRrs> {
        PLL1RGE_W::new(self, 3)
    }
    ///Bit 5 - PLL1 DIVP divider output enable
    #[inline(always)]
    pub fn pll1pen(&mut self) -> PLL1PEN_W<'_, PLLCFGRrs> {
        PLL1PEN_W::new(self, 5)
    }
    ///Bit 6 - PLL1 DIVQ divider output enable
    #[inline(always)]
    pub fn pll1qen(&mut self) -> PLL1QEN_W<'_, PLLCFGRrs> {
        PLL1QEN_W::new(self, 6)
    }
    ///Bit 8 - PLL1 DIVS divider output enable
    #[inline(always)]
    pub fn pll1sen(&mut self) -> PLL1SEN_W<'_, PLLCFGRrs> {
        PLL1SEN_W::new(self, 8)
    }
    ///Bit 11 - PLL2 fractional latch enable
    #[inline(always)]
    pub fn pll2fraclen(&mut self) -> PLL2FRACLEN_W<'_, PLLCFGRrs> {
        PLL2FRACLEN_W::new(self, 11)
    }
    ///Bit 12 - PLL2 VCO selection
    #[inline(always)]
    pub fn pll2vcosel(&mut self) -> PLL2VCOSEL_W<'_, PLLCFGRrs> {
        PLL2VCOSEL_W::new(self, 12)
    }
    ///Bit 13 - PLL2 SSCG enable
    #[inline(always)]
    pub fn pll2sscgen(&mut self) -> PLL2SSCGEN_W<'_, PLLCFGRrs> {
        PLL2SSCGEN_W::new(self, 13)
    }
    ///Bits 14:15 - PLL2 input frequency range
    #[inline(always)]
    pub fn pll2rge(&mut self) -> PLL2RGE_W<'_, PLLCFGRrs> {
        PLL2RGE_W::new(self, 14)
    }
    ///Bit 16 - PLL2 DIVP divider output enable
    #[inline(always)]
    pub fn pll2pen(&mut self) -> PLL2PEN_W<'_, PLLCFGRrs> {
        PLL2PEN_W::new(self, 16)
    }
    ///Bit 17 - PLL2 DIVQ divider output enable
    #[inline(always)]
    pub fn pll2qen(&mut self) -> PLL2QEN_W<'_, PLLCFGRrs> {
        PLL2QEN_W::new(self, 17)
    }
    ///Bit 18 - PLL2 DIVR divider output enable
    #[inline(always)]
    pub fn pll2ren(&mut self) -> PLL2REN_W<'_, PLLCFGRrs> {
        PLL2REN_W::new(self, 18)
    }
    ///Bit 19 - PLL2 DIVS divider output enable
    #[inline(always)]
    pub fn pll2sen(&mut self) -> PLL2SEN_W<'_, PLLCFGRrs> {
        PLL2SEN_W::new(self, 19)
    }
    ///Bit 20 - PLL2 DIVT divider output enable
    #[inline(always)]
    pub fn pll2ten(&mut self) -> PLL2TEN_W<'_, PLLCFGRrs> {
        PLL2TEN_W::new(self, 20)
    }
    ///Bit 22 - PLL3 fractional latch enable
    #[inline(always)]
    pub fn pll3fracen(&mut self) -> PLL3FRACEN_W<'_, PLLCFGRrs> {
        PLL3FRACEN_W::new(self, 22)
    }
    ///Bit 23 - PLL3 VCO selection
    #[inline(always)]
    pub fn pll3vcosel(&mut self) -> PLL3VCOSEL_W<'_, PLLCFGRrs> {
        PLL3VCOSEL_W::new(self, 23)
    }
    ///Bit 24 - PLL3 SSCG enable
    #[inline(always)]
    pub fn pll3sscgen(&mut self) -> PLL3SSCGEN_W<'_, PLLCFGRrs> {
        PLL3SSCGEN_W::new(self, 24)
    }
    ///Bits 25:26 - PLL3 input frequency range
    #[inline(always)]
    pub fn pll3rge(&mut self) -> PLL3RGE_W<'_, PLLCFGRrs> {
        PLL3RGE_W::new(self, 25)
    }
    ///Bit 27 - PLL3 DIVP divider output enable
    #[inline(always)]
    pub fn pll3pen(&mut self) -> PLL3PEN_W<'_, PLLCFGRrs> {
        PLL3PEN_W::new(self, 27)
    }
    ///Bit 28 - PLL3 DIVQ divider output enable
    #[inline(always)]
    pub fn pll3qen(&mut self) -> PLL3QEN_W<'_, PLLCFGRrs> {
        PLL3QEN_W::new(self, 28)
    }
    ///Bit 29 - PLL3 DIVR divider output enable
    #[inline(always)]
    pub fn pll3ren(&mut self) -> PLL3REN_W<'_, PLLCFGRrs> {
        PLL3REN_W::new(self, 29)
    }
    ///Bit 30 - PLL3 DIVS divider output enable
    #[inline(always)]
    pub fn pll3sen(&mut self) -> PLL3SEN_W<'_, PLLCFGRrs> {
        PLL3SEN_W::new(self, 30)
    }
}
/**RCC PLLs configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLLCFGR)*/
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
