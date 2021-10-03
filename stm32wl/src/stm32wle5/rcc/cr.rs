#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Main PLL clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLRDY_A {
    #[doc = "0: PLL unlocked"]
    UNLOCKED = 0,
    #[doc = "1: PLL Locked"]
    LOCKED = 1,
}
impl From<PLLRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRDY` reader - Main PLL clock ready flag"]
pub struct PLLRDY_R(crate::FieldReader<bool, PLLRDY_A>);
impl PLLRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLRDY_R(crate::FieldReader::new(bits))
    }
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
        **self == PLLRDY_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == PLLRDY_A::LOCKED
    }
}
impl core::ops::Deref for PLLRDY_R {
    type Target = crate::FieldReader<bool, PLLRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Main PLL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLON_A {
    #[doc = "0: Main PLL Off"]
    OFF = 0,
    #[doc = "1: Main PLL On"]
    ON = 1,
}
impl From<PLLON_A> for bool {
    #[inline(always)]
    fn from(variant: PLLON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLON` reader - Main PLL enable"]
pub struct PLLON_R(crate::FieldReader<bool, PLLON_A>);
impl PLLON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLON_A {
        match self.bits {
            false => PLLON_A::OFF,
            true => PLLON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == PLLON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == PLLON_A::ON
    }
}
impl core::ops::Deref for PLLON_R {
    type Target = crate::FieldReader<bool, PLLON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLON` writer - Main PLL enable"]
pub struct PLLON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Main PLL Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLLON_A::OFF)
    }
    #[doc = "Main PLL On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLLON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEBYPPWR_A {
    #[doc = "0: PB0 selected"]
    PB0 = 0,
    #[doc = "1: VDDTCXO selected"]
    VDDTCXO = 1,
}
impl From<HSEBYPPWR_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYPPWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEBYPPWR` reader - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
pub struct HSEBYPPWR_R(crate::FieldReader<bool, HSEBYPPWR_A>);
impl HSEBYPPWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEBYPPWR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEBYPPWR_A {
        match self.bits {
            false => HSEBYPPWR_A::PB0,
            true => HSEBYPPWR_A::VDDTCXO,
        }
    }
    #[doc = "Checks if the value of the field is `PB0`"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        **self == HSEBYPPWR_A::PB0
    }
    #[doc = "Checks if the value of the field is `VDDTCXO`"]
    #[inline(always)]
    pub fn is_vddtcxo(&self) -> bool {
        **self == HSEBYPPWR_A::VDDTCXO
    }
}
impl core::ops::Deref for HSEBYPPWR_R {
    type Target = crate::FieldReader<bool, HSEBYPPWR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEBYPPWR` writer - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
pub struct HSEBYPPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEBYPPWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEBYPPWR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PB0 selected"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(HSEBYPPWR_A::PB0)
    }
    #[doc = "VDDTCXO selected"]
    #[inline(always)]
    pub fn vddtcxo(self) -> &'a mut W {
        self.variant(HSEBYPPWR_A::VDDTCXO)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "HSE32 sysclk prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEPRE_A {
    #[doc = "0: SYSCLK not divided (HSE32)"]
    DIV1 = 0,
    #[doc = "1: SYSCLK divided by two (HSE32/2)"]
    DIV2 = 1,
}
impl From<HSEPRE_A> for bool {
    #[inline(always)]
    fn from(variant: HSEPRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEPRE` reader - HSE32 sysclk prescaler"]
pub struct HSEPRE_R(crate::FieldReader<bool, HSEPRE_A>);
impl HSEPRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEPRE_A {
        match self.bits {
            false => HSEPRE_A::DIV1,
            true => HSEPRE_A::DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == HSEPRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == HSEPRE_A::DIV2
    }
}
impl core::ops::Deref for HSEPRE_R {
    type Target = crate::FieldReader<bool, HSEPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEPRE` writer - HSE32 sysclk prescaler"]
pub struct HSEPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEPRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SYSCLK not divided (HSE32)"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HSEPRE_A::DIV1)
    }
    #[doc = "SYSCLK divided by two (HSE32/2)"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HSEPRE_A::DIV2)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "HSE32 Clock security system enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSON_A {
    #[doc = "0: HSE32 CSS off"]
    DISABLED = 0,
    #[doc = "1: HSE32 CSS on if the HSE32 oscillator is stable and off if not"]
    ENABLED = 1,
}
impl From<CSSON_A> for bool {
    #[inline(always)]
    fn from(variant: CSSON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSON` reader - HSE32 Clock security system enable"]
pub struct CSSON_R(crate::FieldReader<bool, CSSON_A>);
impl CSSON_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSON_A {
        match self.bits {
            false => CSSON_A::DISABLED,
            true => CSSON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CSSON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CSSON_A::ENABLED
    }
}
impl core::ops::Deref for CSSON_R {
    type Target = crate::FieldReader<bool, CSSON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSON` writer - HSE32 Clock security system enable"]
pub struct CSSON_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSSON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSE32 CSS off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSSON_A::DISABLED)
    }
    #[doc = "HSE32 CSS on if the HSE32 oscillator is stable and off if not"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSSON_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "HSE32 clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSERDY_A {
    #[doc = "0: HSE32 oscillator not ready"]
    NOTREADY = 0,
    #[doc = "1: HSE32 oscillator ready"]
    READY = 1,
}
impl From<HSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDY` reader - HSE32 clock ready flag"]
pub struct HSERDY_R(crate::FieldReader<bool, HSERDY_A>);
impl HSERDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSERDY_R(crate::FieldReader::new(bits))
    }
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
        **self == HSERDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == HSERDY_A::READY
    }
}
impl core::ops::Deref for HSERDY_R {
    type Target = crate::FieldReader<bool, HSERDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HSE32 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEON_A {
    #[doc = "0: HSE32 oscillator for CPU disabled"]
    DISABLED = 0,
    #[doc = "1: HSE32 oscillator for CPU enabled"]
    ENABLED = 1,
}
impl From<HSEON_A> for bool {
    #[inline(always)]
    fn from(variant: HSEON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEON` reader - HSE32 clock enable"]
pub struct HSEON_R(crate::FieldReader<bool, HSEON_A>);
impl HSEON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEON_A {
        match self.bits {
            false => HSEON_A::DISABLED,
            true => HSEON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == HSEON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == HSEON_A::ENABLED
    }
}
impl core::ops::Deref for HSEON_R {
    type Target = crate::FieldReader<bool, HSEON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEON` writer - HSE32 clock enable"]
pub struct HSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSE32 oscillator for CPU disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSEON_A::DISABLED)
    }
    #[doc = "HSE32 oscillator for CPU enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSEON_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "HSI16 kernel clock ready flag for peripherals requests.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIKERDY_A {
    #[doc = "0: HSI16 oscillator not ready"]
    NOTREADY = 0,
    #[doc = "1: HSI16 oscillator ready"]
    READY = 1,
}
impl From<HSIKERDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSIKERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIKERDY` reader - HSI16 kernel clock ready flag for peripherals requests."]
pub struct HSIKERDY_R(crate::FieldReader<bool, HSIKERDY_A>);
impl HSIKERDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIKERDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIKERDY_A {
        match self.bits {
            false => HSIKERDY_A::NOTREADY,
            true => HSIKERDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == HSIKERDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == HSIKERDY_A::READY
    }
}
impl core::ops::Deref for HSIKERDY_R {
    type Target = crate::FieldReader<bool, HSIKERDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HSI16 automatic start from Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIASFS_A {
    #[doc = "0: HSI16 not enabled by hardware when exiting Stop modes with MSI as wakeup clock"]
    DISABLED = 0,
    #[doc = "1: HSI16 enabled by hardware when exiting Stop mode with MSI as wakeup clock"]
    ENABLED = 1,
}
impl From<HSIASFS_A> for bool {
    #[inline(always)]
    fn from(variant: HSIASFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIASFS` reader - HSI16 automatic start from Stop"]
