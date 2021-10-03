#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Temperature sensor and VREFINT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSVREFE_A {
    #[doc = "0: Temperature sensor and V_REFINT channel disabled"]
    DISABLED = 0,
    #[doc = "1: Temperature sensor and V_REFINT channel enabled"]
    ENABLED = 1,
}
impl From<TSVREFE_A> for bool {
    #[inline(always)]
    fn from(variant: TSVREFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSVREFE` reader - Temperature sensor and VREFINT enable"]
pub struct TSVREFE_R(crate::FieldReader<bool, TSVREFE_A>);
impl TSVREFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSVREFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSVREFE_A {
        match self.bits {
            false => TSVREFE_A::DISABLED,
            true => TSVREFE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TSVREFE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TSVREFE_A::ENABLED
    }
}
impl core::ops::Deref for TSVREFE_R {
    type Target = crate::FieldReader<bool, TSVREFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSVREFE` writer - Temperature sensor and VREFINT enable"]
pub struct TSVREFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSVREFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSVREFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Temperature sensor and V_REFINT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSVREFE_A::DISABLED)
    }
    #[doc = "Temperature sensor and V_REFINT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSVREFE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Start conversion of regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSTART_A {
    #[doc = "0: Reset state"]
    STARTED = 0,
    #[doc = "1: Starting conversion of regular channels"]
    NOTSTARTED = 1,
}
impl From<SWSTART_A> for bool {
    #[inline(always)]
    fn from(variant: SWSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWSTART` reader - Start conversion of regular channels"]
pub struct SWSTART_R(crate::FieldReader<bool, SWSTART_A>);
impl SWSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWSTART_A {
        match self.bits {
            false => SWSTART_A::STARTED,
            true => SWSTART_A::NOTSTARTED,
        }
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        **self == SWSTART_A::STARTED
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        **self == SWSTART_A::NOTSTARTED
    }
}
impl core::ops::Deref for SWSTART_R {
    type Target = crate::FieldReader<bool, SWSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Start conversion of regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSTART_AW {
    #[doc = "1: Start conversion of regular channels"]
    START = 1,
}
impl From<SWSTART_AW> for bool {
    #[inline(always)]
    fn from(variant: SWSTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWSTART` writer - Start conversion of regular channels"]
pub struct SWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SWSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWSTART_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Start conversion of regular channels"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SWSTART_AW::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Start conversion of injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSWSTART_A {
    #[doc = "0: Reset state"]
    STARTED = 0,
    #[doc = "1: Starting conversion of injected channels"]
    NOTSTARTED = 1,
}
impl From<JSWSTART_A> for bool {
    #[inline(always)]
    fn from(variant: JSWSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSWSTART` reader - Start conversion of injected channels"]
pub struct JSWSTART_R(crate::FieldReader<bool, JSWSTART_A>);
impl JSWSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        JSWSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSWSTART_A {
        match self.bits {
            false => JSWSTART_A::STARTED,
            true => JSWSTART_A::NOTSTARTED,
        }
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        **self == JSWSTART_A::STARTED
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        **self == JSWSTART_A::NOTSTARTED
    }
}
impl core::ops::Deref for JSWSTART_R {
    type Target = crate::FieldReader<bool, JSWSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Start conversion of injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSWSTART_AW {
    #[doc = "1: Start conversion of injected channels"]
    START = 1,
}
impl From<JSWSTART_AW> for bool {
    #[inline(always)]
    fn from(variant: JSWSTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSWSTART` writer - Start conversion of injected channels"]
pub struct JSWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> JSWSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JSWSTART_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Start conversion of injected channels"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(JSWSTART_AW::START)
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
#[doc = "External trigger conversion mode for regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTTRIG_A {
    #[doc = "0: Conversion on external event disabled"]
    DISABLED = 0,
    #[doc = "1: Conversion on external event enabled"]
    ENABLED = 1,
}
impl From<EXTTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTTRIG` reader - External trigger conversion mode for regular channels"]
pub struct EXTTRIG_R(crate::FieldReader<bool, EXTTRIG_A>);
impl EXTTRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTTRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTTRIG_A {
        match self.bits {
            false => EXTTRIG_A::DISABLED,
            true => EXTTRIG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EXTTRIG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EXTTRIG_A::ENABLED
    }
}
impl core::ops::Deref for EXTTRIG_R {
    type Target = crate::FieldReader<bool, EXTTRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTTRIG` writer - External trigger conversion mode for regular channels"]
pub struct EXTTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTTRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTTRIG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTTRIG_A::DISABLED)
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTTRIG_A::ENABLED)
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
#[doc = "External event select for regular group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTSEL_A {
    #[doc = "0: Timer 1 CC1 event"]
    TIM1CC1 = 0,
    #[doc = "1: Timer 1 CC2 event"]
    TIM1CC2 = 1,
    #[doc = "2: Timer 1 CC3 event"]
    TIM1CC3 = 2,
    #[doc = "3: Timer 2 CC2 event"]
    TIM2CC2 = 3,
    #[doc = "4: Timer 3 TRGO event"]
    TIM3TRGO = 4,
    #[doc = "5: Timer 4 CC4 event"]
    TIM4CC4 = 5,
    #[doc = "6: EXTI line 11/TIM8_TRGO event (TIM8_TRGO is available only in high-density and XL-density devices)"]
    EXTI11 = 6,
    #[doc = "7: SWSTART"]
    SWSTART = 7,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTSEL` reader - External event select for regular group"]
pub struct EXTSEL_R(crate::FieldReader<u8, EXTSEL_A>);
impl EXTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSEL_A {
        match self.bits {
            0 => EXTSEL_A::TIM1CC1,
            1 => EXTSEL_A::TIM1CC2,
            2 => EXTSEL_A::TIM1CC3,
            3 => EXTSEL_A::TIM2CC2,
            4 => EXTSEL_A::TIM3TRGO,
            5 => EXTSEL_A::TIM4CC4,
            6 => EXTSEL_A::EXTI11,
            7 => EXTSEL_A::SWSTART,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIM1CC1`"]
    #[inline(always)]
    pub fn is_tim1cc1(&self) -> bool {
        **self == EXTSEL_A::TIM1CC1
    }
    #[doc = "Checks if the value of the field is `TIM1CC2`"]
    #[inline(always)]
    pub fn is_tim1cc2(&self) -> bool {
        **self == EXTSEL_A::TIM1CC2
    }
    #[doc = "Checks if the value of the field is `TIM1CC3`"]
    #[inline(always)]
    pub fn is_tim1cc3(&self) -> bool {
        **self == EXTSEL_A::TIM1CC3
    }
    #[doc = "Checks if the value of the field is `TIM2CC2`"]
    #[inline(always)]
    pub fn is_tim2cc2(&self) -> bool {
        **self == EXTSEL_A::TIM2CC2
    }
    #[doc = "Checks if the value of the field is `TIM3TRGO`"]
    #[inline(always)]
    pub fn is_tim3trgo(&self) -> bool {
        **self == EXTSEL_A::TIM3TRGO
    }
    #[doc = "Checks if the value of the field is `TIM4CC4`"]
    #[inline(always)]
    pub fn is_tim4cc4(&self) -> bool {
        **self == EXTSEL_A::TIM4CC4
    }
    #[doc = "Checks if the value of the field is `EXTI11`"]
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        **self == EXTSEL_A::EXTI11
    }
    #[doc = "Checks if the value of the field is `SWSTART`"]
    #[inline(always)]
    pub fn is_swstart(&self) -> bool {
        **self == EXTSEL_A::SWSTART
    }
}
impl core::ops::Deref for EXTSEL_R {
    type Target = crate::FieldReader<u8, EXTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSEL` writer - External event select for regular group"]
pub struct EXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer 1 CC1 event"]
    #[inline(always)]
    pub fn tim1cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1CC1)
    }
    #[doc = "Timer 1 CC2 event"]
    #[inline(always)]
    pub fn tim1cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1CC2)
    }
    #[doc = "Timer 1 CC3 event"]
    #[inline(always)]
    pub fn tim1cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1CC3)
    }
    #[doc = "Timer 2 CC2 event"]
    #[inline(always)]
    pub fn tim2cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2CC2)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn tim3trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM3TRGO)
    }
    #[doc = "Timer 4 CC4 event"]
    #[inline(always)]
    pub fn tim4cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM4CC4)
    }
    #[doc = "EXTI line 11/TIM8_TRGO event (TIM8_TRGO is available only in high-density and XL-density devices)"]
    #[inline(always)]
    pub fn exti11(self) -> &'a mut W {
        self.variant(EXTSEL_A::EXTI11)
    }
    #[doc = "SWSTART"]
    #[inline(always)]
    pub fn swstart(self) -> &'a mut W {
        self.variant(EXTSEL_A::SWSTART)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "External trigger conversion mode for injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEXTTRIG_A {
    #[doc = "0: Conversion on external event disabled"]
    DISABLED = 0,
    #[doc = "1: Conversion on external event enabled"]
    ENABLED = 1,
}
impl From<JEXTTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: JEXTTRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEXTTRIG` reader - External trigger conversion mode for injected channels"]
pub struct JEXTTRIG_R(crate::FieldReader<bool, JEXTTRIG_A>);
impl JEXTTRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEXTTRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEXTTRIG_A {
        match self.bits {
            false => JEXTTRIG_A::DISABLED,
            true => JEXTTRIG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == JEXTTRIG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == JEXTTRIG_A::ENABLED
    }
}
impl core::ops::Deref for JEXTTRIG_R {
    type Target = crate::FieldReader<bool, JEXTTRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEXTTRIG` writer - External trigger conversion mode for injected channels"]
pub struct JEXTTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTTRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JEXTTRIG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion on external event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEXTTRIG_A::DISABLED)
    }
    #[doc = "Conversion on external event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JEXTTRIG_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "External event select for injected group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JEXTSEL_A {
    #[doc = "0: Timer 1 TRGO event"]
    TIM1TRGO = 0,
    #[doc = "1: Timer 1 CC4 event"]
    TIM1CC4 = 1,
    #[doc = "2: Timer 2 TRGO event"]
    TIM2TRGO = 2,
    #[doc = "3: Timer 2 CC1 event"]
    TIM2CC1 = 3,
    #[doc = "4: Timer 3 CC4 event"]
    TIM3CC4 = 4,
    #[doc = "5: Timer 4 TRGO event"]
    TIM4TRGO = 5,
    #[doc = "6: EXTI line15/TIM8_CC4 event (TIM8_CC4 is available only in high-density and XL-density devices)"]
    EXTI15 = 6,
    #[doc = "7: JSWSTART"]
    JSWSTART = 7,
}
impl From<JEXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `JEXTSEL` reader - External event select for injected group"]
pub struct JEXTSEL_R(crate::FieldReader<u8, JEXTSEL_A>);
impl JEXTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        JEXTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEXTSEL_A {
        match self.bits {
            0 => JEXTSEL_A::TIM1TRGO,
            1 => JEXTSEL_A::TIM1CC4,
            2 => JEXTSEL_A::TIM2TRGO,
            3 => JEXTSEL_A::TIM2CC1,
            4 => JEXTSEL_A::TIM3CC4,
            5 => JEXTSEL_A::TIM4TRGO,
            6 => JEXTSEL_A::EXTI15,
            7 => JEXTSEL_A::JSWSTART,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIM1TRGO`"]
    #[inline(always)]
    pub fn is_tim1trgo(&self) -> bool {
        **self == JEXTSEL_A::TIM1TRGO
    }
    #[doc = "Checks if the value of the field is `TIM1CC4`"]
    #[inline(always)]
    pub fn is_tim1cc4(&self) -> bool {
        **self == JEXTSEL_A::TIM1CC4
    }
    #[doc = "Checks if the value of the field is `TIM2TRGO`"]
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        **self == JEXTSEL_A::TIM2TRGO
    }
    #[doc = "Checks if the value of the field is `TIM2CC1`"]
    #[inline(always)]
    pub fn is_tim2cc1(&self) -> bool {
        **self == JEXTSEL_A::TIM2CC1
    }
    #[doc = "Checks if the value of the field is `TIM3CC4`"]
    #[inline(always)]
    pub fn is_tim3cc4(&self) -> bool {
        **self == JEXTSEL_A::TIM3CC4
    }
    #[doc = "Checks if the value of the field is `TIM4TRGO`"]
    #[inline(always)]
    pub fn is_tim4trgo(&self) -> bool {
        **self == JEXTSEL_A::TIM4TRGO
    }
    #[doc = "Checks if the value of the field is `EXTI15`"]
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        **self == JEXTSEL_A::EXTI15
    }
    #[doc = "Checks if the value of the field is `JSWSTART`"]
    #[inline(always)]
    pub fn is_jswstart(&self) -> bool {
        **self == JEXTSEL_A::JSWSTART
    }
}
impl core::ops::Deref for JEXTSEL_R {
    type Target = crate::FieldReader<u8, JEXTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEXTSEL` writer - External event select for injected group"]
pub struct JEXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JEXTSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn tim1trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM1TRGO)
    }
    #[doc = "Timer 1 CC4 event"]
    #[inline(always)]
    pub fn tim1cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM1CC4)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM2TRGO)
    }
    #[doc = "Timer 2 CC1 event"]
    #[inline(always)]
    pub fn tim2cc1(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM2CC1)
    }
    #[doc = "Timer 3 CC4 event"]
    #[inline(always)]
    pub fn tim3cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM3CC4)
    }
    #[doc = "Timer 4 TRGO event"]
    #[inline(always)]
    pub fn tim4trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM4TRGO)
    }
    #[doc = "EXTI line15/TIM8_CC4 event (TIM8_CC4 is available only in high-density and XL-density devices)"]
    #[inline(always)]
    pub fn exti15(self) -> &'a mut W {
        self.variant(JEXTSEL_A::EXTI15)
    }
    #[doc = "JSWSTART"]
    #[inline(always)]
    pub fn jswstart(self) -> &'a mut W {
        self.variant(JEXTSEL_A::JSWSTART)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Data alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN_A {
    #[doc = "0: Right Alignment"]
    RIGHT = 0,
    #[doc = "1: Left Alignment"]
    LEFT = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN` reader - Data alignment"]
pub struct ALIGN_R(crate::FieldReader<bool, ALIGN_A>);
impl ALIGN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::RIGHT,
            true => ALIGN_A::LEFT,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        **self == ALIGN_A::RIGHT
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        **self == ALIGN_A::LEFT
    }
}
impl core::ops::Deref for ALIGN_R {
    type Target = crate::FieldReader<bool, ALIGN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIGN` writer - Data alignment"]
pub struct ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIGN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Right Alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::RIGHT)
    }
    #[doc = "Left Alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_A::LEFT)
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
#[doc = "Direct memory access mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: DMA mode disabled"]
    DISABLED = 0,
    #[doc = "1: DMA mode enabled"]
    ENABLED = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - Direct memory access mode"]
pub struct DMA_R(crate::FieldReader<bool, DMA_A>);
impl DMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::DISABLED,
            true => DMA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMA_A::ENABLED
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, DMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA` writer - Direct memory access mode"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_A::DISABLED)
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_A::ENABLED)
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
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTCAL_A {
    #[doc = "0: Calibration register initialized"]
    INITIALIZED = 0,
    #[doc = "1: Initializing calibration register"]
    NOTINITIALIZED = 1,
}
impl From<RSTCAL_A> for bool {
    #[inline(always)]
    fn from(variant: RSTCAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCAL` reader - Reset calibration"]
pub struct RSTCAL_R(crate::FieldReader<bool, RSTCAL_A>);
impl RSTCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTCAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTCAL_A {
        match self.bits {
            false => RSTCAL_A::INITIALIZED,
            true => RSTCAL_A::NOTINITIALIZED,
        }
    }
    #[doc = "Checks if the value of the field is `INITIALIZED`"]
    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        **self == RSTCAL_A::INITIALIZED
    }
    #[doc = "Checks if the value of the field is `NOTINITIALIZED`"]
    #[inline(always)]
    pub fn is_not_initialized(&self) -> bool {
        **self == RSTCAL_A::NOTINITIALIZED
    }
}
impl core::ops::Deref for RSTCAL_R {
    type Target = crate::FieldReader<bool, RSTCAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTCAL_AW {
    #[doc = "1: Initialize calibration register"]
    INITIALIZE = 1,
}
impl From<RSTCAL_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTCAL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCAL` writer - Reset calibration"]
pub struct RSTCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTCAL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Initialize calibration register"]
    #[inline(always)]
    pub fn initialize(self) -> &'a mut W {
        self.variant(RSTCAL_AW::INITIALIZE)
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
#[doc = "A/D calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_A {
    #[doc = "0: Calibration completed"]
    COMPLETE = 0,
    #[doc = "1: Calibrating"]
    NOTCOMPLETE = 1,
}
impl From<CAL_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL` reader - A/D calibration"]
pub struct CAL_R(crate::FieldReader<bool, CAL_A>);
impl CAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAL_A {
        match self.bits {
            false => CAL_A::COMPLETE,
            true => CAL_A::NOTCOMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == CAL_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == CAL_A::NOTCOMPLETE
    }
}
impl core::ops::Deref for CAL_R {
    type Target = crate::FieldReader<bool, CAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "A/D calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_AW {
    #[doc = "1: Enable calibration"]
    START = 1,
}
impl From<CAL_AW> for bool {
    #[inline(always)]
    fn from(variant: CAL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL` writer - A/D calibration"]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable calibration"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CAL_AW::START)
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
#[doc = "Continuous conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    #[doc = "0: Single conversion mode"]
    SINGLE = 0,
    #[doc = "1: Continuous conversion mode"]
    CONTINUOUS = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - Continuous conversion"]
