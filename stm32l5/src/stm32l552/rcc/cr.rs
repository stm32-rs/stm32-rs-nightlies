#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `MSION` reader - MSI clock enable"]
pub type MSION_R = crate::BitReader;
#[doc = "Field `MSION` writer - MSI clock enable"]
pub type MSION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MSI clock ready flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYR {
    #[doc = "0: Clock not ready"]
    NotReady = 0,
    #[doc = "1: Clock ready"]
    Ready = 1,
}
impl From<MSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIRDY` reader - MSI clock ready flag"]
pub type MSIRDY_R = crate::BitReader<MSIRDYR>;
impl MSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSIRDYR {
        match self.bits {
            false => MSIRDYR::NotReady,
            true => MSIRDYR::Ready,
        }
    }
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == MSIRDYR::NotReady
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == MSIRDYR::Ready
    }
}
#[doc = "Field `MSIPLLEN` reader - MSI clock PLL enable"]
pub type MSIPLLEN_R = crate::BitReader;
#[doc = "Field `MSIPLLEN` writer - MSI clock PLL enable"]
pub type MSIPLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIRGSEL` writer - MSI clock range selection"]
pub type MSIRGSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIRANGE` reader - MSI clock ranges"]
pub type MSIRANGE_R = crate::FieldReader;
#[doc = "Field `MSIRANGE` writer - MSI clock ranges"]
pub type MSIRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "HSI clock enable\n\nValue on reset: 0"]
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
#[doc = "Field `HSION` reader - HSI clock enable"]
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
#[doc = "Field `HSION` writer - HSI clock enable"]
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
#[doc = "Field `HSIKERON` reader - HSI always enable for peripheral kernels"]
pub type HSIKERON_R = crate::BitReader;
#[doc = "Field `HSIKERON` writer - HSI always enable for peripheral kernels"]
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - HSI clock ready flag"]
pub use MSIRDY_R as HSIRDY_R;
#[doc = "Field `HSIASFS` reader - HSI automatic start from Stop"]
pub type HSIASFS_R = crate::BitReader;
#[doc = "Field `HSIASFS` writer - HSI automatic start from Stop"]
pub type HSIASFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub use HSION_R as HSEON_R;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub use HSION_W as HSEON_W;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub use MSIRDY_R as HSERDY_R;
#[doc = "HSE crystal oscillator bypass\n\nValue on reset: 0"]
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
#[doc = "Field `HSEBYP` reader - HSE crystal oscillator bypass"]
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
#[doc = "Field `HSEBYP` writer - HSE crystal oscillator bypass"]
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
#[doc = "Clock security system enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON {
    #[doc = "0: Clock security system disabled (clock detector OFF)"]
    Off = 0,
    #[doc = "1: Clock security system enable (clock detector ON if the HSE is ready, OFF if not)"]
    On = 1,
}
impl From<CSSON> for bool {
    #[inline(always)]
    fn from(variant: CSSON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSON` writer - Clock security system enable"]
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG, CSSON>;
impl<'a, REG> CSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock security system disabled (clock detector OFF)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Off)
    }
    #[doc = "Clock security system enable (clock detector ON if the HSE is ready, OFF if not)"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::On)
    }
}
#[doc = "Field `PLLON` reader - Main PLL enable"]
pub use HSION_R as PLLON_R;
#[doc = "Field `PLLON` writer - Main PLL enable"]
pub use HSION_W as PLLON_W;
#[doc = "Field `PLLRDY` reader - Main PLL clock ready flag"]
pub use MSIRDY_R as PLLRDY_R;
#[doc = "Field `PLLSAI1ON` reader - SAI1 PLL enable"]
pub type PLLSAI1ON_R = crate::BitReader;
#[doc = "Field `PLLSAI1ON` writer - SAI1 PLL enable"]
pub type PLLSAI1ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1RDY` reader - SAI1 PLL clock ready flag"]
pub use MSIRDY_R as PLLSAI1RDY_R;
#[doc = "Field `PLLSAI2ON` reader - SAI2 PLL enable"]
pub type PLLSAI2ON_R = crate::BitReader;
#[doc = "Field `PLLSAI2ON` writer - SAI2 PLL enable"]
pub type PLLSAI2ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI2RDY` reader - SAI2 PLL clock ready flag"]
pub use MSIRDY_R as PLLSAI2RDY_R;
#[doc = "Field `PRIV` reader - PRIV"]
pub type PRIV_R = crate::BitReader;
#[doc = "Field `PRIV` writer - PRIV"]
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSI clock ready flag"]
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - HSI clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSI always enable for peripheral kernels"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI automatic start from Stop"]
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Main PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SAI1 PLL enable"]
    #[inline(always)]
    pub fn pllsai1on(&self) -> PLLSAI1ON_R {
        PLLSAI1ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SAI1 PLL clock ready flag"]
    #[inline(always)]
    pub fn pllsai1rdy(&self) -> PLLSAI1RDY_R {
        PLLSAI1RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SAI2 PLL enable"]
    #[inline(always)]
    pub fn pllsai2on(&self) -> PLLSAI2ON_R {
        PLLSAI2ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SAI2 PLL clock ready flag"]
    #[inline(always)]
    pub fn pllsai2rdy(&self) -> PLLSAI2RDY_R {
        PLLSAI2RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - PRIV"]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn msion(&mut self) -> MSION_W<CRrs> {
        MSION_W::new(self, 0)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn msipllen(&mut self) -> MSIPLLEN_W<CRrs> {
        MSIPLLEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - MSI clock range selection"]
    #[inline(always)]
    #[must_use]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W<CRrs> {
        MSIRGSEL_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    #[must_use]
    pub fn msirange(&mut self) -> MSIRANGE_W<CRrs> {
        MSIRANGE_W::new(self, 4)
    }
    #[doc = "Bit 8 - HSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 8)
    }
    #[doc = "Bit 9 - HSI always enable for peripheral kernels"]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<CRrs> {
        HSIKERON_W::new(self, 9)
    }
    #[doc = "Bit 11 - HSI automatic start from Stop"]
    #[inline(always)]
    #[must_use]
    pub fn hsiasfs(&mut self) -> HSIASFS_W<CRrs> {
        HSIASFS_W::new(self, 11)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CRrs> {
        HSEBYP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<CRrs> {
        CSSON_W::new(self, 19)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<CRrs> {
        PLLON_W::new(self, 24)
    }
    #[doc = "Bit 26 - SAI1 PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1on(&mut self) -> PLLSAI1ON_W<CRrs> {
        PLLSAI1ON_W::new(self, 26)
    }
    #[doc = "Bit 28 - SAI2 PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2on(&mut self) -> PLLSAI2ON_W<CRrs> {
        PLLSAI2ON_W::new(self, 28)
    }
    #[doc = "Bit 31 - PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<CRrs> {
        PRIV_W::new(self, 31)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0x63"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x63;
}
