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
///Field `PLL2FRACEN` reader - PLL2 fractional latch enable
pub use PLL1FRACEN_R as PLL2FRACEN_R;
///Field `PLL3FRACEN` reader - PLL3 fractional latch enable
pub use PLL1FRACEN_R as PLL3FRACEN_R;
///Field `PLL2FRACEN` writer - PLL2 fractional latch enable
pub use PLL1FRACEN_W as PLL2FRACEN_W;
///Field `PLL3FRACEN` writer - PLL3 fractional latch enable
pub use PLL1FRACEN_W as PLL3FRACEN_W;
///Field `PLL2RGE` reader - PLL2 input frequency range
pub use PLL1RGE_R as PLL2RGE_R;
///Field `PLL3RGE` reader - PLL3 input frequency range
pub use PLL1RGE_R as PLL3RGE_R;
///Field `PLL2RGE` writer - PLL2 input frequency range
pub use PLL1RGE_W as PLL2RGE_W;
///Field `PLL3RGE` writer - PLL3 input frequency range
pub use PLL1RGE_W as PLL3RGE_W;
///Field `PLL2VCOSEL` reader - PLL2 VCO selection
pub use PLL1VCOSEL_R as PLL2VCOSEL_R;
///Field `PLL3VCOSEL` reader - PLL3 VCO selection
pub use PLL1VCOSEL_R as PLL3VCOSEL_R;
///Field `PLL2VCOSEL` writer - PLL2 VCO selection
pub use PLL1VCOSEL_W as PLL2VCOSEL_W;
///Field `PLL3VCOSEL` writer - PLL3 VCO selection
pub use PLL1VCOSEL_W as PLL3VCOSEL_W;
/**PLL1 DIVP divider output enable

Value on reset: 1*/
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
///Field `DIVP1EN` reader - PLL1 DIVP divider output enable
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
///Field `DIVP1EN` writer - PLL1 DIVP divider output enable
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
///Field `DIVQ1EN` reader - PLL1 DIVQ divider output enable
pub use DIVP1EN_R as DIVQ1EN_R;
///Field `DIVR1EN` reader - PLL1 DIVR divider output enable
pub use DIVP1EN_R as DIVR1EN_R;
///Field `DIVP2EN` reader - PLL2 DIVP divider output enable
pub use DIVP1EN_R as DIVP2EN_R;
///Field `DIVQ2EN` reader - PLL2 DIVQ divider output enable
pub use DIVP1EN_R as DIVQ2EN_R;
///Field `DIVR2EN` reader - PLL2 DIVR divider output enable
pub use DIVP1EN_R as DIVR2EN_R;
///Field `DIVP3EN` reader - PLL3 DIVP divider output enable
pub use DIVP1EN_R as DIVP3EN_R;
///Field `DIVQ3EN` reader - PLL3 DIVQ divider output enable
pub use DIVP1EN_R as DIVQ3EN_R;
///Field `DIVR3EN` reader - PLL3 DIVR divider output enable
pub use DIVP1EN_R as DIVR3EN_R;
///Field `DIVQ1EN` writer - PLL1 DIVQ divider output enable
pub use DIVP1EN_W as DIVQ1EN_W;
///Field `DIVR1EN` writer - PLL1 DIVR divider output enable
pub use DIVP1EN_W as DIVR1EN_W;
///Field `DIVP2EN` writer - PLL2 DIVP divider output enable
pub use DIVP1EN_W as DIVP2EN_W;
///Field `DIVQ2EN` writer - PLL2 DIVQ divider output enable
pub use DIVP1EN_W as DIVQ2EN_W;
///Field `DIVR2EN` writer - PLL2 DIVR divider output enable
pub use DIVP1EN_W as DIVR2EN_W;
///Field `DIVP3EN` writer - PLL3 DIVP divider output enable
pub use DIVP1EN_W as DIVP3EN_W;
///Field `DIVQ3EN` writer - PLL3 DIVQ divider output enable
pub use DIVP1EN_W as DIVQ3EN_W;
///Field `DIVR3EN` writer - PLL3 DIVR divider output enable
pub use DIVP1EN_W as DIVR3EN_W;
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
    ///Bits 2:3 - PLL1 input frequency range
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PLL2 fractional latch enable
    #[inline(always)]
    pub fn pll2fracen(&self) -> PLL2FRACEN_R {
        PLL2FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PLL2 VCO selection
    #[inline(always)]
    pub fn pll2vcosel(&self) -> PLL2VCOSEL_R {
        PLL2VCOSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - PLL2 input frequency range
    #[inline(always)]
    pub fn pll2rge(&self) -> PLL2RGE_R {
        PLL2RGE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - PLL3 fractional latch enable
    #[inline(always)]
    pub fn pll3fracen(&self) -> PLL3FRACEN_R {
        PLL3FRACEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL3 VCO selection
    #[inline(always)]
    pub fn pll3vcosel(&self) -> PLL3VCOSEL_R {
        PLL3VCOSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - PLL3 input frequency range
    #[inline(always)]
    pub fn pll3rge(&self) -> PLL3RGE_R {
        PLL3RGE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 16 - PLL1 DIVP divider output enable
    #[inline(always)]
    pub fn divp1en(&self) -> DIVP1EN_R {
        DIVP1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL1 DIVQ divider output enable
    #[inline(always)]
    pub fn divq1en(&self) -> DIVQ1EN_R {
        DIVQ1EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL1 DIVR divider output enable
    #[inline(always)]
    pub fn divr1en(&self) -> DIVR1EN_R {
        DIVR1EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PLL2 DIVP divider output enable
    #[inline(always)]
    pub fn divp2en(&self) -> DIVP2EN_R {
        DIVP2EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - PLL2 DIVQ divider output enable
    #[inline(always)]
    pub fn divq2en(&self) -> DIVQ2EN_R {
        DIVQ2EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - PLL2 DIVR divider output enable
    #[inline(always)]
    pub fn divr2en(&self) -> DIVR2EN_R {
        DIVR2EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - PLL3 DIVP divider output enable
    #[inline(always)]
    pub fn divp3en(&self) -> DIVP3EN_R {
        DIVP3EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - PLL3 DIVQ divider output enable
    #[inline(always)]
    pub fn divq3en(&self) -> DIVQ3EN_R {
        DIVQ3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - PLL3 DIVR divider output enable
    #[inline(always)]
    pub fn divr3en(&self) -> DIVR3EN_R {
        DIVR3EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCFGR")
            .field("pll1fracen", &self.pll1fracen())
            .field("pll1vcosel", &self.pll1vcosel())
            .field("pll1rge", &self.pll1rge())
            .field("pll2fracen", &self.pll2fracen())
            .field("pll2vcosel", &self.pll2vcosel())
            .field("pll2rge", &self.pll2rge())
            .field("pll3fracen", &self.pll3fracen())
            .field("pll3vcosel", &self.pll3vcosel())
            .field("pll3rge", &self.pll3rge())
            .field("divp1en", &self.divp1en())
            .field("divq1en", &self.divq1en())
            .field("divr1en", &self.divr1en())
            .field("divp2en", &self.divp2en())
            .field("divq2en", &self.divq2en())
            .field("divr2en", &self.divr2en())
            .field("divp3en", &self.divp3en())
            .field("divq3en", &self.divq3en())
            .field("divr3en", &self.divr3en())
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
    ///Bits 2:3 - PLL1 input frequency range
    #[inline(always)]
    pub fn pll1rge(&mut self) -> PLL1RGE_W<'_, PLLCFGRrs> {
        PLL1RGE_W::new(self, 2)
    }
    ///Bit 4 - PLL2 fractional latch enable
    #[inline(always)]
    pub fn pll2fracen(&mut self) -> PLL2FRACEN_W<'_, PLLCFGRrs> {
        PLL2FRACEN_W::new(self, 4)
    }
    ///Bit 5 - PLL2 VCO selection
    #[inline(always)]
    pub fn pll2vcosel(&mut self) -> PLL2VCOSEL_W<'_, PLLCFGRrs> {
        PLL2VCOSEL_W::new(self, 5)
    }
    ///Bits 6:7 - PLL2 input frequency range
    #[inline(always)]
    pub fn pll2rge(&mut self) -> PLL2RGE_W<'_, PLLCFGRrs> {
        PLL2RGE_W::new(self, 6)
    }
    ///Bit 8 - PLL3 fractional latch enable
    #[inline(always)]
    pub fn pll3fracen(&mut self) -> PLL3FRACEN_W<'_, PLLCFGRrs> {
        PLL3FRACEN_W::new(self, 8)
    }
    ///Bit 9 - PLL3 VCO selection
    #[inline(always)]
    pub fn pll3vcosel(&mut self) -> PLL3VCOSEL_W<'_, PLLCFGRrs> {
        PLL3VCOSEL_W::new(self, 9)
    }
    ///Bits 10:11 - PLL3 input frequency range
    #[inline(always)]
    pub fn pll3rge(&mut self) -> PLL3RGE_W<'_, PLLCFGRrs> {
        PLL3RGE_W::new(self, 10)
    }
    ///Bit 16 - PLL1 DIVP divider output enable
    #[inline(always)]
    pub fn divp1en(&mut self) -> DIVP1EN_W<'_, PLLCFGRrs> {
        DIVP1EN_W::new(self, 16)
    }
    ///Bit 17 - PLL1 DIVQ divider output enable
    #[inline(always)]
    pub fn divq1en(&mut self) -> DIVQ1EN_W<'_, PLLCFGRrs> {
        DIVQ1EN_W::new(self, 17)
    }
    ///Bit 18 - PLL1 DIVR divider output enable
    #[inline(always)]
    pub fn divr1en(&mut self) -> DIVR1EN_W<'_, PLLCFGRrs> {
        DIVR1EN_W::new(self, 18)
    }
    ///Bit 19 - PLL2 DIVP divider output enable
    #[inline(always)]
    pub fn divp2en(&mut self) -> DIVP2EN_W<'_, PLLCFGRrs> {
        DIVP2EN_W::new(self, 19)
    }
    ///Bit 20 - PLL2 DIVQ divider output enable
    #[inline(always)]
    pub fn divq2en(&mut self) -> DIVQ2EN_W<'_, PLLCFGRrs> {
        DIVQ2EN_W::new(self, 20)
    }
    ///Bit 21 - PLL2 DIVR divider output enable
    #[inline(always)]
    pub fn divr2en(&mut self) -> DIVR2EN_W<'_, PLLCFGRrs> {
        DIVR2EN_W::new(self, 21)
    }
    ///Bit 22 - PLL3 DIVP divider output enable
    #[inline(always)]
    pub fn divp3en(&mut self) -> DIVP3EN_W<'_, PLLCFGRrs> {
        DIVP3EN_W::new(self, 22)
    }
    ///Bit 23 - PLL3 DIVQ divider output enable
    #[inline(always)]
    pub fn divq3en(&mut self) -> DIVQ3EN_W<'_, PLLCFGRrs> {
        DIVQ3EN_W::new(self, 23)
    }
    ///Bit 24 - PLL3 DIVR divider output enable
    #[inline(always)]
    pub fn divr3en(&mut self) -> DIVR3EN_W<'_, PLLCFGRrs> {
        DIVR3EN_W::new(self, 24)
    }
}
/**RCC PLLs Configuration Register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#RCC:PLLCFGR)*/
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
///`reset()` method sets PLLCFGR to value 0x01ff_0000
impl crate::Resettable for PLLCFGRrs {
    const RESET_VALUE: u32 = 0x01ff_0000;
}