pub struct HSIASFS_R(crate::FieldReader<bool, HSIASFS_A>);
impl HSIASFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIASFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIASFS_A {
        match self.bits {
            false => HSIASFS_A::DISABLED,
            true => HSIASFS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == HSIASFS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == HSIASFS_A::ENABLED
    }
}
impl core::ops::Deref for HSIASFS_R {
    type Target = crate::FieldReader<bool, HSIASFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIASFS` writer - HSI16 automatic start from Stop"]
pub struct HSIASFS_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIASFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSIASFS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSI16 not enabled by hardware when exiting Stop modes with MSI as wakeup clock"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSIASFS_A::DISABLED)
    }
    #[doc = "HSI16 enabled by hardware when exiting Stop mode with MSI as wakeup clock"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSIASFS_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIRDY_A {
    #[doc = "0: HSI16 oscillator not ready"]
    NOTREADY = 0,
    #[doc = "1: HSI16 oscillator ready"]
    READY = 1,
}
impl From<HSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDY` reader - HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)"]
pub struct HSIRDY_R(crate::FieldReader<bool, HSIRDY_A>);
impl HSIRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDY_A {
        match self.bits {
            false => HSIRDY_A::NOTREADY,
            true => HSIRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == HSIRDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == HSIRDY_A::READY
    }
}
impl core::ops::Deref for HSIRDY_R {
    type Target = crate::FieldReader<bool, HSIRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HSI16 always enable for peripheral kernel clocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIKERON_A {
    #[doc = "0: No effect on HSI16 oscillator"]
    NOTFORCED = 0,
    #[doc = "1: HSI16 oscillator forced on even in Stop modes"]
    FORCED = 1,
}
impl From<HSIKERON_A> for bool {
    #[inline(always)]
    fn from(variant: HSIKERON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIKERON` reader - HSI16 always enable for peripheral kernel clocks."]
pub struct HSIKERON_R(crate::FieldReader<bool, HSIKERON_A>);
impl HSIKERON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIKERON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIKERON_A {
        match self.bits {
            false => HSIKERON_A::NOTFORCED,
            true => HSIKERON_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTFORCED`"]
    #[inline(always)]
    pub fn is_not_forced(&self) -> bool {
        **self == HSIKERON_A::NOTFORCED
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        **self == HSIKERON_A::FORCED
    }
}
impl core::ops::Deref for HSIKERON_R {
    type Target = crate::FieldReader<bool, HSIKERON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIKERON` writer - HSI16 always enable for peripheral kernel clocks."]
pub struct HSIKERON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIKERON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSIKERON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect on HSI16 oscillator"]
    #[inline(always)]
    pub fn not_forced(self) -> &'a mut W {
        self.variant(HSIKERON_A::NOTFORCED)
    }
    #[doc = "HSI16 oscillator forced on even in Stop modes"]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(HSIKERON_A::FORCED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "HSI16 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSION_A {
    #[doc = "0: HSI16 oscillator off"]
    DISABLED = 0,
    #[doc = "1: HSI16 oscillator on"]
    ENABLED = 1,
}
impl From<HSION_A> for bool {
    #[inline(always)]
    fn from(variant: HSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSION` reader - HSI16 clock enable"]
pub struct HSION_R(crate::FieldReader<bool, HSION_A>);
impl HSION_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSION_A {
        match self.bits {
            false => HSION_A::DISABLED,
            true => HSION_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == HSION_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == HSION_A::ENABLED
    }
}
impl core::ops::Deref for HSION_R {
    type Target = crate::FieldReader<bool, HSION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSION` writer - HSI16 clock enable"]
pub struct HSION_W<'a> {
    w: &'a mut W,
}
impl<'a> HSION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSI16 oscillator off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSION_A::DISABLED)
    }
    #[doc = "HSI16 oscillator on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSION_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "MSI clock ranges\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSIRANGE_A {
    #[doc = "0: range 0 around 100 kHz"]
    RANGE100K = 0,
    #[doc = "1: range 1 around 200 kHz"]
    RANGE200K = 1,
    #[doc = "2: range 2 around 400 kHz"]
    RANGE400K = 2,
    #[doc = "3: range 3 around 800 kHz"]
    RANGE800K = 3,
    #[doc = "4: range 4 around 1 MHz"]
    RANGE1M = 4,
    #[doc = "5: range 5 around 2 MHz"]
    RANGE2M = 5,
    #[doc = "6: range 6 around 4 MHz (reset value)"]
    RANGE4M = 6,
    #[doc = "7: range 7 around 8 MHz"]
    RANGE8M = 7,
    #[doc = "8: range 8 around 16 MHz"]
    RANGE16M = 8,
    #[doc = "9: range 9 around 24 MHz"]
    RANGE24M = 9,
    #[doc = "10: range 10 around 32 MHz"]
    RANGE32M = 10,
    #[doc = "11: range 11 around 48 MHz"]
    RANGE48M = 11,
}
impl From<MSIRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSIRANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSIRANGE` reader - MSI clock ranges"]
pub struct MSIRANGE_R(crate::FieldReader<u8, MSIRANGE_A>);
impl MSIRANGE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSIRANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSIRANGE_A> {
        match self.bits {
            0 => Some(MSIRANGE_A::RANGE100K),
            1 => Some(MSIRANGE_A::RANGE200K),
            2 => Some(MSIRANGE_A::RANGE400K),
            3 => Some(MSIRANGE_A::RANGE800K),
            4 => Some(MSIRANGE_A::RANGE1M),
            5 => Some(MSIRANGE_A::RANGE2M),
            6 => Some(MSIRANGE_A::RANGE4M),
            7 => Some(MSIRANGE_A::RANGE8M),
            8 => Some(MSIRANGE_A::RANGE16M),
            9 => Some(MSIRANGE_A::RANGE24M),
            10 => Some(MSIRANGE_A::RANGE32M),
            11 => Some(MSIRANGE_A::RANGE48M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RANGE100K`"]
    #[inline(always)]
    pub fn is_range100k(&self) -> bool {
        **self == MSIRANGE_A::RANGE100K
    }
    #[doc = "Checks if the value of the field is `RANGE200K`"]
    #[inline(always)]
    pub fn is_range200k(&self) -> bool {
        **self == MSIRANGE_A::RANGE200K
    }
    #[doc = "Checks if the value of the field is `RANGE400K`"]
    #[inline(always)]
    pub fn is_range400k(&self) -> bool {
        **self == MSIRANGE_A::RANGE400K
    }
    #[doc = "Checks if the value of the field is `RANGE800K`"]
    #[inline(always)]
    pub fn is_range800k(&self) -> bool {
        **self == MSIRANGE_A::RANGE800K
    }
    #[doc = "Checks if the value of the field is `RANGE1M`"]
    #[inline(always)]
    pub fn is_range1m(&self) -> bool {
        **self == MSIRANGE_A::RANGE1M
    }
    #[doc = "Checks if the value of the field is `RANGE2M`"]
    #[inline(always)]
    pub fn is_range2m(&self) -> bool {
        **self == MSIRANGE_A::RANGE2M
    }
    #[doc = "Checks if the value of the field is `RANGE4M`"]
    #[inline(always)]
    pub fn is_range4m(&self) -> bool {
        **self == MSIRANGE_A::RANGE4M
    }
    #[doc = "Checks if the value of the field is `RANGE8M`"]
    #[inline(always)]
    pub fn is_range8m(&self) -> bool {
        **self == MSIRANGE_A::RANGE8M
    }
    #[doc = "Checks if the value of the field is `RANGE16M`"]
    #[inline(always)]
    pub fn is_range16m(&self) -> bool {
        **self == MSIRANGE_A::RANGE16M
    }
    #[doc = "Checks if the value of the field is `RANGE24M`"]
    #[inline(always)]
    pub fn is_range24m(&self) -> bool {
        **self == MSIRANGE_A::RANGE24M
    }
    #[doc = "Checks if the value of the field is `RANGE32M`"]
    #[inline(always)]
    pub fn is_range32m(&self) -> bool {
        **self == MSIRANGE_A::RANGE32M
    }
    #[doc = "Checks if the value of the field is `RANGE48M`"]
    #[inline(always)]
    pub fn is_range48m(&self) -> bool {
        **self == MSIRANGE_A::RANGE48M
    }
}
impl core::ops::Deref for MSIRANGE_R {
    type Target = crate::FieldReader<u8, MSIRANGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSIRANGE` writer - MSI clock ranges"]
pub struct MSIRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIRANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSIRANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "range 0 around 100 kHz"]
    #[inline(always)]
    pub fn range100k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE100K)
    }
    #[doc = "range 1 around 200 kHz"]
    #[inline(always)]
    pub fn range200k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE200K)
    }
    #[doc = "range 2 around 400 kHz"]
    #[inline(always)]
    pub fn range400k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE400K)
    }
    #[doc = "range 3 around 800 kHz"]
    #[inline(always)]
    pub fn range800k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE800K)
    }
    #[doc = "range 4 around 1 MHz"]
    #[inline(always)]
    pub fn range1m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE1M)
    }
    #[doc = "range 5 around 2 MHz"]
    #[inline(always)]
    pub fn range2m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE2M)
    }
    #[doc = "range 6 around 4 MHz (reset value)"]
    #[inline(always)]
    pub fn range4m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE4M)
    }
    #[doc = "range 7 around 8 MHz"]
    #[inline(always)]
    pub fn range8m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE8M)
    }
    #[doc = "range 8 around 16 MHz"]
    #[inline(always)]
    pub fn range16m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE16M)
    }
    #[doc = "range 9 around 24 MHz"]
    #[inline(always)]
    pub fn range24m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE24M)
    }
    #[doc = "range 10 around 32 MHz"]
    #[inline(always)]
    pub fn range32m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE32M)
    }
    #[doc = "range 11 around 48 MHz"]
    #[inline(always)]
    pub fn range48m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE48M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "MSI range control selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSIRGSEL_A {
    #[doc = "0: MSI frequency range defined by MSISRANGE\\[3:0\\]
