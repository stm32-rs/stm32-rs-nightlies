///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Internal high-speed clock enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    ///0: Clock Off
    Off = 0,
    ///1: Clock On
    On = 1,
}
impl From<HSION> for bool {
    #[inline(always)]
    fn from(variant: HSION) -> Self {
        variant as u8 != 0
    }
}
///Field `HSION` reader - Internal high-speed clock enable
pub type HSION_R = crate::BitReader<HSION>;
impl HSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSION {
        match self.bits {
            false => HSION::Off,
            true => HSION::On,
        }
    }
    ///Clock Off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSION::Off
    }
    ///Clock On
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSION::On
    }
}
///Field `HSION` writer - Internal high-speed clock enable
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock Off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Off)
    }
    ///Clock On
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::On)
    }
}
///Field `HSIKERON` reader - High Speed Internal clock enable in Stop mode
pub use HSION_R as HSIKERON_R;
///Field `HSIKERON` writer - High Speed Internal clock enable in Stop mode
pub use HSION_W as HSIKERON_W;
/**HSI clock ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYR {
    ///0: Clock not ready
    NotReady = 0,
    ///1: Clock ready
    Ready = 1,
}
impl From<HSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDY` reader - HSI clock ready flag
pub type HSIRDY_R = crate::BitReader<HSIRDYR>;
impl HSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYR {
        match self.bits {
            false => HSIRDYR::NotReady,
            true => HSIRDYR::Ready,
        }
    }
    ///Clock not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDYR::NotReady
    }
    ///Clock ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDYR::Ready
    }
}
///Field `HSIRDY` writer - HSI clock ready flag
pub type HSIRDY_W<'a, REG> = crate::BitWriter<'a, REG, HSIRDYR>;
impl<'a, REG> HSIRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock not ready
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYR::NotReady)
    }
    ///Clock ready
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYR::Ready)
    }
}
/**HSI clock divider

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSIDIV {
    ///0: No division
    Div1 = 0,
    ///1: Division by 2
    Div2 = 1,
    ///2: Division by 4
    Div4 = 2,
    ///3: Division by 8
    Div8 = 3,
}
impl From<HSIDIV> for u8 {
    #[inline(always)]
    fn from(variant: HSIDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HSIDIV {
    type Ux = u8;
}
impl crate::IsEnum for HSIDIV {}
///Field `HSIDIV` reader - HSI clock divider
pub type HSIDIV_R = crate::FieldReader<HSIDIV>;
impl HSIDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIDIV {
        match self.bits {
            0 => HSIDIV::Div1,
            1 => HSIDIV::Div2,
            2 => HSIDIV::Div4,
            3 => HSIDIV::Div8,
            _ => unreachable!(),
        }
    }
    ///No division
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HSIDIV::Div1
    }
    ///Division by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HSIDIV::Div2
    }
    ///Division by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSIDIV::Div4
    }
    ///Division by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HSIDIV::Div8
    }
}
///Field `HSIDIV` writer - HSI clock divider
pub type HSIDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HSIDIV, crate::Safe>;
impl<'a, REG> HSIDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No division
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div1)
    }
    ///Division by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div2)
    }
    ///Division by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div4)
    }
    ///Division by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div8)
    }
}
/**HSI divider flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIDIVFR {
    ///0: New HSIDIV ratio has not yet propagated to hsi_ck
    NotPropagated = 0,
    ///1: HSIDIV ratio has propagated to hsi_ck
    Propagated = 1,
}
impl From<HSIDIVFR> for bool {
    #[inline(always)]
    fn from(variant: HSIDIVFR) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIDIVF` reader - HSI divider flag
pub type HSIDIVF_R = crate::BitReader<HSIDIVFR>;
impl HSIDIVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIDIVFR {
        match self.bits {
            false => HSIDIVFR::NotPropagated,
            true => HSIDIVFR::Propagated,
        }
    }
    ///New HSIDIV ratio has not yet propagated to hsi_ck
    #[inline(always)]
    pub fn is_not_propagated(&self) -> bool {
        *self == HSIDIVFR::NotPropagated
    }
    ///HSIDIV ratio has propagated to hsi_ck
    #[inline(always)]
    pub fn is_propagated(&self) -> bool {
        *self == HSIDIVFR::Propagated
    }
}
///Field `HSIDIVF` writer - HSI divider flag
pub type HSIDIVF_W<'a, REG> = crate::BitWriter<'a, REG, HSIDIVFR>;
impl<'a, REG> HSIDIVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///New HSIDIV ratio has not yet propagated to hsi_ck
    #[inline(always)]
    pub fn not_propagated(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIVFR::NotPropagated)
    }
    ///HSIDIV ratio has propagated to hsi_ck
    #[inline(always)]
    pub fn propagated(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIVFR::Propagated)
    }
}
///Field `CSION` reader - CSI clock enable
pub use HSION_R as CSION_R;
///Field `CSIKERON` reader - CSI clock enable in Stop mode
pub use HSION_R as CSIKERON_R;
///Field `HSI48ON` reader - RC48 clock enable
pub use HSION_R as HSI48ON_R;
///Field `HSEON` reader - HSE clock enable
pub use HSION_R as HSEON_R;
///Field `CSION` writer - CSI clock enable
pub use HSION_W as CSION_W;
///Field `CSIKERON` writer - CSI clock enable in Stop mode
pub use HSION_W as CSIKERON_W;
///Field `HSI48ON` writer - RC48 clock enable
pub use HSION_W as HSI48ON_W;
///Field `HSEON` writer - HSE clock enable
pub use HSION_W as HSEON_W;
///Field `CSIRDY` reader - CSI clock ready flag
pub use HSIRDY_R as CSIRDY_R;
///Field `HSI48RDY` reader - RC48 clock ready flag
pub use HSIRDY_R as HSI48RDY_R;
///Field `D1CKRDY` reader - D1 domain clocks ready flag
pub use HSIRDY_R as D1CKRDY_R;
///Field `D2CKRDY` reader - D2 domain clocks ready flag
pub use HSIRDY_R as D2CKRDY_R;
///Field `HSERDY` reader - HSE clock ready flag
pub use HSIRDY_R as HSERDY_R;
///Field `CSIRDY` writer - CSI clock ready flag
pub use HSIRDY_W as CSIRDY_W;
///Field `HSI48RDY` writer - RC48 clock ready flag
pub use HSIRDY_W as HSI48RDY_W;
///Field `D1CKRDY` writer - D1 domain clocks ready flag
pub use HSIRDY_W as D1CKRDY_W;
///Field `D2CKRDY` writer - D2 domain clocks ready flag
pub use HSIRDY_W as D2CKRDY_W;
///Field `HSERDY` writer - HSE clock ready flag
pub use HSIRDY_W as HSERDY_W;
/**HSE clock bypass

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP {
    ///0: HSE crystal oscillator not bypassed
    NotBypassed = 0,
    ///1: HSE crystal oscillator bypassed with external clock
    Bypassed = 1,
}
impl From<HSEBYP> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEBYP` reader - HSE clock bypass
pub type HSEBYP_R = crate::BitReader<HSEBYP>;
impl HSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEBYP {
        match self.bits {
            false => HSEBYP::NotBypassed,
            true => HSEBYP::Bypassed,
        }
    }
    ///HSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP::NotBypassed
    }
    ///HSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP::Bypassed
    }
}
///Field `HSEBYP` writer - HSE clock bypass
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, HSEBYP>;
impl<'a, REG> HSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::NotBypassed)
    }
    ///HSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::Bypassed)
    }
}
///Field `HSECSSON` reader - HSE Clock Security System enable
pub use HSION_R as HSECSSON_R;
///Field `PLL1ON` reader - PLL1 enable
pub use HSION_R as PLL1ON_R;
///Field `PLL2ON` reader - PLL2 enable
pub use HSION_R as PLL2ON_R;
///Field `PLL3ON` reader - PLL3 enable
pub use HSION_R as PLL3ON_R;
///Field `HSECSSON` writer - HSE Clock Security System enable
pub use HSION_W as HSECSSON_W;
///Field `PLL1ON` writer - PLL1 enable
pub use HSION_W as PLL1ON_W;
///Field `PLL2ON` writer - PLL2 enable
pub use HSION_W as PLL2ON_W;
///Field `PLL3ON` writer - PLL3 enable
pub use HSION_W as PLL3ON_W;
///Field `PLL1RDY` reader - PLL1 clock ready flag
pub use HSIRDY_R as PLL1RDY_R;
///Field `PLL2RDY` reader - PLL2 clock ready flag
pub use HSIRDY_R as PLL2RDY_R;
///Field `PLL3RDY` reader - PLL3 clock ready flag
pub use HSIRDY_R as PLL3RDY_R;
///Field `PLL1RDY` writer - PLL1 clock ready flag
pub use HSIRDY_W as PLL1RDY_W;
///Field `PLL2RDY` writer - PLL2 clock ready flag
pub use HSIRDY_W as PLL2RDY_W;
///Field `PLL3RDY` writer - PLL3 clock ready flag
pub use HSIRDY_W as PLL3RDY_W;
impl R {
    ///Bit 0 - Internal high-speed clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - High Speed Internal clock enable in Stop mode
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - HSI clock divider
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - HSI divider flag
    #[inline(always)]
    pub fn hsidivf(&self) -> HSIDIVF_R {
        HSIDIVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - CSI clock enable
    #[inline(always)]
    pub fn csion(&self) -> CSION_R {
        CSION_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CSI clock ready flag
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CSI clock enable in Stop mode
    #[inline(always)]
    pub fn csikeron(&self) -> CSIKERON_R {
        CSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - RC48 clock enable
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RC48 clock ready flag
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - D1 domain clocks ready flag
    #[inline(always)]
    pub fn d1ckrdy(&self) -> D1CKRDY_R {
        D1CKRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - D2 domain clocks ready flag
    #[inline(always)]
    pub fn d2ckrdy(&self) -> D2CKRDY_R {
        D2CKRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HSE clock bypass
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - HSE Clock Security System enable
    #[inline(always)]
    pub fn hsecsson(&self) -> HSECSSON_R {
        HSECSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - PLL1 enable
    #[inline(always)]
    pub fn pll1on(&self) -> PLL1ON_R {
        PLL1ON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - PLL1 clock ready flag
    #[inline(always)]
    pub fn pll1rdy(&self) -> PLL1RDY_R {
        PLL1RDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PLL2 enable
    #[inline(always)]
    pub fn pll2on(&self) -> PLL2ON_R {
        PLL2ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - PLL2 clock ready flag
    #[inline(always)]
    pub fn pll2rdy(&self) -> PLL2RDY_R {
        PLL2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PLL3 enable
    #[inline(always)]
    pub fn pll3on(&self) -> PLL3ON_R {
        PLL3ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - PLL3 clock ready flag
    #[inline(always)]
    pub fn pll3rdy(&self) -> PLL3RDY_R {
        PLL3RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("hsion", &self.hsion())
            .field("hsikeron", &self.hsikeron())
            .field("hsirdy", &self.hsirdy())
            .field("hsidiv", &self.hsidiv())
            .field("hsidivf", &self.hsidivf())
            .field("csion", &self.csion())
            .field("csirdy", &self.csirdy())
            .field("csikeron", &self.csikeron())
            .field("hsi48on", &self.hsi48on())
            .field("hsi48rdy", &self.hsi48rdy())
            .field("d1ckrdy", &self.d1ckrdy())
            .field("d2ckrdy", &self.d2ckrdy())
            .field("hseon", &self.hseon())
            .field("hserdy", &self.hserdy())
            .field("hsebyp", &self.hsebyp())
            .field("hsecsson", &self.hsecsson())
            .field("pll1on", &self.pll1on())
            .field("pll1rdy", &self.pll1rdy())
            .field("pll2on", &self.pll2on())
            .field("pll2rdy", &self.pll2rdy())
            .field("pll3on", &self.pll3on())
            .field("pll3rdy", &self.pll3rdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - Internal high-speed clock enable
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 0)
    }
    ///Bit 1 - High Speed Internal clock enable in Stop mode
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W<CRrs> {
        HSIKERON_W::new(self, 1)
    }
    ///Bit 2 - HSI clock ready flag
    #[inline(always)]
    pub fn hsirdy(&mut self) -> HSIRDY_W<CRrs> {
        HSIRDY_W::new(self, 2)
    }
    ///Bits 3:4 - HSI clock divider
    #[inline(always)]
    pub fn hsidiv(&mut self) -> HSIDIV_W<CRrs> {
        HSIDIV_W::new(self, 3)
    }
    ///Bit 5 - HSI divider flag
    #[inline(always)]
    pub fn hsidivf(&mut self) -> HSIDIVF_W<CRrs> {
        HSIDIVF_W::new(self, 5)
    }
    ///Bit 7 - CSI clock enable
    #[inline(always)]
    pub fn csion(&mut self) -> CSION_W<CRrs> {
        CSION_W::new(self, 7)
    }
    ///Bit 8 - CSI clock ready flag
    #[inline(always)]
    pub fn csirdy(&mut self) -> CSIRDY_W<CRrs> {
        CSIRDY_W::new(self, 8)
    }
    ///Bit 9 - CSI clock enable in Stop mode
    #[inline(always)]
    pub fn csikeron(&mut self) -> CSIKERON_W<CRrs> {
        CSIKERON_W::new(self, 9)
    }
    ///Bit 12 - RC48 clock enable
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W<CRrs> {
        HSI48ON_W::new(self, 12)
    }
    ///Bit 13 - RC48 clock ready flag
    #[inline(always)]
    pub fn hsi48rdy(&mut self) -> HSI48RDY_W<CRrs> {
        HSI48RDY_W::new(self, 13)
    }
    ///Bit 14 - D1 domain clocks ready flag
    #[inline(always)]
    pub fn d1ckrdy(&mut self) -> D1CKRDY_W<CRrs> {
        D1CKRDY_W::new(self, 14)
    }
    ///Bit 15 - D2 domain clocks ready flag
    #[inline(always)]
    pub fn d2ckrdy(&mut self) -> D2CKRDY_W<CRrs> {
        D2CKRDY_W::new(self, 15)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&mut self) -> HSERDY_W<CRrs> {
        HSERDY_W::new(self, 17)
    }
    ///Bit 18 - HSE clock bypass
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CRrs> {
        HSEBYP_W::new(self, 18)
    }
    ///Bit 19 - HSE Clock Security System enable
    #[inline(always)]
    pub fn hsecsson(&mut self) -> HSECSSON_W<CRrs> {
        HSECSSON_W::new(self, 19)
    }
    ///Bit 24 - PLL1 enable
    #[inline(always)]
    pub fn pll1on(&mut self) -> PLL1ON_W<CRrs> {
        PLL1ON_W::new(self, 24)
    }
    ///Bit 25 - PLL1 clock ready flag
    #[inline(always)]
    pub fn pll1rdy(&mut self) -> PLL1RDY_W<CRrs> {
        PLL1RDY_W::new(self, 25)
    }
    ///Bit 26 - PLL2 enable
    #[inline(always)]
    pub fn pll2on(&mut self) -> PLL2ON_W<CRrs> {
        PLL2ON_W::new(self, 26)
    }
    ///Bit 27 - PLL2 clock ready flag
    #[inline(always)]
    pub fn pll2rdy(&mut self) -> PLL2RDY_W<CRrs> {
        PLL2RDY_W::new(self, 27)
    }
    ///Bit 28 - PLL3 enable
    #[inline(always)]
    pub fn pll3on(&mut self) -> PLL3ON_W<CRrs> {
        PLL3ON_W::new(self, 28)
    }
    ///Bit 29 - PLL3 clock ready flag
    #[inline(always)]
    pub fn pll3rdy(&mut self) -> PLL3RDY_W<CRrs> {
        PLL3RDY_W::new(self, 29)
    }
}
/**clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RCC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x83
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x83;
}
