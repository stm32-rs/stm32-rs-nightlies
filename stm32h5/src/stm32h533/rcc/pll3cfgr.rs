///Register `PLL3CFGR` reader
pub type R = crate::R<PLL3CFGRrs>;
///Register `PLL3CFGR` writer
pub type W = crate::W<PLL3CFGRrs>;
/**PLL3M and PLLs clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL3SRC {
    ///0: No clock sent to DIVMx dividers and PLLs
    None = 0,
    ///1: HSI selected as PLL clock
    Hsi = 1,
    ///2: CSI selected as PLL clock
    Csi = 2,
    ///3: HSE selected as PLL clock
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
impl crate::IsEnum for PLL3SRC {}
///Field `PLL3SRC` reader - PLL3M and PLLs clock source selection
pub type PLL3SRC_R = crate::FieldReader<PLL3SRC>;
impl PLL3SRC_R {
    ///Get enumerated values variant
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
    ///No clock sent to DIVMx dividers and PLLs
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PLL3SRC::None
    }
    ///HSI selected as PLL clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == PLL3SRC::Hsi
    }
    ///CSI selected as PLL clock
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == PLL3SRC::Csi
    }
    ///HSE selected as PLL clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLL3SRC::Hse
    }
}
///Field `PLL3SRC` writer - PLL3M and PLLs clock source selection
pub type PLL3SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL3SRC, crate::Safe>;
impl<'a, REG> PLL3SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock sent to DIVMx dividers and PLLs
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3SRC::None)
    }
    ///HSI selected as PLL clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3SRC::Hsi)
    }
    ///CSI selected as PLL clock
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3SRC::Csi)
    }
    ///HSE selected as PLL clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3SRC::Hse)
    }
}
/**PLL3 input frequency range

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL3RGE {
    ///0: Frequency is between 1 and 2 MHz
    Range1 = 0,
    ///1: Frequency is between 2 and 4 MHz
    Range2 = 1,
    ///2: Frequency is between 4 and 8 MHz
    Range4 = 2,
    ///3: Frequency is between 8 and 16 MHz
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
impl crate::IsEnum for PLL3RGE {}
///Field `PLL3RGE` reader - PLL3 input frequency range
pub type PLL3RGE_R = crate::FieldReader<PLL3RGE>;
impl PLL3RGE_R {
    ///Get enumerated values variant
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
    ///Frequency is between 1 and 2 MHz
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == PLL3RGE::Range1
    }
    ///Frequency is between 2 and 4 MHz
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == PLL3RGE::Range2
    }
    ///Frequency is between 4 and 8 MHz
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == PLL3RGE::Range4
    }
    ///Frequency is between 8 and 16 MHz
    #[inline(always)]
    pub fn is_range8(&self) -> bool {
        *self == PLL3RGE::Range8
    }
}
///Field `PLL3RGE` writer - PLL3 input frequency range
pub type PLL3RGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL3RGE, crate::Safe>;
impl<'a, REG> PLL3RGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Frequency is between 1 and 2 MHz
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3RGE::Range1)
    }
    ///Frequency is between 2 and 4 MHz
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3RGE::Range2)
    }
    ///Frequency is between 4 and 8 MHz
    #[inline(always)]
    pub fn range4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3RGE::Range4)
    }
    ///Frequency is between 8 and 16 MHz
    #[inline(always)]
    pub fn range8(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3RGE::Range8)
    }
}
/**PLL3 fractional latch enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL3FRACEN {
    ///0: Reset latch to transfer FRACN to the Sigma-Delta modulator
    Reset = 0,
    ///1: Set latch to transfer FRACN to the Sigma-Delta modulator
    Set = 1,
}
impl From<PLL3FRACEN> for bool {
    #[inline(always)]
    fn from(variant: PLL3FRACEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL3FRACEN` reader - PLL3 fractional latch enable
pub type PLL3FRACEN_R = crate::BitReader<PLL3FRACEN>;
impl PLL3FRACEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL3FRACEN {
        match self.bits {
            false => PLL3FRACEN::Reset,
            true => PLL3FRACEN::Set,
        }
    }
    ///Reset latch to transfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PLL3FRACEN::Reset
    }
    ///Set latch to transfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == PLL3FRACEN::Set
    }
}
///Field `PLL3FRACEN` writer - PLL3 fractional latch enable
pub type PLL3FRACEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL3FRACEN>;
impl<'a, REG> PLL3FRACEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset latch to transfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3FRACEN::Reset)
    }
    ///Set latch to transfer FRACN to the Sigma-Delta modulator
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3FRACEN::Set)
    }
}
/**PLL3 VCO selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL3VCOSEL {
    ///0: VCO frequency range 192 to 836 MHz
    WideVco = 0,
    ///1: VCO frequency range 150 to 420 MHz
    MediumVco = 1,
}
impl From<PLL3VCOSEL> for bool {
    #[inline(always)]
    fn from(variant: PLL3VCOSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL3VCOSEL` reader - PLL3 VCO selection
pub type PLL3VCOSEL_R = crate::BitReader<PLL3VCOSEL>;
impl PLL3VCOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL3VCOSEL {
        match self.bits {
            false => PLL3VCOSEL::WideVco,
            true => PLL3VCOSEL::MediumVco,
        }
    }
    ///VCO frequency range 192 to 836 MHz
    #[inline(always)]
    pub fn is_wide_vco(&self) -> bool {
        *self == PLL3VCOSEL::WideVco
    }
    ///VCO frequency range 150 to 420 MHz
    #[inline(always)]
    pub fn is_medium_vco(&self) -> bool {
        *self == PLL3VCOSEL::MediumVco
    }
}
///Field `PLL3VCOSEL` writer - PLL3 VCO selection
pub type PLL3VCOSEL_W<'a, REG> = crate::BitWriter<'a, REG, PLL3VCOSEL>;
impl<'a, REG> PLL3VCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VCO frequency range 192 to 836 MHz
    #[inline(always)]
    pub fn wide_vco(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3VCOSEL::WideVco)
    }
    ///VCO frequency range 150 to 420 MHz
    #[inline(always)]
    pub fn medium_vco(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3VCOSEL::MediumVco)
    }
}
///Field `PLL3M` reader - prescaler for PLL3
pub type PLL3M_R = crate::FieldReader;
///Field `PLL3M` writer - prescaler for PLL3
pub type PLL3M_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
/**PLL3 DIVP divider output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL3PEN {
    ///0: Clock output is disabled
    Disabled = 0,
    ///1: Clock output is enabled
    Enabled = 1,
}
impl From<PLL3PEN> for bool {
    #[inline(always)]
    fn from(variant: PLL3PEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL3PEN` reader - PLL3 DIVP divider output enable
pub type PLL3PEN_R = crate::BitReader<PLL3PEN>;
impl PLL3PEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLL3PEN {
        match self.bits {
            false => PLL3PEN::Disabled,
            true => PLL3PEN::Enabled,
        }
    }
    ///Clock output is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLL3PEN::Disabled
    }
    ///Clock output is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLL3PEN::Enabled
    }
}
///Field `PLL3PEN` writer - PLL3 DIVP divider output enable
pub type PLL3PEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL3PEN>;
impl<'a, REG> PLL3PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock output is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3PEN::Disabled)
    }
    ///Clock output is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLL3PEN::Enabled)
    }
}
///Field `PLL3QEN` reader - PLL3 DIVQ divider output enable
pub use PLL3PEN_R as PLL3QEN_R;
///Field `PLL3REN` reader - PLL3 DIVR divider output enable
pub use PLL3PEN_R as PLL3REN_R;
///Field `PLL3QEN` writer - PLL3 DIVQ divider output enable
pub use PLL3PEN_W as PLL3QEN_W;
///Field `PLL3REN` writer - PLL3 DIVR divider output enable
pub use PLL3PEN_W as PLL3REN_W;
impl R {
    ///Bits 0:1 - PLL3M and PLLs clock source selection
    #[inline(always)]
    pub fn pll3src(&self) -> PLL3SRC_R {
        PLL3SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PLL3 input frequency range
    #[inline(always)]
    pub fn pll3rge(&self) -> PLL3RGE_R {
        PLL3RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PLL3 fractional latch enable
    #[inline(always)]
    pub fn pll3fracen(&self) -> PLL3FRACEN_R {
        PLL3FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PLL3 VCO selection
    #[inline(always)]
    pub fn pll3vcosel(&self) -> PLL3VCOSEL_R {
        PLL3VCOSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:13 - prescaler for PLL3
    #[inline(always)]
    pub fn pll3m(&self) -> PLL3M_R {
        PLL3M_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 16 - PLL3 DIVP divider output enable
    #[inline(always)]
    pub fn pll3pen(&self) -> PLL3PEN_R {
        PLL3PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL3 DIVQ divider output enable
    #[inline(always)]
    pub fn pll3qen(&self) -> PLL3QEN_R {
        PLL3QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL3 DIVR divider output enable
    #[inline(always)]
    pub fn pll3ren(&self) -> PLL3REN_R {
        PLL3REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL3CFGR")
            .field("pll3src", &self.pll3src())
            .field("pll3rge", &self.pll3rge())
            .field("pll3fracen", &self.pll3fracen())
            .field("pll3vcosel", &self.pll3vcosel())
            .field("pll3m", &self.pll3m())
            .field("pll3pen", &self.pll3pen())
            .field("pll3qen", &self.pll3qen())
            .field("pll3ren", &self.pll3ren())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL3M and PLLs clock source selection
    #[inline(always)]
    pub fn pll3src(&mut self) -> PLL3SRC_W<PLL3CFGRrs> {
        PLL3SRC_W::new(self, 0)
    }
    ///Bits 2:3 - PLL3 input frequency range
    #[inline(always)]
    pub fn pll3rge(&mut self) -> PLL3RGE_W<PLL3CFGRrs> {
        PLL3RGE_W::new(self, 2)
    }
    ///Bit 4 - PLL3 fractional latch enable
    #[inline(always)]
    pub fn pll3fracen(&mut self) -> PLL3FRACEN_W<PLL3CFGRrs> {
        PLL3FRACEN_W::new(self, 4)
    }
    ///Bit 5 - PLL3 VCO selection
    #[inline(always)]
    pub fn pll3vcosel(&mut self) -> PLL3VCOSEL_W<PLL3CFGRrs> {
        PLL3VCOSEL_W::new(self, 5)
    }
    ///Bits 8:13 - prescaler for PLL3
    #[inline(always)]
    pub fn pll3m(&mut self) -> PLL3M_W<PLL3CFGRrs> {
        PLL3M_W::new(self, 8)
    }
    ///Bit 16 - PLL3 DIVP divider output enable
    #[inline(always)]
    pub fn pll3pen(&mut self) -> PLL3PEN_W<PLL3CFGRrs> {
        PLL3PEN_W::new(self, 16)
    }
    ///Bit 17 - PLL3 DIVQ divider output enable
    #[inline(always)]
    pub fn pll3qen(&mut self) -> PLL3QEN_W<PLL3CFGRrs> {
        PLL3QEN_W::new(self, 17)
    }
    ///Bit 18 - PLL3 DIVR divider output enable
    #[inline(always)]
    pub fn pll3ren(&mut self) -> PLL3REN_W<PLL3CFGRrs> {
        PLL3REN_W::new(self, 18)
    }
}
/**RCC PLL clock source selection register

You can [`read`](crate::Reg::read) this register and get [`pll3cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RCC:PLL3CFGR)*/
pub struct PLL3CFGRrs;
impl crate::RegisterSpec for PLL3CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pll3cfgr::R`](R) reader structure
impl crate::Readable for PLL3CFGRrs {}
///`write(|w| ..)` method takes [`pll3cfgr::W`](W) writer structure
impl crate::Writable for PLL3CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL3CFGR to value 0
impl crate::Resettable for PLL3CFGRrs {}
