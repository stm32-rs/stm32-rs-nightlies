///Register `PLL2CFGR` reader
pub type R = crate::R<PLL2CFGRrs>;
///Register `PLL2CFGR` writer
pub type W = crate::W<PLL2CFGRrs>;
/**PLL2M and PLLs clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2SRC {
    ///0: No clock sent to DIVMx dividers and PLLs
    None = 0,
    ///1: HSI selected as PLL clock
    Hsi = 1,
    ///2: CSI selected as PLL clock
    Csi = 2,
    ///3: HSE selected as PLL clock
    Hse = 3,
}
impl From<PLL2SRC> for u8 {
    #[inline(always)]
    fn from(variant: PLL2SRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2SRC {
    type Ux = u8;
}
impl crate::IsEnum for PLL2SRC {}
///Field `PLL2SRC` reader - PLL2M and PLLs clock source selection
pub type PLL2SRC_R = crate::FieldReader<PLL2SRC>;
impl PLL2SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2SRC {
        match self.bits {
            0 => PLL2SRC::None,
            1 => PLL2SRC::Hsi,
            2 => PLL2SRC::Csi,
            3 => PLL2SRC::Hse,
            _ => unreachable!(),
        }
    }
    ///No clock sent to DIVMx dividers and PLLs
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PLL2SRC::None
    }
    ///HSI selected as PLL clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == PLL2SRC::Hsi
    }
    ///CSI selected as PLL clock
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == PLL2SRC::Csi
    }
    ///HSE selected as PLL clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLL2SRC::Hse
    }
}
///Field `PLL2SRC` writer - PLL2M and PLLs clock source selection
pub type PLL2SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL2SRC, crate::Safe>;
impl<'a, REG> PLL2SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock sent to DIVMx dividers and PLLs
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2SRC::None)
    }
    ///HSI selected as PLL clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2SRC::Hsi)
    }
    ///CSI selected as PLL clock
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2SRC::Csi)
    }
    ///HSE selected as PLL clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2SRC::Hse)
    }
}
/**PLL2 input frequency range

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2RGE {
    ///0: Frequency is between 1 and 2 MHz
    Range1 = 0,
    ///1: Frequency is between 2 and 4 MHz
    Range2 = 1,
    ///2: Frequency is between 4 and 8 MHz
    Range4 = 2,
    ///3: Frequency is between 8 and 16 MHz
    Range8 = 3,
}
impl From<PLL2RGE> for u8 {
    #[inline(always)]
    fn from(variant: PLL2RGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2RGE {
    type Ux = u8;
}
impl crate::IsEnum for PLL2RGE {}
///Field `PLL2RGE` reader - PLL2 input frequency range
pub type PLL2RGE_R = crate::FieldReader<PLL2RGE>;
impl PLL2RGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2RGE {
        match self.bits {
            0 => PLL2RGE::Range1,
            1 => PLL2RGE::Range2,
            2 => PLL2RGE::Range4,
            3 => PLL2RGE::Range8,
            _ => unreachable!(),
        }
    }
    ///Frequency is between 1 and 2 MHz
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == PLL2RGE::Range1
    }
    ///Frequency is between 2 and 4 MHz
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == PLL2RGE::Range2
    }
    ///Frequency is between 4 and 8 MHz
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == PLL2RGE::Range4
    }
    ///Frequency is between 8 and 16 MHz
    #[inline(always)]
    pub fn is_range8(&self) -> bool {
        *self == PLL2RGE::Range8
    }
}
///Field `PLL2RGE` writer - PLL2 input frequency range
pub type PLL2RGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL2RGE, crate::Safe>;
impl<'a, REG> PLL2RGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Frequency is between 1 and 2 MHz
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RGE::Range1)
    }
    ///Frequency is between 2 and 4 MHz
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RGE::Range2)
    }
    ///Frequency is between 4 and 8 MHz
    #[inline(always)]
    pub fn range4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RGE::Range4)
    }
    ///Frequency is between 8 and 16 MHz
    #[inline(always)]
    pub fn range8(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RGE::Range8)
    }
}
/**PLL2 fractional latch enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2FRACEN {
    ///0: Reset latch to transfer FRACN to the Sigma-Delta modulator
    Reset = 0,
    ///1: Set latch to transfer FRACN to the Sigma-Delta modulator
    Set = 1,
}
impl From<PLL2FRACEN> for bool {
    #[inline(always)]
    fn from(variant: PLL2FRACEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL2FRACEN` reader - PLL2 fractional latch enable
pub type PLL2FRACEN_R = crate::BitReader<PLL2FRACEN>;
impl PLL2FRACEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2FRACEN {
        match self.bits {
            false => PLL2FRACEN::Reset,
            true => PLL2FRACEN::Set,
        }
    }
    ///Reset latch to transfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PLL2FRACEN::Reset
    }
    ///Set latch to transfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == PLL2FRACEN::Set
    }
}
///Field `PLL2FRACEN` writer - PLL2 fractional latch enable
pub type PLL2FRACEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL2FRACEN>;
impl<'a, REG> PLL2FRACEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset latch to transfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2FRACEN::Reset)
    }
    ///Set latch to transfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2FRACEN::Set)
    }
}
/**PLL2 VCO selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2VCOSEL {
    ///0: VCO frequency range 192 to 836 MHz
    WideVco = 0,
    ///1: VCO frequency range 150 to 420 MHz
    MediumVco = 1,
}
impl From<PLL2VCOSEL> for bool {
    #[inline(always)]
    fn from(variant: PLL2VCOSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL2VCOSEL` reader - PLL2 VCO selection
pub type PLL2VCOSEL_R = crate::BitReader<PLL2VCOSEL>;
impl PLL2VCOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2VCOSEL {
        match self.bits {
            false => PLL2VCOSEL::WideVco,
            true => PLL2VCOSEL::MediumVco,
        }
    }
    ///VCO frequency range 192 to 836 MHz
    #[inline(always)]
    pub fn is_wide_vco(&self) -> bool {
        *self == PLL2VCOSEL::WideVco
    }
    ///VCO frequency range 150 to 420 MHz
    #[inline(always)]
    pub fn is_medium_vco(&self) -> bool {
        *self == PLL2VCOSEL::MediumVco
    }
}
///Field `PLL2VCOSEL` writer - PLL2 VCO selection
pub type PLL2VCOSEL_W<'a, REG> = crate::BitWriter<'a, REG, PLL2VCOSEL>;
impl<'a, REG> PLL2VCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VCO frequency range 192 to 836 MHz
    #[inline(always)]
    pub fn wide_vco(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2VCOSEL::WideVco)
    }
    ///VCO frequency range 150 to 420 MHz
    #[inline(always)]
    pub fn medium_vco(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2VCOSEL::MediumVco)
    }
}
///Field `PLL2M` reader - prescaler for PLL2
pub type PLL2M_R = crate::FieldReader;
///Field `PLL2M` writer - prescaler for PLL2
pub type PLL2M_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
/**PLL2 DIVP divider output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2PEN {
    ///0: Clock output is disabled
    Disabled = 0,
    ///1: Clock output is enabled
    Enabled = 1,
}
impl From<PLL2PEN> for bool {
    #[inline(always)]
    fn from(variant: PLL2PEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL2PEN` reader - PLL2 DIVP divider output enable
pub type PLL2PEN_R = crate::BitReader<PLL2PEN>;
impl PLL2PEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL2PEN {
        match self.bits {
            false => PLL2PEN::Disabled,
            true => PLL2PEN::Enabled,
        }
    }
    ///Clock output is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL2PEN::Disabled
    }
    ///Clock output is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL2PEN::Enabled
    }
}
///Field `PLL2PEN` writer - PLL2 DIVP divider output enable
pub type PLL2PEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL2PEN>;
impl<'a, REG> PLL2PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock output is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2PEN::Disabled)
    }
    ///Clock output is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2PEN::Enabled)
    }
}
///Field `PLL2QEN` reader - PLL2 DIVQ divider output enable
pub use PLL2PEN_R as PLL2QEN_R;
///Field `PLL2REN` reader - PLL2 DIVR divider output enable
pub use PLL2PEN_R as PLL2REN_R;
///Field `PLL2QEN` writer - PLL2 DIVQ divider output enable
pub use PLL2PEN_W as PLL2QEN_W;
///Field `PLL2REN` writer - PLL2 DIVR divider output enable
pub use PLL2PEN_W as PLL2REN_W;
impl R {
    ///Bits 0:1 - PLL2M and PLLs clock source selection
    #[inline(always)]
    pub fn pll2src(&self) -> PLL2SRC_R {
        PLL2SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PLL2 input frequency range
    #[inline(always)]
    pub fn pll2rge(&self) -> PLL2RGE_R {
        PLL2RGE_R::new(((self.bits >> 2) & 3) as u8)
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
    ///Bits 8:13 - prescaler for PLL2
    #[inline(always)]
    pub fn pll2m(&self) -> PLL2M_R {
        PLL2M_R::new(((self.bits >> 8) & 0x3f) as u8)
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2CFGR")
            .field("pll2src", &self.pll2src())
            .field("pll2rge", &self.pll2rge())
            .field("pll2fracen", &self.pll2fracen())
            .field("pll2vcosel", &self.pll2vcosel())
            .field("pll2m", &self.pll2m())
            .field("pll2pen", &self.pll2pen())
            .field("pll2qen", &self.pll2qen())
            .field("pll2ren", &self.pll2ren())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL2M and PLLs clock source selection
    #[inline(always)]
    pub fn pll2src(&mut self) -> PLL2SRC_W<'_, PLL2CFGRrs> {
        PLL2SRC_W::new(self, 0)
    }
    ///Bits 2:3 - PLL2 input frequency range
    #[inline(always)]
    pub fn pll2rge(&mut self) -> PLL2RGE_W<'_, PLL2CFGRrs> {
        PLL2RGE_W::new(self, 2)
    }
    ///Bit 4 - PLL2 fractional latch enable
    #[inline(always)]
    pub fn pll2fracen(&mut self) -> PLL2FRACEN_W<'_, PLL2CFGRrs> {
        PLL2FRACEN_W::new(self, 4)
    }
    ///Bit 5 - PLL2 VCO selection
    #[inline(always)]
    pub fn pll2vcosel(&mut self) -> PLL2VCOSEL_W<'_, PLL2CFGRrs> {
        PLL2VCOSEL_W::new(self, 5)
    }
    ///Bits 8:13 - prescaler for PLL2
    #[inline(always)]
    pub fn pll2m(&mut self) -> PLL2M_W<'_, PLL2CFGRrs> {
        PLL2M_W::new(self, 8)
    }
    ///Bit 16 - PLL2 DIVP divider output enable
    #[inline(always)]
    pub fn pll2pen(&mut self) -> PLL2PEN_W<'_, PLL2CFGRrs> {
        PLL2PEN_W::new(self, 16)
    }
    ///Bit 17 - PLL2 DIVQ divider output enable
    #[inline(always)]
    pub fn pll2qen(&mut self) -> PLL2QEN_W<'_, PLL2CFGRrs> {
        PLL2QEN_W::new(self, 17)
    }
    ///Bit 18 - PLL2 DIVR divider output enable
    #[inline(always)]
    pub fn pll2ren(&mut self) -> PLL2REN_W<'_, PLL2CFGRrs> {
        PLL2REN_W::new(self, 18)
    }
}
/**RCC PLL clock source selection register

You can [`read`](crate::Reg::read) this register and get [`pll2cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RCC:PLL2CFGR)*/
pub struct PLL2CFGRrs;
impl crate::RegisterSpec for PLL2CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pll2cfgr::R`](R) reader structure
impl crate::Readable for PLL2CFGRrs {}
///`write(|w| ..)` method takes [`pll2cfgr::W`](W) writer structure
impl crate::Writable for PLL2CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL2CFGR to value 0
impl crate::Resettable for PLL2CFGRrs {}