pub struct CONT_R(crate::FieldReader<bool, CONT_A>);
impl CONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::SINGLE,
            true => CONT_A::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == CONT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == CONT_A::CONTINUOUS
    }
}
impl core::ops::Deref for CONT_R {
    type Target = crate::FieldReader<bool, CONT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONT` writer - Continuous conversion"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CONT_A::SINGLE)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::CONTINUOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "A/D converter ON / OFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADON_A {
    #[doc = "0: Disable ADC conversion/calibration and go to power down mode"]
    DISABLED = 0,
    #[doc = "1: Enable ADC and to start conversion"]
    ENABLED = 1,
}
impl From<ADON_A> for bool {
    #[inline(always)]
    fn from(variant: ADON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADON` reader - A/D converter ON / OFF"]
pub struct ADON_R(crate::FieldReader<bool, ADON_A>);
impl ADON_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADON_A {
        match self.bits {
            false => ADON_A::DISABLED,
            true => ADON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ADON_A::ENABLED
    }
}
impl core::ops::Deref for ADON_R {
    type Target = crate::FieldReader<bool, ADON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADON` writer - A/D converter ON / OFF"]
pub struct ADON_W<'a> {
    w: &'a mut W,
}
impl<'a> ADON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable ADC conversion/calibration and go to power down mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADON_A::DISABLED)
    }
    #[doc = "Enable ADC and to start conversion"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADON_A::ENABLED)
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
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Start conversion of regular channels"]
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - External trigger conversion mode for regular channels"]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - External event select for regular group"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 15 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    pub fn jexttrig(&self) -> JEXTTRIG_R {
        JEXTTRIG_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - External event select for injected group"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstcal(&self) -> RSTCAL_R {
        RSTCAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - A/D calibration"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - A/D converter ON / OFF"]
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&mut self) -> TSVREFE_W {
        TSVREFE_W { w: self }
    }
    #[doc = "Bit 22 - Start conversion of regular channels"]
    #[inline(always)]
    pub fn swstart(&mut self) -> SWSTART_W {
        SWSTART_W { w: self }
    }
    #[doc = "Bit 21 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W {
        JSWSTART_W { w: self }
    }
    #[doc = "Bit 20 - External trigger conversion mode for regular channels"]
    #[inline(always)]
    pub fn exttrig(&mut self) -> EXTTRIG_W {
        EXTTRIG_W { w: self }
    }
    #[doc = "Bits 17:19 - External event select for regular group"]
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W {
        EXTSEL_W { w: self }
    }
    #[doc = "Bit 15 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    pub fn jexttrig(&mut self) -> JEXTTRIG_W {
        JEXTTRIG_W { w: self }
    }
    #[doc = "Bits 12:14 - External event select for injected group"]
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W {
        JEXTSEL_W { w: self }
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W {
        ALIGN_W { w: self }
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstcal(&mut self) -> RSTCAL_W {
        RSTCAL_W { w: self }
    }
    #[doc = "Bit 2 - A/D calibration"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bit 0 - A/D converter ON / OFF"]
    #[inline(always)]
    pub fn adon(&mut self) -> ADON_W {
        ADON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
