///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `MSION` reader - MSI clock enable
pub type MSION_R = crate::BitReader;
///Field `MSION` writer - MSI clock enable
pub type MSION_W<'a, REG> = crate::BitWriter<'a, REG>;
/**MSI clock ready flag

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYR {
    ///0: Clock not ready
    NotReady = 0,
    ///1: Clock ready
    Ready = 1,
}
impl From<MSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIRDY` reader - MSI clock ready flag
pub type MSIRDY_R = crate::BitReader<MSIRDYR>;
impl MSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIRDYR {
        match self.bits {
            false => MSIRDYR::NotReady,
            true => MSIRDYR::Ready,
        }
    }
    ///Clock not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == MSIRDYR::NotReady
    }
    ///Clock ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == MSIRDYR::Ready
    }
}
///Field `MSIPLLEN` reader - MSI clock PLL enable
pub type MSIPLLEN_R = crate::BitReader;
///Field `MSIPLLEN` writer - MSI clock PLL enable
pub type MSIPLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIRGSEL` writer - MSI clock range selection
pub type MSIRGSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIRANGE` reader - MSI clock ranges
pub type MSIRANGE_R = crate::FieldReader;
///Field `MSIRANGE` writer - MSI clock ranges
pub type MSIRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**HSI clock enable

Value on reset: 0*/
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
///Field `HSION` reader - HSI clock enable
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
///Field `HSION` writer - HSI clock enable
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
///Field `HSIKERON` reader - HSI always enable for peripheral kernels
pub type HSIKERON_R = crate::BitReader;
///Field `HSIKERON` writer - HSI always enable for peripheral kernels
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDY` reader - HSI clock ready flag
pub use MSIRDY_R as HSIRDY_R;
///Field `HSIASFS` reader - HSI automatic start from Stop
pub type HSIASFS_R = crate::BitReader;
///Field `HSIASFS` writer - HSI automatic start from Stop
pub type HSIASFS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEON` reader - HSE clock enable
pub use HSION_R as HSEON_R;
///Field `HSEON` writer - HSE clock enable
pub use HSION_W as HSEON_W;
///Field `HSERDY` reader - HSE clock ready flag
pub use MSIRDY_R as HSERDY_R;
/**HSE crystal oscillator bypass

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
///Field `HSEBYP` reader - HSE crystal oscillator bypass
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
///Field `HSEBYP` writer - HSE crystal oscillator bypass
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
/**Clock security system enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON {
    ///0: Clock security system disabled (clock detector OFF)
    Off = 0,
    ///1: Clock security system enable (clock detector ON if the HSE is ready, OFF if not)
    On = 1,
}
impl From<CSSON> for bool {
    #[inline(always)]
    fn from(variant: CSSON) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSON` writer - Clock security system enable
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG, CSSON>;
impl<'a, REG> CSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock security system disabled (clock detector OFF)
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Off)
    }
    ///Clock security system enable (clock detector ON if the HSE is ready, OFF if not)
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::On)
    }
}
///Field `PLLON` reader - Main PLL enable
pub use HSION_R as PLLON_R;
///Field `PLLON` writer - Main PLL enable
pub use HSION_W as PLLON_W;
///Field `PLLRDY` reader - Main PLL clock ready flag
pub use MSIRDY_R as PLLRDY_R;
///Field `PLLSAI1ON` reader - SAI1 PLL enable
pub type PLLSAI1ON_R = crate::BitReader;
///Field `PLLSAI1ON` writer - SAI1 PLL enable
pub type PLLSAI1ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI1RDY` reader - SAI1 PLL clock ready flag
pub use MSIRDY_R as PLLSAI1RDY_R;
///Field `PLLSAI2ON` reader - SAI2 PLL enable
pub type PLLSAI2ON_R = crate::BitReader;
///Field `PLLSAI2ON` writer - SAI2 PLL enable
pub type PLLSAI2ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI2RDY` reader - SAI2 PLL clock ready flag
pub use MSIRDY_R as PLLSAI2RDY_R;
///Field `PRIV` reader - PRIV
pub type PRIV_R = crate::BitReader;
///Field `PRIV` writer - PRIV
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MSI clock enable
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MSI clock ready flag
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI clock PLL enable
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:7 - MSI clock ranges
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - HSI clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSI always enable for peripheral kernels
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSI automatic start from Stop
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 1) != 0)
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
    ///Bit 18 - HSE crystal oscillator bypass
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - Main PLL enable
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Main PLL clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SAI1 PLL enable
    #[inline(always)]
    pub fn pllsai1on(&self) -> PLLSAI1ON_R {
        PLLSAI1ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SAI1 PLL clock ready flag
    #[inline(always)]
    pub fn pllsai1rdy(&self) -> PLLSAI1RDY_R {
        PLLSAI1RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SAI2 PLL enable
    #[inline(always)]
    pub fn pllsai2on(&self) -> PLLSAI2ON_R {
        PLLSAI2ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SAI2 PLL clock ready flag
    #[inline(always)]
    pub fn pllsai2rdy(&self) -> PLLSAI2RDY_R {
        PLLSAI2RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - PRIV
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("priv_", &self.priv_())
            .field("msirdy", &self.msirdy())
            .field("pllsai2rdy", &self.pllsai2rdy())
            .field("pllsai2on", &self.pllsai2on())
            .field("pllsai1rdy", &self.pllsai1rdy())
            .field("pllsai1on", &self.pllsai1on())
            .field("pllrdy", &self.pllrdy())
            .field("hsion", &self.hsion())
            .field("pllon", &self.pllon())
            .field("hsebyp", &self.hsebyp())
            .field("hserdy", &self.hserdy())
            .field("hseon", &self.hseon())
            .field("hsiasfs", &self.hsiasfs())
            .field("hsirdy", &self.hsirdy())
            .field("hsikeron", &self.hsikeron())
            .field("msirange", &self.msirange())
            .field("msipllen", &self.msipllen())
            .field("msion", &self.msion())
            .finish()
    }
}
impl W {
    ///Bit 0 - MSI clock enable
    #[inline(always)]
    pub fn msion(&mut self) -> MSION_W<'_, CRrs> {
        MSION_W::new(self, 0)
    }
    ///Bit 2 - MSI clock PLL enable
    #[inline(always)]
    pub fn msipllen(&mut self) -> MSIPLLEN_W<'_, CRrs> {
        MSIPLLEN_W::new(self, 2)
    }
    ///Bit 3 - MSI clock range selection
    #[inline(always)]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W<'_, CRrs> {
        MSIRGSEL_W::new(self, 3)
    }
    ///Bits 4:7 - MSI clock ranges
    #[inline(always)]
    pub fn msirange(&mut self) -> MSIRANGE_W<'_, CRrs> {
        MSIRANGE_W::new(self, 4)
    }
    ///Bit 8 - HSI clock enable
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<'_, CRrs> {
        HSION_W::new(self, 8)
    }
    ///Bit 9 - HSI always enable for peripheral kernels
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W<'_, CRrs> {
        HSIKERON_W::new(self, 9)
    }
    ///Bit 11 - HSI automatic start from Stop
    #[inline(always)]
    pub fn hsiasfs(&mut self) -> HSIASFS_W<'_, CRrs> {
        HSIASFS_W::new(self, 11)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<'_, CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 18 - HSE crystal oscillator bypass
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<'_, CRrs> {
        HSEBYP_W::new(self, 18)
    }
    ///Bit 19 - Clock security system enable
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W<'_, CRrs> {
        CSSON_W::new(self, 19)
    }
    ///Bit 24 - Main PLL enable
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W<'_, CRrs> {
        PLLON_W::new(self, 24)
    }
    ///Bit 26 - SAI1 PLL enable
    #[inline(always)]
    pub fn pllsai1on(&mut self) -> PLLSAI1ON_W<'_, CRrs> {
        PLLSAI1ON_W::new(self, 26)
    }
    ///Bit 28 - SAI2 PLL enable
    #[inline(always)]
    pub fn pllsai2on(&mut self) -> PLLSAI2ON_W<'_, CRrs> {
        PLLSAI2ON_W::new(self, 28)
    }
    ///Bit 31 - PRIV
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<'_, CRrs> {
        PRIV_W::new(self, 31)
    }
}
/**Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#RCC:CR)*/
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
///`reset()` method sets CR to value 0x63
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x63;
}
