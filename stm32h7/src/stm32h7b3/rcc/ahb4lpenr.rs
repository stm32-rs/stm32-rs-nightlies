#[doc = "Register `AHB4LPENR` reader"]
pub struct R(crate::R<AHB4LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB4LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB4LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB4LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB4LPENR` writer"]
pub struct W(crate::W<AHB4LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB4LPENR_SPEC>;
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
impl From<crate::W<AHB4LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB4LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIOA peripheral clock enable during CSleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOALPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    ENABLED = 1,
}
impl From<GPIOALPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOALPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOALPEN` reader - GPIOA peripheral clock enable during CSleep mode Set and reset by software."]
pub struct GPIOALPEN_R(crate::FieldReader<bool, GPIOALPEN_A>);
impl GPIOALPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOALPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOALPEN_A {
        match self.bits {
            false => GPIOALPEN_A::DISABLED,
            true => GPIOALPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == GPIOALPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == GPIOALPEN_A::ENABLED
    }
}
impl core::ops::Deref for GPIOALPEN_R {
    type Target = crate::FieldReader<bool, GPIOALPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOALPEN` writer - GPIOA peripheral clock enable during CSleep mode Set and reset by software."]
pub struct GPIOALPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOALPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOALPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOALPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOALPEN_A::ENABLED)
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
#[doc = "GPIOB peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOBLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOBLPEN` reader - GPIOB peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOBLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOBLPEN` writer - GPIOB peripheral clock enable during CSleep mode Set and reset by software."]
pub struct GPIOBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOBLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOBLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOBLPEN_A::ENABLED)
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
#[doc = "GPIOC peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOCLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOCLPEN` reader - GPIOC peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOCLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOCLPEN` writer - GPIOC peripheral clock enable during CSleep mode Set and reset by software."]
pub struct GPIOCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOCLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOCLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOCLPEN_A::ENABLED)
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
#[doc = "GPIOD peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIODLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIODLPEN` reader - GPIOD peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIODLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIODLPEN` writer - GPIOD peripheral clock enable during CSleep mode Set and reset by software."]
pub struct GPIODLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIODLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIODLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIODLPEN_A::ENABLED)
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
#[doc = "GPIOE peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOELPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOELPEN` reader - GPIOE peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOELPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOELPEN` writer - GPIOE peripheral clock enable during CSleep mode Set and reset by software."]
pub struct GPIOELPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOELPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOELPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOELPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOELPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "GPIOF peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOFLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOFLPEN` reader - GPIOF peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOFLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOFLPEN` writer - GPIOF peripheral clock enable during CSleep mode Set and reset by software."]
pub struct GPIOFLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOFLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOFLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOFLPEN_A::ENABLED)
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
#[doc = "GPIOG peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOGLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOGLPEN` reader - GPIOG peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOGLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOGLPEN` writer - GPIOG peripheral clock enable during CSleep mode Set and reset by software."]
pub struct GPIOGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOGLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOGLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOGLPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "GPIOH peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOHLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOHLPEN` reader - GPIOH peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOHLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOHLPEN` writer - GPIOH peripheral clock enable during CSleep mode Set and reset by software."]
pub struct GPIOHLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOHLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOHLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOHLPEN_A::ENABLED)
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
#[doc = "GPIOI peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOILPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOILPEN` reader - GPIOI peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOILPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOILPEN` writer - GPIOI peripheral clock enable during CSleep mode Set and reset by software."]
pub struct GPIOILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOILPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOILPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOILPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOILPEN_A::ENABLED)
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
#[doc = "GPIOJ peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOJLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOJLPEN` reader - GPIOJ peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOJLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOJLPEN` writer - GPIOJ peripheral clock enable during CSleep mode Set and reset by software."]
pub struct GPIOJLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOJLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOJLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOJLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOJLPEN_A::ENABLED)
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
#[doc = "GPIOK peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOKLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOKLPEN` reader - GPIOK peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOKLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOKLPEN` writer - GPIOK peripheral clock enable during CSleep mode Set and reset by software."]
pub struct GPIOKLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOKLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOKLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOKLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOKLPEN_A::ENABLED)
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
#[doc = "SmartRun domain DMA and DMAMUX clock enable during CSleep mode Set and reset by software."]
pub type BDMA2LPEN_A = GPIOALPEN_A;
#[doc = "Field `BDMA2LPEN` reader - SmartRun domain DMA and DMAMUX clock enable during CSleep mode Set and reset by software."]
pub type BDMA2LPEN_R = GPIOALPEN_R;
#[doc = "Field `BDMA2LPEN` writer - SmartRun domain DMA and DMAMUX clock enable during CSleep mode Set and reset by software."]
pub struct BDMA2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMA2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDMA2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BDMA2LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BDMA2LPEN_A::ENABLED)
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
#[doc = "Backup RAM clock enable during CSleep mode Set and reset by software."]
pub type BKPRAMLPEN_A = GPIOALPEN_A;
#[doc = "Field `BKPRAMLPEN` reader - Backup RAM clock enable during CSleep mode Set and reset by software."]
pub type BKPRAMLPEN_R = GPIOALPEN_R;
#[doc = "Field `BKPRAMLPEN` writer - Backup RAM clock enable during CSleep mode Set and reset by software."]
pub struct BKPRAMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPRAMLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKPRAMLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BKPRAMLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BKPRAMLPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "SmartRun domain SRAM clock enable during CSleep mode Set and reset by software."]
pub type SRDSRAMLPEN_A = GPIOALPEN_A;
#[doc = "Field `SRDSRAMLPEN` reader - SmartRun domain SRAM clock enable during CSleep mode Set and reset by software."]
pub type SRDSRAMLPEN_R = GPIOALPEN_R;
#[doc = "Field `SRDSRAMLPEN` writer - SmartRun domain SRAM clock enable during CSleep mode Set and reset by software."]
pub struct SRDSRAMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRDSRAMLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRDSRAMLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRDSRAMLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRDSRAMLPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GPIOA peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIOB peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIOC peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIOD peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIOE peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIOF peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIOG peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIOH peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIOI peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIOJ peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GPIOJLPEN_R {
        GPIOJLPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIOK peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioklpen(&self) -> GPIOKLPEN_R {
        GPIOKLPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SmartRun domain DMA and DMAMUX clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn bdma2lpen(&self) -> BDMA2LPEN_R {
        BDMA2LPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Backup RAM clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn bkpramlpen(&self) -> BKPRAMLPEN_R {
        BKPRAMLPEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SmartRun domain SRAM clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn srdsramlpen(&self) -> SRDSRAMLPEN_R {
        SRDSRAMLPEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W {
        GPIOALPEN_W { w: self }
    }
    #[doc = "Bit 1 - GPIOB peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W {
        GPIOBLPEN_W { w: self }
    }
    #[doc = "Bit 2 - GPIOC peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W {
        GPIOCLPEN_W { w: self }
    }
    #[doc = "Bit 3 - GPIOD peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W {
        GPIODLPEN_W { w: self }
    }
    #[doc = "Bit 4 - GPIOE peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W {
        GPIOELPEN_W { w: self }
    }
    #[doc = "Bit 5 - GPIOF peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W {
        GPIOFLPEN_W { w: self }
    }
    #[doc = "Bit 6 - GPIOG peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W {
        GPIOGLPEN_W { w: self }
    }
    #[doc = "Bit 7 - GPIOH peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W {
        GPIOHLPEN_W { w: self }
    }
    #[doc = "Bit 8 - GPIOI peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W {
        GPIOILPEN_W { w: self }
    }
    #[doc = "Bit 9 - GPIOJ peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiojlpen(&mut self) -> GPIOJLPEN_W {
        GPIOJLPEN_W { w: self }
    }
    #[doc = "Bit 10 - GPIOK peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioklpen(&mut self) -> GPIOKLPEN_W {
        GPIOKLPEN_W { w: self }
    }
    #[doc = "Bit 21 - SmartRun domain DMA and DMAMUX clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn bdma2lpen(&mut self) -> BDMA2LPEN_W {
        BDMA2LPEN_W { w: self }
    }
    #[doc = "Bit 28 - Backup RAM clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn bkpramlpen(&mut self) -> BKPRAMLPEN_W {
        BKPRAMLPEN_W { w: self }
    }
    #[doc = "Bit 29 - SmartRun domain SRAM clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn srdsramlpen(&mut self) -> SRDSRAMLPEN_W {
        SRDSRAMLPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb4lpenr](index.html) module"]
pub struct AHB4LPENR_SPEC;
impl crate::RegisterSpec for AHB4LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb4lpenr::R](R) reader structure"]
impl crate::Readable for AHB4LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb4lpenr::W](W) writer structure"]
impl crate::Writable for AHB4LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB4LPENR to value 0x3020_07ff"]
impl crate::Resettable for AHB4LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3020_07ff
    }
}
