///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Internal high-speed clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
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
            false => HSION::Disabled,
            true => HSION::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSION::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSION::Enabled
    }
}
///Field `HSION` writer - Internal high-speed clock enable
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Enabled)
    }
}
/**Internal high-speed clock ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYR {
    ///0: Oscillator is not stable
    NotReady = 0,
    ///1: Oscillator is stable
    Ready = 1,
}
impl From<HSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDY` reader - Internal high-speed clock ready flag
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
    ///Oscillator is not stable
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDYR::NotReady
    }
    ///Oscillator is stable
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDYR::Ready
    }
}
///Field `MSION` reader - MSI clock enable
pub use HSION_R as MSION_R;
///Field `HSEON` reader - HSE clock enable
pub use HSION_R as HSEON_R;
///Field `MSION` writer - MSI clock enable
pub use HSION_W as MSION_W;
///Field `HSEON` writer - HSE clock enable
pub use HSION_W as HSEON_W;
///Field `MSIRDY` reader - MSI clock ready flag
pub use HSIRDY_R as MSIRDY_R;
///Field `HSERDY` reader - HSE clock ready flag
pub use HSIRDY_R as HSERDY_R;
/**HSE clock bypass

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP {
    ///0: HSE oscillator not bypassed
    NotBypassed = 0,
    ///1: HSE oscillator bypassed
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
    ///HSE oscillator not bypassed
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP::NotBypassed
    }
    ///HSE oscillator bypassed
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
    ///HSE oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::NotBypassed)
    }
    ///HSE oscillator bypassed
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::Bypassed)
    }
}
///Field `PLLON` reader - PLL enable
pub use HSION_R as PLLON_R;
///Field `PLLON` writer - PLL enable
pub use HSION_W as PLLON_W;
/**PLL clock ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYR {
    ///0: PLL unlocked
    Unlocked = 0,
    ///1: PLL locked
    Locked = 1,
}
impl From<PLLRDYR> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLRDY` reader - PLL clock ready flag
pub type PLLRDY_R = crate::BitReader<PLLRDYR>;
impl PLLRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLRDYR {
        match self.bits {
            false => PLLRDYR::Unlocked,
            true => PLLRDYR::Locked,
        }
    }
    ///PLL unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLRDYR::Unlocked
    }
    ///PLL locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLRDYR::Locked
    }
}
///Field `CSSON` reader - Clock security system enable
pub use HSION_R as CSSON_R;
///Field `CSSON` writer - Clock security system enable
pub use HSION_W as CSSON_W;
/**TC/LCD prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCPRE {
    ///0: HSE divided by 2
    Div2 = 0,
    ///1: HSE divided by 4
    Div4 = 1,
    ///2: HSE divided by 8
    Div8 = 2,
    ///3: HSE divided by 16
    Div16 = 3,
}
impl From<RTCPRE> for u8 {
    #[inline(always)]
    fn from(variant: RTCPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCPRE {
    type Ux = u8;
}
impl crate::IsEnum for RTCPRE {}
///Field `RTCPRE` reader - TC/LCD prescaler
pub type RTCPRE_R = crate::FieldReader<RTCPRE>;
impl RTCPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCPRE {
        match self.bits {
            0 => RTCPRE::Div2,
            1 => RTCPRE::Div4,
            2 => RTCPRE::Div8,
            3 => RTCPRE::Div16,
            _ => unreachable!(),
        }
    }
    ///HSE divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RTCPRE::Div2
    }
    ///HSE divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == RTCPRE::Div4
    }
    ///HSE divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == RTCPRE::Div8
    }
    ///HSE divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == RTCPRE::Div16
    }
}
///Field `RTCPRE` writer - TC/LCD prescaler
pub type RTCPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RTCPRE, crate::Safe>;
impl<'a, REG> RTCPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSE divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE::Div2)
    }
    ///HSE divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE::Div4)
    }
    ///HSE divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE::Div8)
    }
    ///HSE divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE::Div16)
    }
}
impl R {
    ///Bit 0 - Internal high-speed clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal high-speed clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - MSI clock enable
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MSI clock ready flag
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 9) & 1) != 0)
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
    ///Bit 24 - PLL enable
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - PLL clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - Clock security system enable
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:30 - TC/LCD prescaler
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rtcpre", &self.rtcpre())
            .field("hsion", &self.hsion())
            .field("csson", &self.csson())
            .field("pllrdy", &self.pllrdy())
            .field("pllon", &self.pllon())
            .field("hsebyp", &self.hsebyp())
            .field("hsirdy", &self.hsirdy())
            .field("hserdy", &self.hserdy())
            .field("hseon", &self.hseon())
            .field("msirdy", &self.msirdy())
            .field("msion", &self.msion())
            .finish()
    }
}
impl W {
    ///Bit 0 - Internal high-speed clock enable
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<'_, CRrs> {
        HSION_W::new(self, 0)
    }
    ///Bit 8 - MSI clock enable
    #[inline(always)]
    pub fn msion(&mut self) -> MSION_W<'_, CRrs> {
        MSION_W::new(self, 8)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<'_, CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 18 - HSE clock bypass
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<'_, CRrs> {
        HSEBYP_W::new(self, 18)
    }
    ///Bit 24 - PLL enable
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W<'_, CRrs> {
        PLLON_W::new(self, 24)
    }
    ///Bit 28 - Clock security system enable
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W<'_, CRrs> {
        CSSON_W::new(self, 28)
    }
    ///Bits 29:30 - TC/LCD prescaler
    #[inline(always)]
    pub fn rtcpre(&mut self) -> RTCPRE_W<'_, CRrs> {
        RTCPRE_W::new(self, 29)
    }
}
/**Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RCC:CR)*/
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
///`reset()` method sets CR to value 0x0300
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x0300;
}
