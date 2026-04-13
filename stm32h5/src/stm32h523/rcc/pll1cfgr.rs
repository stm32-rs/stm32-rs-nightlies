///Register `PLL1CFGR` reader
pub type R = crate::R<PLL1CFGRrs>;
///Register `PLL1CFGR` writer
pub type W = crate::W<PLL1CFGRrs>;
/**PLL1M and PLLs clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1SRC {
    ///0: No clock sent to DIVMx dividers and PLLs
    None = 0,
    ///1: HSI selected as PLL clock
    Hsi = 1,
    ///2: CSI selected as PLL clock
    Csi = 2,
    ///3: HSE selected as PLL clock
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
impl crate::IsEnum for PLL1SRC {}
///Field `PLL1SRC` reader - PLL1M and PLLs clock source selection
pub type PLL1SRC_R = crate::FieldReader<PLL1SRC>;
impl PLL1SRC_R {
    ///Get enumerated values variant
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
    ///No clock sent to DIVMx dividers and PLLs
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PLL1SRC::None
    }
    ///HSI selected as PLL clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == PLL1SRC::Hsi
    }
    ///CSI selected as PLL clock
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == PLL1SRC::Csi
    }
    ///HSE selected as PLL clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLL1SRC::Hse
    }
}
///Field `PLL1SRC` writer - PLL1M and PLLs clock source selection
pub type PLL1SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL1SRC, crate::Safe>;
impl<'a, REG> PLL1SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock sent to DIVMx dividers and PLLs
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC::None)
    }
    ///HSI selected as PLL clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC::Hsi)
    }
    ///CSI selected as PLL clock
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC::Csi)
    }
    ///HSE selected as PLL clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC::Hse)
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
/**PLL1 fractional latch enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1FRACEN {
    ///0: Reset latch to transfer FRACN to the Sigma-Delta modulator
    Reset = 0,
    ///1: Set latch to transfer FRACN to the Sigma-Delta modulator
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
    ///Reset latch to transfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PLL1FRACEN::Reset
    }
    ///Set latch to transfer FRACN to the Sigma-Delta modulator
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
    ///Reset latch to transfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1FRACEN::Reset)
    }
    ///Set latch to transfer FRACN to the Sigma-Delta modulator
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
///Field `PLL1M` reader - prescaler for PLL1
pub type PLL1M_R = crate::FieldReader;
///Field `PLL1M` writer - prescaler for PLL1
pub type PLL1M_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
/**PLL1 DIVP divider output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1PEN {
    ///0: Clock output is disabled
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
    ///Clock output is disabled
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
    ///Clock output is disabled
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
///Field `PLL1REN` reader - PLL1 DIVR divider output enable
pub use PLL1PEN_R as PLL1REN_R;
///Field `PLL1QEN` writer - PLL1 DIVQ divider output enable
pub use PLL1PEN_W as PLL1QEN_W;
///Field `PLL1REN` writer - PLL1 DIVR divider output enable
pub use PLL1PEN_W as PLL1REN_W;
impl R {
    ///Bits 0:1 - PLL1M and PLLs clock source selection
    #[inline(always)]
    pub fn pll1src(&self) -> PLL1SRC_R {
        PLL1SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PLL1 input frequency range
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PLL1 fractional latch enable
    #[inline(always)]
    pub fn pll1fracen(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PLL1 VCO selection
    #[inline(always)]
    pub fn pll1vcosel(&self) -> PLL1VCOSEL_R {
        PLL1VCOSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:13 - prescaler for PLL1
    #[inline(always)]
    pub fn pll1m(&self) -> PLL1M_R {
        PLL1M_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 16 - PLL1 DIVP divider output enable
    #[inline(always)]
    pub fn pll1pen(&self) -> PLL1PEN_R {
        PLL1PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL1 DIVQ divider output enable
    #[inline(always)]
    pub fn pll1qen(&self) -> PLL1QEN_R {
        PLL1QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL1 DIVR divider output enable
    #[inline(always)]
    pub fn pll1ren(&self) -> PLL1REN_R {
        PLL1REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1CFGR")
            .field("pll1src", &self.pll1src())
            .field("pll1rge", &self.pll1rge())
            .field("pll1fracen", &self.pll1fracen())
            .field("pll1vcosel", &self.pll1vcosel())
            .field("pll1m", &self.pll1m())
            .field("pll1pen", &self.pll1pen())
            .field("pll1qen", &self.pll1qen())
            .field("pll1ren", &self.pll1ren())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL1M and PLLs clock source selection
    #[inline(always)]
    pub fn pll1src(&mut self) -> PLL1SRC_W<'_, PLL1CFGRrs> {
        PLL1SRC_W::new(self, 0)
    }
    ///Bits 2:3 - PLL1 input frequency range
    #[inline(always)]
    pub fn pll1rge(&mut self) -> PLL1RGE_W<'_, PLL1CFGRrs> {
        PLL1RGE_W::new(self, 2)
    }
    ///Bit 4 - PLL1 fractional latch enable
    #[inline(always)]
    pub fn pll1fracen(&mut self) -> PLL1FRACEN_W<'_, PLL1CFGRrs> {
        PLL1FRACEN_W::new(self, 4)
    }
    ///Bit 5 - PLL1 VCO selection
    #[inline(always)]
    pub fn pll1vcosel(&mut self) -> PLL1VCOSEL_W<'_, PLL1CFGRrs> {
        PLL1VCOSEL_W::new(self, 5)
    }
    ///Bits 8:13 - prescaler for PLL1
    #[inline(always)]
    pub fn pll1m(&mut self) -> PLL1M_W<'_, PLL1CFGRrs> {
        PLL1M_W::new(self, 8)
    }
    ///Bit 16 - PLL1 DIVP divider output enable
    #[inline(always)]
    pub fn pll1pen(&mut self) -> PLL1PEN_W<'_, PLL1CFGRrs> {
        PLL1PEN_W::new(self, 16)
    }
    ///Bit 17 - PLL1 DIVQ divider output enable
    #[inline(always)]
    pub fn pll1qen(&mut self) -> PLL1QEN_W<'_, PLL1CFGRrs> {
        PLL1QEN_W::new(self, 17)
    }
    ///Bit 18 - PLL1 DIVR divider output enable
    #[inline(always)]
    pub fn pll1ren(&mut self) -> PLL1REN_W<'_, PLL1CFGRrs> {
        PLL1REN_W::new(self, 18)
    }
}
/**RCC PLL clock source selection register

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#RCC:PLL1CFGR)*/
pub struct PLL1CFGRrs;
impl crate::RegisterSpec for PLL1CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pll1cfgr::R`](R) reader structure
impl crate::Readable for PLL1CFGRrs {}
///`write(|w| ..)` method takes [`pll1cfgr::W`](W) writer structure
impl crate::Writable for PLL1CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1CFGR to value 0
impl crate::Resettable for PLL1CFGRrs {}