in the RCC_CSR register"]
    CSR = 0,
    #[doc = "1: MSI frequency range defined by MSIRANGE\\[3:0\\]
in the RCC_CR register"]
    CR = 1,
}
impl From<MSIRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MSIRGSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIRGSEL` reader - MSI range control selection"]
pub struct MSIRGSEL_R(crate::FieldReader<bool, MSIRGSEL_A>);
impl MSIRGSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSIRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSIRGSEL_A {
        match self.bits {
            false => MSIRGSEL_A::CSR,
            true => MSIRGSEL_A::CR,
        }
    }
    #[doc = "Checks if the value of the field is `CSR`"]
    #[inline(always)]
    pub fn is_csr(&self) -> bool {
        **self == MSIRGSEL_A::CSR
    }
    #[doc = "Checks if the value of the field is `CR`"]
    #[inline(always)]
    pub fn is_cr(&self) -> bool {
        **self == MSIRGSEL_A::CR
    }
}
impl core::ops::Deref for MSIRGSEL_R {
    type Target = crate::FieldReader<bool, MSIRGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSIRGSEL` writer - MSI range control selection"]
pub struct MSIRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSIRGSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MSI frequency range defined by MSISRANGE\\[3:0\\]
in the RCC_CSR register"]
    #[inline(always)]
    pub fn csr(self) -> &'a mut W {
        self.variant(MSIRGSEL_A::CSR)
    }
    #[doc = "MSI frequency range defined by MSIRANGE\\[3:0\\]
