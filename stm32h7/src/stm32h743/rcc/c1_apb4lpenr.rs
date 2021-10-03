#[doc = "Register `C1_APB4LPENR` reader"]
pub struct R(crate::R<C1_APB4LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_APB4LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_APB4LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_APB4LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_APB4LPENR` writer"]
pub struct W(crate::W<C1_APB4LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_APB4LPENR_SPEC>;
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
impl From<crate::W<C1_APB4LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_APB4LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SYSCFG peripheral clock enable during CSleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCFGLPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    ENABLED = 1,
}
impl From<SYSCFGLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGLPEN` reader - SYSCFG peripheral clock enable during CSleep mode"]
pub struct SYSCFGLPEN_R(crate::FieldReader<bool, SYSCFGLPEN_A>);
impl SYSCFGLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGLPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGLPEN_A {
        match self.bits {
            false => SYSCFGLPEN_A::DISABLED,
            true => SYSCFGLPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SYSCFGLPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SYSCFGLPEN_A::ENABLED
    }
}
impl core::ops::Deref for SYSCFGLPEN_R {
    type Target = crate::FieldReader<bool, SYSCFGLPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGLPEN` writer - SYSCFG peripheral clock enable during CSleep mode"]
pub struct SYSCFGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSCFGLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSCFGLPEN_A::ENABLED)
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
#[doc = "LPUART1 Peripheral Clocks Enable During CSleep Mode"]
pub type LPUART1LPEN_A = SYSCFGLPEN_A;
#[doc = "Field `LPUART1LPEN` reader - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
pub type LPUART1LPEN_R = SYSCFGLPEN_R;
#[doc = "Field `LPUART1LPEN` writer - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
pub struct LPUART1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPUART1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPUART1LPEN_A::ENABLED)
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
#[doc = "SPI6 Peripheral Clocks Enable During CSleep Mode"]
pub type SPI6LPEN_A = SYSCFGLPEN_A;
#[doc = "Field `SPI6LPEN` reader - SPI6 Peripheral Clocks Enable During CSleep Mode"]
pub type SPI6LPEN_R = SYSCFGLPEN_R;
#[doc = "Field `SPI6LPEN` writer - SPI6 Peripheral Clocks Enable During CSleep Mode"]
pub struct SPI6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI6LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI6LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI6LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "I2C4 Peripheral Clocks Enable During CSleep Mode"]
pub type I2C4LPEN_A = SYSCFGLPEN_A;
#[doc = "Field `I2C4LPEN` reader - I2C4 Peripheral Clocks Enable During CSleep Mode"]
pub type I2C4LPEN_R = SYSCFGLPEN_R;
#[doc = "Field `I2C4LPEN` writer - I2C4 Peripheral Clocks Enable During CSleep Mode"]
pub struct I2C4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C4LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C4LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C4LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM2LPEN_A = SYSCFGLPEN_A;
#[doc = "Field `LPTIM2LPEN` reader - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM2LPEN_R = SYSCFGLPEN_R;
#[doc = "Field `LPTIM2LPEN` writer - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
pub struct LPTIM2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM2LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM2LPEN_A::ENABLED)
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
#[doc = "LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM3LPEN_A = SYSCFGLPEN_A;
#[doc = "Field `LPTIM3LPEN` reader - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM3LPEN_R = SYSCFGLPEN_R;
#[doc = "Field `LPTIM3LPEN` writer - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
pub struct LPTIM3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM3LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM3LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM3LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM4LPEN_A = SYSCFGLPEN_A;
#[doc = "Field `LPTIM4LPEN` reader - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM4LPEN_R = SYSCFGLPEN_R;
#[doc = "Field `LPTIM4LPEN` writer - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
pub struct LPTIM4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM4LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM4LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM4LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM4LPEN_A::ENABLED)
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
#[doc = "LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM5LPEN_A = SYSCFGLPEN_A;
#[doc = "Field `LPTIM5LPEN` reader - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM5LPEN_R = SYSCFGLPEN_R;
#[doc = "Field `LPTIM5LPEN` writer - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
pub struct LPTIM5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM5LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM5LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM5LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM5LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "COMP1/2 peripheral clock enable during CSleep mode"]
pub type COMP12LPEN_A = SYSCFGLPEN_A;
#[doc = "Field `COMP12LPEN` reader - COMP1/2 peripheral clock enable during CSleep mode"]
pub type COMP12LPEN_R = SYSCFGLPEN_R;
#[doc = "Field `COMP12LPEN` writer - COMP1/2 peripheral clock enable during CSleep mode"]
pub struct COMP12LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP12LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP12LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP12LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP12LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "VREF peripheral clock enable during CSleep mode"]
pub type VREFLPEN_A = SYSCFGLPEN_A;
#[doc = "Field `VREFLPEN` reader - VREF peripheral clock enable during CSleep mode"]
pub type VREFLPEN_R = SYSCFGLPEN_R;
#[doc = "Field `VREFLPEN` writer - VREF peripheral clock enable during CSleep mode"]
pub struct VREFLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREFLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREFLPEN_A::ENABLED)
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
#[doc = "RTC APB Clock Enable During CSleep Mode"]
pub type RTCAPBLPEN_A = SYSCFGLPEN_A;
#[doc = "Field `RTCAPBLPEN` reader - RTC APB Clock Enable During CSleep Mode"]
pub type RTCAPBLPEN_R = SYSCFGLPEN_R;
#[doc = "Field `RTCAPBLPEN` writer - RTC APB Clock Enable During CSleep Mode"]
pub struct RTCAPBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAPBLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCAPBLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCAPBLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCAPBLPEN_A::ENABLED)
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
#[doc = "SAI4 Peripheral Clocks Enable During CSleep Mode"]
pub type SAI4LPEN_A = SYSCFGLPEN_A;
#[doc = "Field `SAI4LPEN` reader - SAI4 Peripheral Clocks Enable During CSleep Mode"]
pub type SAI4LPEN_R = SYSCFGLPEN_R;
#[doc = "Field `SAI4LPEN` writer - SAI4 Peripheral Clocks Enable During CSleep Mode"]
pub struct SAI4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI4LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI4LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAI4LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAI4LPEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn comp12lpen(&self) -> COMP12LPEN_R {
        COMP12LPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai4lpen(&self) -> SAI4LPEN_R {
        SAI4LPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W {
        SYSCFGLPEN_W { w: self }
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W {
        LPUART1LPEN_W { w: self }
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W {
        SPI6LPEN_W { w: self }
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W {
        I2C4LPEN_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W {
        LPTIM2LPEN_W { w: self }
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W {
        LPTIM3LPEN_W { w: self }
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W {
        LPTIM4LPEN_W { w: self }
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W {
        LPTIM5LPEN_W { w: self }
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn comp12lpen(&mut self) -> COMP12LPEN_W {
        COMP12LPEN_W { w: self }
    }
    #[doc = "Bit 15 - VREF peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn vreflpen(&mut self) -> VREFLPEN_W {
        VREFLPEN_W { w: self }
    }
    #[doc = "Bit 16 - RTC APB Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W {
        RTCAPBLPEN_W { w: self }
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai4lpen(&mut self) -> SAI4LPEN_W {
        SAI4LPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB4 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb4lpenr](index.html) module"]
pub struct C1_APB4LPENR_SPEC;
impl crate::RegisterSpec for C1_APB4LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_apb4lpenr::R](R) reader structure"]
impl crate::Readable for C1_APB4LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_apb4lpenr::W](W) writer structure"]
impl crate::Writable for C1_APB4LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1_APB4LPENR to value 0"]
impl crate::Resettable for C1_APB4LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
