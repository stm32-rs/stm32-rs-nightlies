///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**HSI16 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    ///0: HSI oscillator powered off
    Disabled = 0,
    ///1: HSI oscillator enabled
    Enabled = 1,
}
impl From<HSION> for bool {
    #[inline(always)]
    fn from(variant: HSION) -> Self {
        variant as u8 != 0
    }
}
///Field `HSION` reader - HSI16 clock enable
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
    ///HSI oscillator powered off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSION::Disabled
    }
    ///HSI oscillator enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSION::Enabled
    }
}
///Field `HSION` writer - HSI16 clock enable
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI oscillator powered off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Disabled)
    }
    ///HSI oscillator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Enabled)
    }
}
/**HSI16 always enable for peripheral kernels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERON {
    ///0: No effect on HSI16 oscillator
    NotForce = 0,
    ///1: HSI16 oscillator forced on even in Stop modes
    Forced = 1,
}
impl From<HSIKERON> for bool {
    #[inline(always)]
    fn from(variant: HSIKERON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIKERON` reader - HSI16 always enable for peripheral kernels
pub type HSIKERON_R = crate::BitReader<HSIKERON>;
impl HSIKERON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIKERON {
        match self.bits {
            false => HSIKERON::NotForce,
            true => HSIKERON::Forced,
        }
    }
    ///No effect on HSI16 oscillator
    #[inline(always)]
    pub fn is_not_force(&self) -> bool {
        *self == HSIKERON::NotForce
    }
    ///HSI16 oscillator forced on even in Stop modes
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == HSIKERON::Forced
    }
}
///Field `HSIKERON` writer - HSI16 always enable for peripheral kernels
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG, HSIKERON>;
impl<'a, REG> HSIKERON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on HSI16 oscillator
    #[inline(always)]
    pub fn not_force(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON::NotForce)
    }
    ///HSI16 oscillator forced on even in Stop modes
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON::Forced)
    }
}
/**HSI16 clock ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDY {
    ///0: HSI oscillator not ready
    NotReady = 0,
    ///1: HSI oscillator ready
    Ready = 1,
}
impl From<HSIRDY> for bool {
    #[inline(always)]
    fn from(variant: HSIRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDY` reader - HSI16 clock ready flag
pub type HSIRDY_R = crate::BitReader<HSIRDY>;
impl HSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDY {
        match self.bits {
            false => HSIRDY::NotReady,
            true => HSIRDY::Ready,
        }
    }
    ///HSI oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDY::NotReady
    }
    ///HSI oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDY::Ready
    }
}
///Field `HSIRDY` writer - HSI16 clock ready flag
pub type HSIRDY_W<'a, REG> = crate::BitWriter<'a, REG, HSIRDY>;
impl<'a, REG> HSIRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI oscillator not ready
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDY::NotReady)
    }
    ///HSI oscillator ready
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDY::Ready)
    }
}
/**HSI16 clock division factor

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSIDIV {
    ///0: Divide HSI16 by 1
    Div1 = 0,
    ///1: Divide HSI16 by 2
    Div2 = 1,
    ///2: Divide HSI16 by 4
    Div4 = 2,
    ///3: Divide HSI16 by 8
    Div8 = 3,
    ///4: Divide HSI16 by 16
    Div16 = 4,
    ///5: Divide HSI16 by 32
    Div32 = 5,
    ///6: Divide HSI16 by 64
    Div64 = 6,
    ///7: Divide HSI16 by 128
    Div128 = 7,
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
///Field `HSIDIV` reader - HSI16 clock division factor
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
            4 => HSIDIV::Div16,
            5 => HSIDIV::Div32,
            6 => HSIDIV::Div64,
            7 => HSIDIV::Div128,
            _ => unreachable!(),
        }
    }
    ///Divide HSI16 by 1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HSIDIV::Div1
    }
    ///Divide HSI16 by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HSIDIV::Div2
    }
    ///Divide HSI16 by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSIDIV::Div4
    }
    ///Divide HSI16 by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HSIDIV::Div8
    }
    ///Divide HSI16 by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HSIDIV::Div16
    }
    ///Divide HSI16 by 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == HSIDIV::Div32
    }
    ///Divide HSI16 by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HSIDIV::Div64
    }
    ///Divide HSI16 by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HSIDIV::Div128
    }
}
///Field `HSIDIV` writer - HSI16 clock division factor
pub type HSIDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, HSIDIV, crate::Safe>;
impl<'a, REG> HSIDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Divide HSI16 by 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div1)
    }
    ///Divide HSI16 by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div2)
    }
    ///Divide HSI16 by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div4)
    }
    ///Divide HSI16 by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div8)
    }
    ///Divide HSI16 by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div16)
    }
    ///Divide HSI16 by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div32)
    }
    ///Divide HSI16 by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div64)
    }
    ///Divide HSI16 by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::Div128)
    }
}
/**HSE clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEON {
    ///0: HSE oscillator powered off
    Disabled = 0,
    ///1: HSE oscillator enabled
    Enabled = 1,
}
impl From<HSEON> for bool {
    #[inline(always)]
    fn from(variant: HSEON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEON` reader - HSE clock enable
pub type HSEON_R = crate::BitReader<HSEON>;
impl HSEON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEON {
        match self.bits {
            false => HSEON::Disabled,
            true => HSEON::Enabled,
        }
    }
    ///HSE oscillator powered off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSEON::Disabled
    }
    ///HSE oscillator enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSEON::Enabled
    }
}
///Field `HSEON` writer - HSE clock enable
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG, HSEON>;
impl<'a, REG> HSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE oscillator powered off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON::Disabled)
    }
    ///HSE oscillator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON::Enabled)
    }
}
/**HSE clock ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDY {
    ///0: HSE oscillator not ready
    NotReady = 0,
    ///1: HSE oscillator ready
    Ready = 1,
}
impl From<HSERDY> for bool {
    #[inline(always)]
    fn from(variant: HSERDY) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDY` reader - HSE clock ready flag
pub type HSERDY_R = crate::BitReader<HSERDY>;
impl HSERDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSERDY {
        match self.bits {
            false => HSERDY::NotReady,
            true => HSERDY::Ready,
        }
    }
    ///HSE oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSERDY::NotReady
    }
    ///HSE oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSERDY::Ready
    }
}
///Field `HSERDY` writer - HSE clock ready flag
pub type HSERDY_W<'a, REG> = crate::BitWriter<'a, REG, HSERDY>;
impl<'a, REG> HSERDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE oscillator not ready
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDY::NotReady)
    }
    ///HSE oscillator ready
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDY::Ready)
    }
}
/**HSE crystal oscillator bypass

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP {
    ///0: HSE is a crystal oscillator or ceramic resonator
    Crystal = 0,
    ///1: HSE is driven by an external clock
    ExtClock = 1,
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
            false => HSEBYP::Crystal,
            true => HSEBYP::ExtClock,
        }
    }
    ///HSE is a crystal oscillator or ceramic resonator
    #[inline(always)]
    pub fn is_crystal(&self) -> bool {
        *self == HSEBYP::Crystal
    }
    ///HSE is driven by an external clock
    #[inline(always)]
    pub fn is_ext_clock(&self) -> bool {
        *self == HSEBYP::ExtClock
    }
}
///Field `HSEBYP` writer - HSE crystal oscillator bypass
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, HSEBYP>;
impl<'a, REG> HSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE is a crystal oscillator or ceramic resonator
    #[inline(always)]
    pub fn crystal(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::Crystal)
    }
    ///HSE is driven by an external clock
    #[inline(always)]
    pub fn ext_clock(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::ExtClock)
    }
}
/**Clock security system enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON {
    ///0: HSE clock is not monitored
    Disabled = 0,
    ///1: HSE clock monitor enabled when HSE is ready, otherwise disabled
    Enabled = 1,
}
impl From<CSSON> for bool {
    #[inline(always)]
    fn from(variant: CSSON) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSON` reader - Clock security system enable
pub type CSSON_R = crate::BitReader<CSSON>;
impl CSSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSON {
        match self.bits {
            false => CSSON::Disabled,
            true => CSSON::Enabled,
        }
    }
    ///HSE clock is not monitored
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSSON::Disabled
    }
    ///HSE clock monitor enabled when HSE is ready, otherwise disabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSSON::Enabled
    }
}
///Field `CSSON` writer - Clock security system enable
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG, CSSON>;
impl<'a, REG> CSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE clock is not monitored
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Disabled)
    }
    ///HSE clock monitor enabled when HSE is ready, otherwise disabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Enabled)
    }
}
/**PLL enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLON {
    ///0: PLL powered off
    Disabled = 0,
    ///1: PLL enabled
    Enabled = 1,
}
impl From<PLLON> for bool {
    #[inline(always)]
    fn from(variant: PLLON) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLON` reader - PLL enable
pub type PLLON_R = crate::BitReader<PLLON>;
impl PLLON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLON {
        match self.bits {
            false => PLLON::Disabled,
            true => PLLON::Enabled,
        }
    }
    ///PLL powered off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLON::Disabled
    }
    ///PLL enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLON::Enabled
    }
}
///Field `PLLON` writer - PLL enable
pub type PLLON_W<'a, REG> = crate::BitWriter<'a, REG, PLLON>;
impl<'a, REG> PLLON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL powered off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLON::Disabled)
    }
    ///PLL enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PLLON::Enabled)
    }
}
/**PLL clock ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDY {
    ///0: PLL unlocked
    Unlocked = 0,
    ///1: PLL locked
    Locked = 1,
}
impl From<PLLRDY> for bool {
    #[inline(always)]
    fn from(variant: PLLRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLRDY` reader - PLL clock ready flag
pub type PLLRDY_R = crate::BitReader<PLLRDY>;
impl PLLRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLRDY {
        match self.bits {
            false => PLLRDY::Unlocked,
            true => PLLRDY::Locked,
        }
    }
    ///PLL unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLRDY::Unlocked
    }
    ///PLL locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLRDY::Locked
    }
}
///Field `PLLRDY` writer - PLL clock ready flag
pub type PLLRDY_W<'a, REG> = crate::BitWriter<'a, REG, PLLRDY>;
impl<'a, REG> PLLRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL unlocked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(PLLRDY::Unlocked)
    }
    ///PLL locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(PLLRDY::Locked)
    }
}
impl R {
    ///Bit 8 - HSI16 clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSI16 always enable for peripheral kernels
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI16 clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:13 - HSI16 clock division factor
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new(((self.bits >> 11) & 7) as u8)
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
    ///Bit 19 - Clock security system enable
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("hsion", &self.hsion())
            .field("hsikeron", &self.hsikeron())
            .field("hsirdy", &self.hsirdy())
            .field("hsidiv", &self.hsidiv())
            .field("hseon", &self.hseon())
            .field("hserdy", &self.hserdy())
            .field("hsebyp", &self.hsebyp())
            .field("csson", &self.csson())
            .field("pllon", &self.pllon())
            .field("pllrdy", &self.pllrdy())
            .finish()
    }
}
impl W {
    ///Bit 8 - HSI16 clock enable
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<'_, CRrs> {
        HSION_W::new(self, 8)
    }
    ///Bit 9 - HSI16 always enable for peripheral kernels
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W<'_, CRrs> {
        HSIKERON_W::new(self, 9)
    }
    ///Bit 10 - HSI16 clock ready flag
    #[inline(always)]
    pub fn hsirdy(&mut self) -> HSIRDY_W<'_, CRrs> {
        HSIRDY_W::new(self, 10)
    }
    ///Bits 11:13 - HSI16 clock division factor
    #[inline(always)]
    pub fn hsidiv(&mut self) -> HSIDIV_W<'_, CRrs> {
        HSIDIV_W::new(self, 11)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<'_, CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&mut self) -> HSERDY_W<'_, CRrs> {
        HSERDY_W::new(self, 17)
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
    ///Bit 24 - PLL enable
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W<'_, CRrs> {
        PLLON_W::new(self, 24)
    }
    ///Bit 25 - PLL clock ready flag
    #[inline(always)]
    pub fn pllrdy(&mut self) -> PLLRDY_W<'_, CRrs> {
        PLLRDY_W::new(self, 25)
    }
}
/**Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#RCC:CR)*/
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
