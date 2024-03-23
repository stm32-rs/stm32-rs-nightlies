#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Internal high-speed clock enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    #[doc = "0: Clock Off"]
    Off = 0,
    #[doc = "1: Clock On"]
    On = 1,
}
impl From<HSION> for bool {
    #[inline(always)]
    fn from(variant: HSION) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSION` reader - Internal high-speed clock enable"]
pub type HSION_R = crate::BitReader<HSION>;
impl HSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSION {
        match self.bits {
            false => HSION::Off,
            true => HSION::On,
        }
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSION::Off
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSION::On
    }
}
#[doc = "Field `HSION` writer - Internal high-speed clock enable"]
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Off)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::On)
    }
}
#[doc = "Field `HSIKERON` reader - High Speed Internal clock enable in Stop mode"]
pub use HSION_R as HSIKERON_R;
#[doc = "Field `HSIKERON` writer - High Speed Internal clock enable in Stop mode"]
pub use HSION_W as HSIKERON_W;
#[doc = "HSI clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYR {
    #[doc = "0: Clock not ready"]
    NotReady = 0,
    #[doc = "1: Clock ready"]
    Ready = 1,
}
impl From<HSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDY` reader - HSI clock ready flag"]
pub type HSIRDY_R = crate::BitReader<HSIRDYR>;
impl HSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYR {
        match self.bits {
            false => HSIRDYR::NotReady,
            true => HSIRDYR::Ready,
        }
    }
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDYR::NotReady
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDYR::Ready
    }
}
#[doc = "Field `HSIRDY` writer - HSI clock ready flag"]
pub type HSIRDY_W<'a, REG> = crate::BitWriter<'a, REG, HSIRDYR>;
impl<'a, REG> HSIRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYR::NotReady)
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYR::Ready)
    }
}
#[doc = "HSI clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSIDIV {
    #[doc = "0: No division"]
    Div1 = 0,
    #[doc = "1: Division by 2"]
    Div2 = 1,
    #[doc = "2: Division by 4"]
    Div4 = 2,
    #[doc = "3: Division by 8"]
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
#[doc = "Field `HSIDIV` reader - HSI clock divider"]
pub type HSIDIV_R = crate::FieldReader<HSIDIV>;
impl HSIDIV_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "No division"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HSIDIV::Div1
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HSIDIV::Div2
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSIDIV::Div4
    }
    #[doc = "Division by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HSIDIV::Div8
    }
}
#[doc = "Field `HSIDIV` writer - HSI clock divider"]
pub type HSIDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HSIDIV>;
impl<'a, REG> HSIDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div1)
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div2)
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div4)
    }
    #[doc = "Division by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div8)
    }
}
#[doc = "HSI divider flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIDIVFR {
    #[doc = "0: New HSIDIV ratio has not yet propagated to hsi_ck"]
    NotPropagated = 0,
    #[doc = "1: HSIDIV ratio has propagated to hsi_ck"]
    Propagated = 1,
}
impl From<HSIDIVFR> for bool {
    #[inline(always)]
    fn from(variant: HSIDIVFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIDIVF` reader - HSI divider flag"]
pub type HSIDIVF_R = crate::BitReader<HSIDIVFR>;
impl HSIDIVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIDIVFR {
        match self.bits {
            false => HSIDIVFR::NotPropagated,
            true => HSIDIVFR::Propagated,
        }
    }
    #[doc = "New HSIDIV ratio has not yet propagated to hsi_ck"]
    #[inline(always)]
    pub fn is_not_propagated(&self) -> bool {
        *self == HSIDIVFR::NotPropagated
    }
    #[doc = "HSIDIV ratio has propagated to hsi_ck"]
    #[inline(always)]
    pub fn is_propagated(&self) -> bool {
        *self == HSIDIVFR::Propagated
    }
}
#[doc = "Field `HSIDIVF` writer - HSI divider flag"]
pub type HSIDIVF_W<'a, REG> = crate::BitWriter<'a, REG, HSIDIVFR>;
impl<'a, REG> HSIDIVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "New HSIDIV ratio has not yet propagated to hsi_ck"]
    #[inline(always)]
    pub fn not_propagated(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIVFR::NotPropagated)
    }
    #[doc = "HSIDIV ratio has propagated to hsi_ck"]
    #[inline(always)]
    pub fn propagated(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIVFR::Propagated)
    }
}
#[doc = "Field `CSION` reader - CSI clock enable"]
pub use HSION_R as CSION_R;
#[doc = "Field `CSIKERON` reader - CSI clock enable in Stop mode"]
pub use HSION_R as CSIKERON_R;
#[doc = "Field `HSI48ON` reader - RC48 clock enable"]
pub use HSION_R as HSI48ON_R;
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub use HSION_R as HSEON_R;
#[doc = "Field `CSION` writer - CSI clock enable"]
pub use HSION_W as CSION_W;
#[doc = "Field `CSIKERON` writer - CSI clock enable in Stop mode"]
pub use HSION_W as CSIKERON_W;
#[doc = "Field `HSI48ON` writer - RC48 clock enable"]
pub use HSION_W as HSI48ON_W;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub use HSION_W as HSEON_W;
#[doc = "Field `CSIRDY` reader - CSI clock ready flag"]
pub use HSIRDY_R as CSIRDY_R;
#[doc = "Field `HSI48RDY` reader - RC48 clock ready flag"]
pub use HSIRDY_R as HSI48RDY_R;
#[doc = "Field `D1CKRDY` reader - D1 domain clocks ready flag"]
pub use HSIRDY_R as D1CKRDY_R;
#[doc = "Field `D2CKRDY` reader - D2 domain clocks ready flag"]
pub use HSIRDY_R as D2CKRDY_R;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub use HSIRDY_R as HSERDY_R;
#[doc = "Field `CSIRDY` writer - CSI clock ready flag"]
pub use HSIRDY_W as CSIRDY_W;
#[doc = "Field `HSI48RDY` writer - RC48 clock ready flag"]
pub use HSIRDY_W as HSI48RDY_W;
#[doc = "Field `D1CKRDY` writer - D1 domain clocks ready flag"]
pub use HSIRDY_W as D1CKRDY_W;
#[doc = "Field `D2CKRDY` writer - D2 domain clocks ready flag"]
pub use HSIRDY_W as D2CKRDY_W;
#[doc = "Field `HSERDY` writer - HSE clock ready flag"]
pub use HSIRDY_W as HSERDY_W;
#[doc = "HSE clock bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP {
    #[doc = "0: HSE crystal oscillator not bypassed"]
    NotBypassed = 0,
    #[doc = "1: HSE crystal oscillator bypassed with external clock"]
    Bypassed = 1,
}
impl From<HSEBYP> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEBYP` reader - HSE clock bypass"]
pub type HSEBYP_R = crate::BitReader<HSEBYP>;
impl HSEBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSEBYP {
        match self.bits {
            false => HSEBYP::NotBypassed,
            true => HSEBYP::Bypassed,
        }
    }
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP::NotBypassed
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP::Bypassed
    }
}
#[doc = "Field `HSEBYP` writer - HSE clock bypass"]
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, HSEBYP>;
impl<'a, REG> HSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::NotBypassed)
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::Bypassed)
    }
}
#[doc = "Field `HSECSSON` reader - HSE Clock Security System enable"]
pub use HSION_R as HSECSSON_R;
#[doc = "Field `PLL1ON` reader - PLL1 enable"]
pub use HSION_R as PLL1ON_R;
#[doc = "Field `PLL2ON` reader - PLL2 enable"]
pub use HSION_R as PLL2ON_R;
#[doc = "Field `PLL3ON` reader - PLL3 enable"]
pub use HSION_R as PLL3ON_R;
#[doc = "Field `HSECSSON` writer - HSE Clock Security System enable"]
pub use HSION_W as HSECSSON_W;
#[doc = "Field `PLL1ON` writer - PLL1 enable"]
pub use HSION_W as PLL1ON_W;
#[doc = "Field `PLL2ON` writer - PLL2 enable"]
pub use HSION_W as PLL2ON_W;
#[doc = "Field `PLL3ON` writer - PLL3 enable"]
pub use HSION_W as PLL3ON_W;
#[doc = "Field `PLL1RDY` reader - PLL1 clock ready flag"]
pub use HSIRDY_R as PLL1RDY_R;
#[doc = "Field `PLL2RDY` reader - PLL2 clock ready flag"]
pub use HSIRDY_R as PLL2RDY_R;
#[doc = "Field `PLL3RDY` reader - PLL3 clock ready flag"]
pub use HSIRDY_R as PLL3RDY_R;
#[doc = "Field `PLL1RDY` writer - PLL1 clock ready flag"]
pub use HSIRDY_W as PLL1RDY_W;
#[doc = "Field `PLL2RDY` writer - PLL2 clock ready flag"]
pub use HSIRDY_W as PLL2RDY_W;
#[doc = "Field `PLL3RDY` writer - PLL3 clock ready flag"]
pub use HSIRDY_W as PLL3RDY_W;
impl R {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High Speed Internal clock enable in Stop mode"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - HSI clock divider"]
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - HSI divider flag"]
    #[inline(always)]
    pub fn hsidivf(&self) -> HSIDIVF_R {
        HSIDIVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - CSI clock enable"]
    #[inline(always)]
    pub fn csion(&self) -> CSION_R {
        CSION_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CSI clock ready flag"]
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSI clock enable in Stop mode"]
    #[inline(always)]
    pub fn csikeron(&self) -> CSIKERON_R {
        CSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - RC48 clock enable"]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RC48 clock ready flag"]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - D1 domain clocks ready flag"]
    #[inline(always)]
    pub fn d1ckrdy(&self) -> D1CKRDY_R {
        D1CKRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - D2 domain clocks ready flag"]
    #[inline(always)]
    pub fn d2ckrdy(&self) -> D2CKRDY_R {
        D2CKRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE Clock Security System enable"]
    #[inline(always)]
    pub fn hsecsson(&self) -> HSECSSON_R {
        HSECSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL1 enable"]
    #[inline(always)]
    pub fn pll1on(&self) -> PLL1ON_R {
        PLL1ON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL1 clock ready flag"]
    #[inline(always)]
    pub fn pll1rdy(&self) -> PLL1RDY_R {
        PLL1RDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PLL2 enable"]
    #[inline(always)]
    pub fn pll2on(&self) -> PLL2ON_R {
        PLL2ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL2 clock ready flag"]
    #[inline(always)]
    pub fn pll2rdy(&self) -> PLL2RDY_R {
        PLL2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PLL3 enable"]
    #[inline(always)]
    pub fn pll3on(&self) -> PLL3ON_R {
        PLL3ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLL3 clock ready flag"]
    #[inline(always)]
    pub fn pll3rdy(&self) -> PLL3RDY_R {
        PLL3RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 0)
    }
    #[doc = "Bit 1 - High Speed Internal clock enable in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<CRrs> {
        HSIKERON_W::new(self, 1)
    }
    #[doc = "Bit 2 - HSI clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdy(&mut self) -> HSIRDY_W<CRrs> {
        HSIRDY_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - HSI clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn hsidiv(&mut self) -> HSIDIV_W<CRrs> {
        HSIDIV_W::new(self, 3)
    }
    #[doc = "Bit 5 - HSI divider flag"]
    #[inline(always)]
    #[must_use]
    pub fn hsidivf(&mut self) -> HSIDIVF_W<CRrs> {
        HSIDIVF_W::new(self, 5)
    }
    #[doc = "Bit 7 - CSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn csion(&mut self) -> CSION_W<CRrs> {
        CSION_W::new(self, 7)
    }
    #[doc = "Bit 8 - CSI clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn csirdy(&mut self) -> CSIRDY_W<CRrs> {
        CSIRDY_W::new(self, 8)
    }
    #[doc = "Bit 9 - CSI clock enable in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn csikeron(&mut self) -> CSIKERON_W<CRrs> {
        CSIKERON_W::new(self, 9)
    }
    #[doc = "Bit 12 - RC48 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48on(&mut self) -> HSI48ON_W<CRrs> {
        HSI48ON_W::new(self, 12)
    }
    #[doc = "Bit 13 - RC48 clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdy(&mut self) -> HSI48RDY_W<CRrs> {
        HSI48RDY_W::new(self, 13)
    }
    #[doc = "Bit 14 - D1 domain clocks ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn d1ckrdy(&mut self) -> D1CKRDY_W<CRrs> {
        D1CKRDY_W::new(self, 14)
    }
    #[doc = "Bit 15 - D2 domain clocks ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn d2ckrdy(&mut self) -> D2CKRDY_W<CRrs> {
        D2CKRDY_W::new(self, 15)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn hserdy(&mut self) -> HSERDY_W<CRrs> {
        HSERDY_W::new(self, 17)
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CRrs> {
        HSEBYP_W::new(self, 18)
    }
    #[doc = "Bit 19 - HSE Clock Security System enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsecsson(&mut self) -> HSECSSON_W<CRrs> {
        HSECSSON_W::new(self, 19)
    }
    #[doc = "Bit 24 - PLL1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll1on(&mut self) -> PLL1ON_W<CRrs> {
        PLL1ON_W::new(self, 24)
    }
    #[doc = "Bit 25 - PLL1 clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn pll1rdy(&mut self) -> PLL1RDY_W<CRrs> {
        PLL1RDY_W::new(self, 25)
    }
    #[doc = "Bit 26 - PLL2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll2on(&mut self) -> PLL2ON_W<CRrs> {
        PLL2ON_W::new(self, 26)
    }
    #[doc = "Bit 27 - PLL2 clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn pll2rdy(&mut self) -> PLL2RDY_W<CRrs> {
        PLL2RDY_W::new(self, 27)
    }
    #[doc = "Bit 28 - PLL3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll3on(&mut self) -> PLL3ON_W<CRrs> {
        PLL3ON_W::new(self, 28)
    }
    #[doc = "Bit 29 - PLL3 clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn pll3rdy(&mut self) -> PLL3RDY_W<CRrs> {
        PLL3RDY_W::new(self, 29)
    }
}
#[doc = "clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x83"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x83;
}