in the RCC_CR register"]
    #[inline(always)]
    pub fn cr(self) -> &'a mut W {
        self.variant(MSIRGSEL_A::CR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "MSI clock PLL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSIPLLEN_A {
    #[doc = "0: MSI PLL Off"]
    OFF = 0,
    #[doc = "1: MSI PLL On"]
    ON = 1,
}
impl From<MSIPLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSIPLLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIPLLEN` reader - MSI clock PLL enable"]
pub struct MSIPLLEN_R(crate::FieldReader<bool, MSIPLLEN_A>);
impl MSIPLLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSIPLLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSIPLLEN_A {
        match self.bits {
            false => MSIPLLEN_A::OFF,
            true => MSIPLLEN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == MSIPLLEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == MSIPLLEN_A::ON
    }
}
impl core::ops::Deref for MSIPLLEN_R {
    type Target = crate::FieldReader<bool, MSIPLLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSIPLLEN` writer - MSI clock PLL enable"]
pub struct MSIPLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIPLLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSIPLLEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MSI PLL Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MSIPLLEN_A::OFF)
    }
    #[doc = "MSI PLL On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(MSIPLLEN_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSIRDY_A {
    #[doc = "0: MSI oscillator not ready"]
    NOTREADY = 0,
    #[doc = "1: MSI oscillator ready"]
    READY = 1,
}
impl From<MSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIRDY` reader - MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)"]
pub struct MSIRDY_R(crate::FieldReader<bool, MSIRDY_A>);
impl MSIRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSIRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSIRDY_A {
        match self.bits {
            false => MSIRDY_A::NOTREADY,
            true => MSIRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == MSIRDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == MSIRDY_A::READY
    }
}
impl core::ops::Deref for MSIRDY_R {
    type Target = crate::FieldReader<bool, MSIRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "MSI clock enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSION_A {
    #[doc = "0: MSI oscillator off"]
    DISABLED = 0,
    #[doc = "1: MSI oscillator on"]
    ENABLED = 1,
}
impl From<MSION_A> for bool {
    #[inline(always)]
    fn from(variant: MSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSION` reader - MSI clock enable"]
