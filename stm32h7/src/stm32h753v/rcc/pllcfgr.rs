#[doc = "Register `PLLCFGR` reader"]
pub type R = crate::R<PLLCFGRrs>;
#[doc = "Register `PLLCFGR` writer"]
pub type W = crate::W<PLLCFGRrs>;
#[doc = "PLL1 fractional latch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1FRACEN {
    #[doc = "0: Reset latch to tranfer FRACN to the Sigma-Delta modulator"]
    Reset = 0,
    #[doc = "1: Set latch to tranfer FRACN to the Sigma-Delta modulator"]
    Set = 1,
}
impl From<PLL1FRACEN> for bool {
    #[inline(always)]
    fn from(variant: PLL1FRACEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1FRACEN` reader - PLL1 fractional latch enable"]
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
    #[doc = "Reset latch to tranfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PLL1FRACEN::Reset
    }
    #[doc = "Set latch to tranfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == PLL1FRACEN::Set
    }
}
#[doc = "Field `PLL1FRACEN` writer - PLL1 fractional latch enable"]
pub type PLL1FRACEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL1FRACEN>;
impl<'a, REG> PLL1FRACEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset latch to tranfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1FRACEN::Reset)
    }
    #[doc = "Set latch to tranfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1FRACEN::Set)
    }
}
#[doc = "PLL1 VCO selection\n\nValue on reset: 0"]
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
#[doc = "Field `PLL1VCOSEL` reader - PLL1 VCO selection"]
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
#[doc = "Field `PLL1VCOSEL` writer - PLL1 VCO selection"]
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
#[doc = "PLL1 input frequency range\n\nValue on reset: 0"]
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
#[doc = "Field `PLL1RGE` reader - PLL1 input frequency range"]
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
#[doc = "Field `PLL1RGE` writer - PLL1 input frequency range"]
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
#[doc = "Field `PLL2FRACEN` reader - PLL2 fractional latch enable"]
pub use PLL1FRACEN_R as PLL2FRACEN_R;
#[doc = "Field `PLL3FRACEN` reader - PLL3 fractional latch enable"]
pub use PLL1FRACEN_R as PLL3FRACEN_R;
#[doc = "Field `PLL2FRACEN` writer - PLL2 fractional latch enable"]
pub use PLL1FRACEN_W as PLL2FRACEN_W;
#[doc = "Field `PLL3FRACEN` writer - PLL3 fractional latch enable"]
pub use PLL1FRACEN_W as PLL3FRACEN_W;
#[doc = "Field `PLL2RGE` reader - PLL2 input frequency range"]
pub use PLL1RGE_R as PLL2RGE_R;
#[doc = "Field `PLL3RGE` reader - PLL3 input frequency range"]
pub use PLL1RGE_R as PLL3RGE_R;
#[doc = "Field `PLL2RGE` writer - PLL2 input frequency range"]
pub use PLL1RGE_W as PLL2RGE_W;
#[doc = "Field `PLL3RGE` writer - PLL3 input frequency range"]
pub use PLL1RGE_W as PLL3RGE_W;
#[doc = "Field `PLL2VCOSEL` reader - PLL2 VCO selection"]
pub use PLL1VCOSEL_R as PLL2VCOSEL_R;
#[doc = "Field `PLL3VCOSEL` reader - PLL3 VCO selection"]
pub use PLL1VCOSEL_R as PLL3VCOSEL_R;
#[doc = "Field `PLL2VCOSEL` writer - PLL2 VCO selection"]
pub use PLL1VCOSEL_W as PLL2VCOSEL_W;
#[doc = "Field `PLL3VCOSEL` writer - PLL3 VCO selection"]
pub use PLL1VCOSEL_W as PLL3VCOSEL_W;
#[doc = "PLL1 DIVP divider output enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIVP1EN {
    #[doc = "0: Clock ouput is disabled"]
    Disabled = 0,
    #[doc = "1: Clock output is enabled"]
    Enabled = 1,
}
impl From<DIVP1EN> for bool {
    #[inline(always)]
    fn from(variant: DIVP1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVP1EN` reader - PLL1 DIVP divider output enable"]
pub type DIVP1EN_R = crate::BitReader<DIVP1EN>;
impl DIVP1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIVP1EN {
        match self.bits {
            false => DIVP1EN::Disabled,
            true => DIVP1EN::Enabled,
        }
    }
    #[doc = "Clock ouput is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIVP1EN::Disabled
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIVP1EN::Enabled
    }
}
#[doc = "Field `DIVP1EN` writer - PLL1 DIVP divider output enable"]
pub type DIVP1EN_W<'a, REG> = crate::BitWriter<'a, REG, DIVP1EN>;
impl<'a, REG> DIVP1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock ouput is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1EN::Disabled)
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1EN::Enabled)
    }
}
#[doc = "Field `DIVQ1EN` reader - PLL1 DIVQ divider output enable"]
pub use DIVP1EN_R as DIVQ1EN_R;
#[doc = "Field `DIVR1EN` reader - PLL1 DIVR divider output enable"]
pub use DIVP1EN_R as DIVR1EN_R;
#[doc = "Field `DIVP2EN` reader - PLL2 DIVP divider output enable"]
pub use DIVP1EN_R as DIVP2EN_R;
#[doc = "Field `DIVQ2EN` reader - PLL2 DIVQ divider output enable"]
pub use DIVP1EN_R as DIVQ2EN_R;
#[doc = "Field `DIVR2EN` reader - PLL2 DIVR divider output enable"]
pub use DIVP1EN_R as DIVR2EN_R;
#[doc = "Field `DIVP3EN` reader - PLL3 DIVP divider output enable"]
pub use DIVP1EN_R as DIVP3EN_R;
#[doc = "Field `DIVQ3EN` reader - PLL3 DIVQ divider output enable"]
pub use DIVP1EN_R as DIVQ3EN_R;
#[doc = "Field `DIVR3EN` reader - PLL3 DIVR divider output enable"]
pub use DIVP1EN_R as DIVR3EN_R;
#[doc = "Field `DIVQ1EN` writer - PLL1 DIVQ divider output enable"]
pub use DIVP1EN_W as DIVQ1EN_W;
#[doc = "Field `DIVR1EN` writer - PLL1 DIVR divider output enable"]
pub use DIVP1EN_W as DIVR1EN_W;
#[doc = "Field `DIVP2EN` writer - PLL2 DIVP divider output enable"]
pub use DIVP1EN_W as DIVP2EN_W;
#[doc = "Field `DIVQ2EN` writer - PLL2 DIVQ divider output enable"]
pub use DIVP1EN_W as DIVQ2EN_W;
#[doc = "Field `DIVR2EN` writer - PLL2 DIVR divider output enable"]
pub use DIVP1EN_W as DIVR2EN_W;
#[doc = "Field `DIVP3EN` writer - PLL3 DIVP divider output enable"]
pub use DIVP1EN_W as DIVP3EN_W;
#[doc = "Field `DIVQ3EN` writer - PLL3 DIVQ divider output enable"]
pub use DIVP1EN_W as DIVQ3EN_W;
#[doc = "Field `DIVR3EN` writer - PLL3 DIVR divider output enable"]
pub use DIVP1EN_W as DIVR3EN_W;
impl R {
    #[doc = "Bit 0 - PLL1 fractional latch enable"]
    #[inline(always)]
    pub fn pll1fracen(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL1 VCO selection"]
    #[inline(always)]
    pub fn pll1vcosel(&self) -> PLL1VCOSEL_R {
        PLL1VCOSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range"]
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable"]
    #[inline(always)]
    pub fn pll2fracen(&self) -> PLL2FRACEN_R {
        PLL2FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL2 VCO selection"]
    #[inline(always)]
    pub fn pll2vcosel(&self) -> PLL2VCOSEL_R {
        PLL2VCOSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - PLL2 input frequency range"]
    #[inline(always)]
    pub fn pll2rge(&self) -> PLL2RGE_R {
        PLL2RGE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - PLL3 fractional latch enable"]
    #[inline(always)]
    pub fn pll3fracen(&self) -> PLL3FRACEN_R {
        PLL3FRACEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL3 VCO selection"]
    #[inline(always)]
    pub fn pll3vcosel(&self) -> PLL3VCOSEL_R {
        PLL3VCOSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - PLL3 input frequency range"]
    #[inline(always)]
    pub fn pll3rge(&self) -> PLL3RGE_R {
        PLL3RGE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp1en(&self) -> DIVP1EN_R {
        DIVP1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq1en(&self) -> DIVQ1EN_R {
        DIVQ1EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr1en(&self) -> DIVR1EN_R {
        DIVR1EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PLL2 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp2en(&self) -> DIVP2EN_R {
        DIVP2EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PLL2 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq2en(&self) -> DIVQ2EN_R {
        DIVQ2EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PLL2 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr2en(&self) -> DIVR2EN_R {
        DIVR2EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PLL3 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp3en(&self) -> DIVP3EN_R {
        DIVP3EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PLL3 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq3en(&self) -> DIVQ3EN_R {
        DIVQ3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL3 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr3en(&self) -> DIVR3EN_R {
        DIVR3EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL1 fractional latch enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll1fracen(&mut self) -> PLL1FRACEN_W<PLLCFGRrs> {
        PLL1FRACEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - PLL1 VCO selection"]
    #[inline(always)]
    #[must_use]
    pub fn pll1vcosel(&mut self) -> PLL1VCOSEL_W<PLLCFGRrs> {
        PLL1VCOSEL_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range"]
    #[inline(always)]
    #[must_use]
    pub fn pll1rge(&mut self) -> PLL1RGE_W<PLLCFGRrs> {
        PLL1RGE_W::new(self, 2)
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll2fracen(&mut self) -> PLL2FRACEN_W<PLLCFGRrs> {
        PLL2FRACEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - PLL2 VCO selection"]
    #[inline(always)]
    #[must_use]
    pub fn pll2vcosel(&mut self) -> PLL2VCOSEL_W<PLLCFGRrs> {
        PLL2VCOSEL_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - PLL2 input frequency range"]
    #[inline(always)]
    #[must_use]
    pub fn pll2rge(&mut self) -> PLL2RGE_W<PLLCFGRrs> {
        PLL2RGE_W::new(self, 6)
    }
    #[doc = "Bit 8 - PLL3 fractional latch enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll3fracen(&mut self) -> PLL3FRACEN_W<PLLCFGRrs> {
        PLL3FRACEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - PLL3 VCO selection"]
    #[inline(always)]
    #[must_use]
    pub fn pll3vcosel(&mut self) -> PLL3VCOSEL_W<PLLCFGRrs> {
        PLL3VCOSEL_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - PLL3 input frequency range"]
    #[inline(always)]
    #[must_use]
    pub fn pll3rge(&mut self) -> PLL3RGE_W<PLLCFGRrs> {
        PLL3RGE_W::new(self, 10)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divp1en(&mut self) -> DIVP1EN_W<PLLCFGRrs> {
        DIVP1EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divq1en(&mut self) -> DIVQ1EN_W<PLLCFGRrs> {
        DIVQ1EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divr1en(&mut self) -> DIVR1EN_W<PLLCFGRrs> {
        DIVR1EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - PLL2 DIVP divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divp2en(&mut self) -> DIVP2EN_W<PLLCFGRrs> {
        DIVP2EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - PLL2 DIVQ divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divq2en(&mut self) -> DIVQ2EN_W<PLLCFGRrs> {
        DIVQ2EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - PLL2 DIVR divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divr2en(&mut self) -> DIVR2EN_W<PLLCFGRrs> {
        DIVR2EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - PLL3 DIVP divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divp3en(&mut self) -> DIVP3EN_W<PLLCFGRrs> {
        DIVP3EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - PLL3 DIVQ divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divq3en(&mut self) -> DIVQ3EN_W<PLLCFGRrs> {
        DIVQ3EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - PLL3 DIVR divider output enable"]
    #[inline(always)]
    #[must_use]
    pub fn divr3en(&mut self) -> DIVR3EN_W<PLLCFGRrs> {
        DIVR3EN_W::new(self, 24)
    }
}
#[doc = "RCC PLLs Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCFGRrs;
impl crate::RegisterSpec for PLLCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfgr::R`](R) reader structure"]
impl crate::Readable for PLLCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure"]
impl crate::Writable for PLLCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x01ff_0000"]
impl crate::Resettable for PLLCFGRrs {
    const RESET_VALUE: u32 = 0x01ff_0000;
}
