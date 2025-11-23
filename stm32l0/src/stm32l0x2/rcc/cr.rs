///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**16 MHz high-speed internal clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16ON {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<HSI16ON> for bool {
    #[inline(always)]
    fn from(variant: HSI16ON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSI16ON` reader - 16 MHz high-speed internal clock enable
pub type HSI16ON_R = crate::BitReader<HSI16ON>;
impl HSI16ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSI16ON {
        match self.bits {
            false => HSI16ON::Disabled,
            true => HSI16ON::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI16ON::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI16ON::Enabled
    }
}
///Field `HSI16ON` writer - 16 MHz high-speed internal clock enable
pub type HSI16ON_W<'a, REG> = crate::BitWriter<'a, REG, HSI16ON>;
impl<'a, REG> HSI16ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16ON::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16ON::Enabled)
    }
}
///Field `HSI16KERON` reader - High-speed internal clock enable bit for some IP kernels
pub use HSI16ON_R as HSI16KERON_R;
/**Internal high-speed clock ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16RDYFR {
    ///0: HSI 16 MHz oscillator not ready
    NotReady = 0,
    ///1: HSI 16 MHz oscillator ready
    Ready = 1,
}
impl From<HSI16RDYFR> for bool {
    #[inline(always)]
    fn from(variant: HSI16RDYFR) -> Self {
        variant as u8 != 0
    }
}
///Field `HSI16RDYF` reader - Internal high-speed clock ready flag
pub type HSI16RDYF_R = crate::BitReader<HSI16RDYFR>;
impl HSI16RDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSI16RDYFR {
        match self.bits {
            false => HSI16RDYFR::NotReady,
            true => HSI16RDYFR::Ready,
        }
    }
    ///HSI 16 MHz oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSI16RDYFR::NotReady
    }
    ///HSI 16 MHz oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSI16RDYFR::Ready
    }
}
///Field `HSI16RDYF` writer - Internal high-speed clock ready flag
pub type HSI16RDYF_W<'a, REG> = crate::BitWriter<'a, REG, HSI16RDYFR>;
impl<'a, REG> HSI16RDYF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI 16 MHz oscillator not ready
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16RDYFR::NotReady)
    }
    ///HSI 16 MHz oscillator ready
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16RDYFR::Ready)
    }
}
/**HSI16DIVEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16DIVEN {
    ///0: no 16 MHz HSI division requested
    NotDivided = 0,
    ///1: 16 MHz HSI division by 4 requested
    Div4 = 1,
}
impl From<HSI16DIVEN> for bool {
    #[inline(always)]
    fn from(variant: HSI16DIVEN) -> Self {
        variant as u8 != 0
    }
}
///Field `HSI16DIVEN` reader - HSI16DIVEN
pub type HSI16DIVEN_R = crate::BitReader<HSI16DIVEN>;
impl HSI16DIVEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSI16DIVEN {
        match self.bits {
            false => HSI16DIVEN::NotDivided,
            true => HSI16DIVEN::Div4,
        }
    }
    ///no 16 MHz HSI division requested
    #[inline(always)]
    pub fn is_not_divided(&self) -> bool {
        *self == HSI16DIVEN::NotDivided
    }
    ///16 MHz HSI division by 4 requested
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSI16DIVEN::Div4
    }
}
///Field `HSI16DIVEN` writer - HSI16DIVEN
pub type HSI16DIVEN_W<'a, REG> = crate::BitWriter<'a, REG, HSI16DIVEN>;
impl<'a, REG> HSI16DIVEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no 16 MHz HSI division requested
    #[inline(always)]
    pub fn not_divided(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16DIVEN::NotDivided)
    }
    ///16 MHz HSI division by 4 requested
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16DIVEN::Div4)
    }
}
/**HSI16DIVF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16DIVFR {
    ///0: 16 MHz HSI clock not divided
    NotDivided = 0,
    ///1: 16 MHz HSI clock divided by 4
    Div4 = 1,
}
impl From<HSI16DIVFR> for bool {
    #[inline(always)]
    fn from(variant: HSI16DIVFR) -> Self {
        variant as u8 != 0
    }
}
///Field `HSI16DIVF` reader - HSI16DIVF
pub type HSI16DIVF_R = crate::BitReader<HSI16DIVFR>;
impl HSI16DIVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSI16DIVFR {
        match self.bits {
            false => HSI16DIVFR::NotDivided,
            true => HSI16DIVFR::Div4,
        }
    }
    ///16 MHz HSI clock not divided
    #[inline(always)]
    pub fn is_not_divided(&self) -> bool {
        *self == HSI16DIVFR::NotDivided
    }
    ///16 MHz HSI clock divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSI16DIVFR::Div4
    }
}
/**16 MHz high-speed internal clock output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16OUTEN {
    ///0: HSI output clock disabled
    Disabled = 0,
    ///1: HSI output clock enabled
    Enabled = 1,
}
impl From<HSI16OUTEN> for bool {
    #[inline(always)]
    fn from(variant: HSI16OUTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `HSI16OUTEN` reader - 16 MHz high-speed internal clock output enable
pub type HSI16OUTEN_R = crate::BitReader<HSI16OUTEN>;
impl HSI16OUTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSI16OUTEN {
        match self.bits {
            false => HSI16OUTEN::Disabled,
            true => HSI16OUTEN::Enabled,
        }
    }
    ///HSI output clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI16OUTEN::Disabled
    }
    ///HSI output clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI16OUTEN::Enabled
    }
}
///Field `HSI16OUTEN` writer - 16 MHz high-speed internal clock output enable
pub type HSI16OUTEN_W<'a, REG> = crate::BitWriter<'a, REG, HSI16OUTEN>;
impl<'a, REG> HSI16OUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI output clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16OUTEN::Disabled)
    }
    ///HSI output clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16OUTEN::Enabled)
    }
}
///Field `MSION` reader - MSI clock enable bit
pub use HSI16ON_R as MSION_R;
///Field `MSION` writer - MSI clock enable bit
pub use HSI16ON_W as MSION_W;
/**MSI clock ready flag

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYR {
    ///0: Oscillator is not stable
    NotReady = 0,
    ///1: Oscillator is stable
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
    ///Oscillator is not stable
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == MSIRDYR::NotReady
    }
    ///Oscillator is stable
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == MSIRDYR::Ready
    }
}
///Field `HSEON` reader - HSE clock enable bit
pub use HSI16ON_R as HSEON_R;
///Field `HSEON` writer - HSE clock enable bit
pub use HSI16ON_W as HSEON_W;
///Field `HSERDY` reader - HSE clock ready flag
pub use MSIRDY_R as HSERDY_R;
/**HSE clock bypass bit

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
///Field `HSEBYP` reader - HSE clock bypass bit
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
///Field `HSEBYP` writer - HSE clock bypass bit
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
///Field `CSSHSEON` reader - Clock security system on HSE enable bit
pub use HSI16ON_R as CSSHSEON_R;
///Field `CSSHSEON` writer - Clock security system on HSE enable bit
pub use HSI16ON_W as CSSHSEON_W;
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
///Field `PLLON` reader - PLL enable bit
pub use HSI16ON_R as PLLON_R;
///Field `PLLON` writer - PLL enable bit
pub use HSI16ON_W as PLLON_W;
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
impl R {
    ///Bit 0 - 16 MHz high-speed internal clock enable
    #[inline(always)]
    pub fn hsi16on(&self) -> HSI16ON_R {
        HSI16ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - High-speed internal clock enable bit for some IP kernels
    #[inline(always)]
    pub fn hsi16keron(&self) -> HSI16KERON_R {
        HSI16KERON_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Internal high-speed clock ready flag
    #[inline(always)]
    pub fn hsi16rdyf(&self) -> HSI16RDYF_R {
        HSI16RDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI16DIVEN
    #[inline(always)]
    pub fn hsi16diven(&self) -> HSI16DIVEN_R {
        HSI16DIVEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSI16DIVF
    #[inline(always)]
    pub fn hsi16divf(&self) -> HSI16DIVF_R {
        HSI16DIVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - 16 MHz high-speed internal clock output enable
    #[inline(always)]
    pub fn hsi16outen(&self) -> HSI16OUTEN_R {
        HSI16OUTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - MSI clock enable bit
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MSI clock ready flag
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - HSE clock enable bit
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HSE clock bypass bit
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Clock security system on HSE enable bit
    #[inline(always)]
    pub fn csshseon(&self) -> CSSHSEON_R {
        CSSHSEON_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - TC/LCD prescaler
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 24 - PLL enable bit
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - PLL clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("pllrdy", &self.pllrdy())
            .field("hsi16on", &self.hsi16on())
            .field("pllon", &self.pllon())
            .field("rtcpre", &self.rtcpre())
            .field("csshseon", &self.csshseon())
            .field("hsebyp", &self.hsebyp())
            .field("msirdy", &self.msirdy())
            .field("hserdy", &self.hserdy())
            .field("hseon", &self.hseon())
            .field("msion", &self.msion())
            .field("hsi16divf", &self.hsi16divf())
            .field("hsi16diven", &self.hsi16diven())
            .field("hsi16rdyf", &self.hsi16rdyf())
            .field("hsi16keron", &self.hsi16keron())
            .field("hsi16outen", &self.hsi16outen())
            .finish()
    }
}
impl W {
    ///Bit 0 - 16 MHz high-speed internal clock enable
    #[inline(always)]
    pub fn hsi16on(&mut self) -> HSI16ON_W<'_, CRrs> {
        HSI16ON_W::new(self, 0)
    }
    ///Bit 2 - Internal high-speed clock ready flag
    #[inline(always)]
    pub fn hsi16rdyf(&mut self) -> HSI16RDYF_W<'_, CRrs> {
        HSI16RDYF_W::new(self, 2)
    }
    ///Bit 3 - HSI16DIVEN
    #[inline(always)]
    pub fn hsi16diven(&mut self) -> HSI16DIVEN_W<'_, CRrs> {
        HSI16DIVEN_W::new(self, 3)
    }
    ///Bit 5 - 16 MHz high-speed internal clock output enable
    #[inline(always)]
    pub fn hsi16outen(&mut self) -> HSI16OUTEN_W<'_, CRrs> {
        HSI16OUTEN_W::new(self, 5)
    }
    ///Bit 8 - MSI clock enable bit
    #[inline(always)]
    pub fn msion(&mut self) -> MSION_W<'_, CRrs> {
        MSION_W::new(self, 8)
    }
    ///Bit 16 - HSE clock enable bit
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<'_, CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 18 - HSE clock bypass bit
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<'_, CRrs> {
        HSEBYP_W::new(self, 18)
    }
    ///Bit 19 - Clock security system on HSE enable bit
    #[inline(always)]
    pub fn csshseon(&mut self) -> CSSHSEON_W<'_, CRrs> {
        CSSHSEON_W::new(self, 19)
    }
    ///Bits 20:21 - TC/LCD prescaler
    #[inline(always)]
    pub fn rtcpre(&mut self) -> RTCPRE_W<'_, CRrs> {
        RTCPRE_W::new(self, 20)
    }
    ///Bit 24 - PLL enable bit
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W<'_, CRrs> {
        PLLON_W::new(self, 24)
    }
}
/**Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x2.html#RCC:CR)*/
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