pub struct MSION_R(crate::FieldReader<bool, MSION_A>);
impl MSION_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSION_A {
        match self.bits {
            false => MSION_A::DISABLED,
            true => MSION_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MSION_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MSION_A::ENABLED
    }
}
impl core::ops::Deref for MSION_R {
    type Target = crate::FieldReader<bool, MSION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSION` writer - MSI clock enable"]
pub struct MSION_W<'a> {
    w: &'a mut W,
}
impl<'a> MSION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MSI oscillator off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSION_A::DISABLED)
    }
    #[doc = "MSI oscillator on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSION_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - Main PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
    #[inline(always)]
    pub fn hsebyppwr(&self) -> HSEBYPPWR_R {
        HSEBYPPWR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - HSE32 sysclk prescaler"]
    #[inline(always)]
    pub fn hsepre(&self) -> HSEPRE_R {
        HSEPRE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HSE32 Clock security system enable"]
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HSE32 clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HSE32 clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HSI16 kernel clock ready flag for peripherals requests."]
    #[inline(always)]
    pub fn hsikerdy(&self) -> HSIKERDY_R {
        HSIKERDY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSI16 automatic start from Stop"]
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernel clocks."]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HSI16 clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - MSI range control selection"]
    #[inline(always)]
    pub fn msirgsel(&self) -> MSIRGSEL_R {
        MSIRGSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)"]
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W {
        PLLON_W { w: self }
    }
    #[doc = "Bit 21 - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO."]
    #[inline(always)]
    pub fn hsebyppwr(&mut self) -> HSEBYPPWR_W {
        HSEBYPPWR_W { w: self }
    }
    #[doc = "Bit 20 - HSE32 sysclk prescaler"]
    #[inline(always)]
    pub fn hsepre(&mut self) -> HSEPRE_W {
        HSEPRE_W { w: self }
    }
    #[doc = "Bit 19 - HSE32 Clock security system enable"]
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W {
        CSSON_W { w: self }
    }
    #[doc = "Bit 16 - HSE32 clock enable"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W {
        HSEON_W { w: self }
    }
    #[doc = "Bit 11 - HSI16 automatic start from Stop"]
    #[inline(always)]
    pub fn hsiasfs(&mut self) -> HSIASFS_W {
        HSIASFS_W { w: self }
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernel clocks."]
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W {
        HSIKERON_W { w: self }
    }
    #[doc = "Bit 8 - HSI16 clock enable"]
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W {
        HSION_W { w: self }
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&mut self) -> MSIRANGE_W {
        MSIRANGE_W { w: self }
    }
    #[doc = "Bit 3 - MSI range control selection"]
    #[inline(always)]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W {
        MSIRGSEL_W { w: self }
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    pub fn msipllen(&mut self) -> MSIPLLEN_W {
        MSIPLLEN_W { w: self }
    }
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&mut self) -> MSION_W {
        MSION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x61"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x61
    }
}
