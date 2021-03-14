#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0x0300"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0300
    }
}
#[doc = "PLL clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLRDY_A {
    #[doc = "0: PLL unlocked"]
    UNLOCKED = 0,
    #[doc = "1: PLL locked"]
    LOCKED = 1,
}
impl From<PLLRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLRDY`"]
pub type PLLRDY_R = crate::R<bool, PLLRDY_A>;
impl PLLRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLRDY_A {
        match self.bits {
            false => PLLRDY_A::UNLOCKED,
            true => PLLRDY_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLRDY_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLRDY_A::LOCKED
    }
}
#[doc = "PLL enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLON_A {
    #[doc = "0: Clock disabled"]
    DISABLED = 0,
    #[doc = "1: Clock enabled"]
    ENABLED = 1,
}
impl From<PLLON_A> for bool {
    #[inline(always)]
    fn from(variant: PLLON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLON`"]
pub type PLLON_R = crate::R<bool, PLLON_A>;
impl PLLON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLON_A {
        match self.bits {
            false => PLLON_A::DISABLED,
            true => PLLON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLON_A::ENABLED
    }
}
#[doc = "Write proxy for field `PLLON`"]
pub struct PLLON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLON_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLON_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "TC/LCD prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCPRE_A {
    #[doc = "0: HSE divided by 2"]
    DIV2 = 0,
    #[doc = "1: HSE divided by 4"]
    DIV4 = 1,
    #[doc = "2: HSE divided by 8"]
    DIV8 = 2,
    #[doc = "3: HSE divided by 16"]
    DIV16 = 3,
}
impl From<RTCPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCPRE`"]
pub type RTCPRE_R = crate::R<u8, RTCPRE_A>;
impl RTCPRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCPRE_A {
        match self.bits {
            0 => RTCPRE_A::DIV2,
            1 => RTCPRE_A::DIV4,
            2 => RTCPRE_A::DIV8,
            3 => RTCPRE_A::DIV16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RTCPRE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == RTCPRE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == RTCPRE_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == RTCPRE_A::DIV16
    }
}
#[doc = "Write proxy for field `RTCPRE`"]
pub struct RTCPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCPRE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "HSE divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RTCPRE_A::DIV2)
    }
    #[doc = "HSE divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(RTCPRE_A::DIV4)
    }
    #[doc = "HSE divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(RTCPRE_A::DIV8)
    }
    #[doc = "HSE divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(RTCPRE_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Clock security system on HSE enable bit"]
pub type CSSHSEON_A = PLLON_A;
#[doc = "Reader of field `CSSHSEON`"]
pub type CSSHSEON_R = crate::R<bool, PLLON_A>;
#[doc = "Write proxy for field `CSSHSEON`"]
pub struct CSSHSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSHSEON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSSHSEON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLON_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLON_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "HSE clock bypass bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEBYP_A {
    #[doc = "0: HSE oscillator not bypassed"]
    NOTBYPASSED = 0,
    #[doc = "1: HSE oscillator bypassed"]
    BYPASSED = 1,
}
impl From<HSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSEBYP`"]
pub type HSEBYP_R = crate::R<bool, HSEBYP_A>;
impl HSEBYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEBYP_A {
        match self.bits {
            false => HSEBYP_A::NOTBYPASSED,
            true => HSEBYP_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBYPASSED`"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP_A::NOTBYPASSED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP_A::BYPASSED
    }
}
#[doc = "Write proxy for field `HSEBYP`"]
pub struct HSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEBYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSE oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::NOTBYPASSED)
    }
    #[doc = "HSE oscillator bypassed"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::BYPASSED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "HSE clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSERDY_A {
    #[doc = "0: Oscillator is not stable"]
    NOTREADY = 0,
    #[doc = "1: Oscillator is stable"]
    READY = 1,
}
impl From<HSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSERDY`"]
pub type HSERDY_R = crate::R<bool, HSERDY_A>;
impl HSERDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSERDY_A {
        match self.bits {
            false => HSERDY_A::NOTREADY,
            true => HSERDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSERDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSERDY_A::READY
    }
}
#[doc = "HSE clock enable bit"]
pub type HSEON_A = PLLON_A;
#[doc = "Reader of field `HSEON`"]
pub type HSEON_R = crate::R<bool, PLLON_A>;
#[doc = "Write proxy for field `HSEON`"]
pub struct HSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLON_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLON_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "MSI clock ready flag"]
pub type MSIRDY_A = HSERDY_A;
#[doc = "Reader of field `MSIRDY`"]
pub type MSIRDY_R = crate::R<bool, HSERDY_A>;
#[doc = "MSI clock enable bit"]
pub type MSION_A = PLLON_A;
#[doc = "Reader of field `MSION`"]
pub type MSION_R = crate::R<bool, PLLON_A>;
#[doc = "Write proxy for field `MSION`"]
pub struct MSION_W<'a> {
    w: &'a mut W,
}
impl<'a> MSION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLON_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLON_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "HSI16DIVF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI16DIVF_A {
    #[doc = "0: 16 MHz HSI clock not divided"]
    NOTDIVIDED = 0,
    #[doc = "1: 16 MHz HSI clock divided by 4"]
    DIV4 = 1,
}
impl From<HSI16DIVF_A> for bool {
    #[inline(always)]
    fn from(variant: HSI16DIVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSI16DIVF`"]
pub type HSI16DIVF_R = crate::R<bool, HSI16DIVF_A>;
impl HSI16DIVF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI16DIVF_A {
        match self.bits {
            false => HSI16DIVF_A::NOTDIVIDED,
            true => HSI16DIVF_A::DIV4,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDIVIDED`"]
    #[inline(always)]
    pub fn is_not_divided(&self) -> bool {
        *self == HSI16DIVF_A::NOTDIVIDED
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSI16DIVF_A::DIV4
    }
}
#[doc = "HSI16DIVEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI16DIVEN_A {
    #[doc = "0: no 16 MHz HSI division requested"]
    NOTDIVIDED = 0,
    #[doc = "1: 16 MHz HSI division by 4 requested"]
    DIV4 = 1,
}
impl From<HSI16DIVEN_A> for bool {
    #[inline(always)]
    fn from(variant: HSI16DIVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSI16DIVEN`"]
pub type HSI16DIVEN_R = crate::R<bool, HSI16DIVEN_A>;
impl HSI16DIVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI16DIVEN_A {
        match self.bits {
            false => HSI16DIVEN_A::NOTDIVIDED,
            true => HSI16DIVEN_A::DIV4,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDIVIDED`"]
    #[inline(always)]
    pub fn is_not_divided(&self) -> bool {
        *self == HSI16DIVEN_A::NOTDIVIDED
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSI16DIVEN_A::DIV4
    }
}
#[doc = "Write proxy for field `HSI16DIVEN`"]
pub struct HSI16DIVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI16DIVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSI16DIVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no 16 MHz HSI division requested"]
    #[inline(always)]
    pub fn not_divided(self) -> &'a mut W {
        self.variant(HSI16DIVEN_A::NOTDIVIDED)
    }
    #[doc = "16 MHz HSI division by 4 requested"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HSI16DIVEN_A::DIV4)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Internal high-speed clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI16RDYF_A {
    #[doc = "0: HSI 16 MHz oscillator not ready"]
    NOTREADY = 0,
    #[doc = "1: HSI 16 MHz oscillator ready"]
    READY = 1,
}
impl From<HSI16RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSI16RDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSI16RDYF`"]
pub type HSI16RDYF_R = crate::R<bool, HSI16RDYF_A>;
impl HSI16RDYF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI16RDYF_A {
        match self.bits {
            false => HSI16RDYF_A::NOTREADY,
            true => HSI16RDYF_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSI16RDYF_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSI16RDYF_A::READY
    }
}
#[doc = "Write proxy for field `HSI16RDYF`"]
pub struct HSI16RDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI16RDYF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSI16RDYF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSI 16 MHz oscillator not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(HSI16RDYF_A::NOTREADY)
    }
    #[doc = "HSI 16 MHz oscillator ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(HSI16RDYF_A::READY)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "High-speed internal clock enable bit for some IP kernels"]
pub type HSI16KERON_A = PLLON_A;
#[doc = "Reader of field `HSI16KERON`"]
pub type HSI16KERON_R = crate::R<bool, PLLON_A>;
#[doc = "16 MHz high-speed internal clock enable"]
pub type HSI16ON_A = PLLON_A;
#[doc = "Reader of field `HSI16ON`"]
pub type HSI16ON_R = crate::R<bool, PLLON_A>;
#[doc = "Write proxy for field `HSI16ON`"]
pub struct HSI16ON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI16ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSI16ON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLON_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLON_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "16 MHz high-speed internal clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI16OUTEN_A {
    #[doc = "0: HSI output clock disabled"]
    DISABLED = 0,
    #[doc = "1: HSI output clock enabled"]
    ENABLED = 1,
}
impl From<HSI16OUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: HSI16OUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSI16OUTEN`"]
pub type HSI16OUTEN_R = crate::R<bool, HSI16OUTEN_A>;
impl HSI16OUTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI16OUTEN_A {
        match self.bits {
            false => HSI16OUTEN_A::DISABLED,
            true => HSI16OUTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI16OUTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI16OUTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `HSI16OUTEN`"]
pub struct HSI16OUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI16OUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSI16OUTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSI output clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSI16OUTEN_A::DISABLED)
    }
    #[doc = "HSI output clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSI16OUTEN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PLL enable bit"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - TC/LCD prescaler"]
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Clock security system on HSE enable bit"]
    #[inline(always)]
    pub fn csshseon(&self) -> CSSHSEON_R {
        CSSHSEON_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HSE clock bypass bit"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable bit"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MSI clock ready flag"]
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MSI clock enable bit"]
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HSI16DIVF"]
    #[inline(always)]
    pub fn hsi16divf(&self) -> HSI16DIVF_R {
        HSI16DIVF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSI16DIVEN"]
    #[inline(always)]
    pub fn hsi16diven(&self) -> HSI16DIVEN_R {
        HSI16DIVEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Internal high-speed clock ready flag"]
    #[inline(always)]
    pub fn hsi16rdyf(&self) -> HSI16RDYF_R {
        HSI16RDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - High-speed internal clock enable bit for some IP kernels"]
    #[inline(always)]
    pub fn hsi16keron(&self) -> HSI16KERON_R {
        HSI16KERON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 16 MHz high-speed internal clock enable"]
    #[inline(always)]
    pub fn hsi16on(&self) -> HSI16ON_R {
        HSI16ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - 16 MHz high-speed internal clock output enable"]
    #[inline(always)]
    pub fn hsi16outen(&self) -> HSI16OUTEN_R {
        HSI16OUTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - PLL enable bit"]
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W {
        PLLON_W { w: self }
    }
    #[doc = "Bits 20:21 - TC/LCD prescaler"]
    #[inline(always)]
    pub fn rtcpre(&mut self) -> RTCPRE_W {
        RTCPRE_W { w: self }
    }
    #[doc = "Bit 19 - Clock security system on HSE enable bit"]
    #[inline(always)]
    pub fn csshseon(&mut self) -> CSSHSEON_W {
        CSSHSEON_W { w: self }
    }
    #[doc = "Bit 18 - HSE clock bypass bit"]
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W {
        HSEBYP_W { w: self }
    }
    #[doc = "Bit 16 - HSE clock enable bit"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W {
        HSEON_W { w: self }
    }
    #[doc = "Bit 8 - MSI clock enable bit"]
    #[inline(always)]
    pub fn msion(&mut self) -> MSION_W {
        MSION_W { w: self }
    }
    #[doc = "Bit 3 - HSI16DIVEN"]
    #[inline(always)]
    pub fn hsi16diven(&mut self) -> HSI16DIVEN_W {
        HSI16DIVEN_W { w: self }
    }
    #[doc = "Bit 2 - Internal high-speed clock ready flag"]
    #[inline(always)]
    pub fn hsi16rdyf(&mut self) -> HSI16RDYF_W {
        HSI16RDYF_W { w: self }
    }
    #[doc = "Bit 0 - 16 MHz high-speed internal clock enable"]
    #[inline(always)]
    pub fn hsi16on(&mut self) -> HSI16ON_W {
        HSI16ON_W { w: self }
    }
    #[doc = "Bit 5 - 16 MHz high-speed internal clock output enable"]
    #[inline(always)]
    pub fn hsi16outen(&mut self) -> HSI16OUTEN_W {
        HSI16OUTEN_W { w: self }
    }
}
