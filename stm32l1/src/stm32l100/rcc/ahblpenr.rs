#[doc = "Register `AHBLPENR` reader"]
pub struct R(crate::R<AHBLPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBLPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBLPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBLPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBLPENR` writer"]
pub struct W(crate::W<AHBLPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBLPENR_SPEC>;
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
impl From<crate::W<AHBLPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBLPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA2 clock enable during Sleep mode"]
pub type DMA2LPEN_A = GPIOALPEN_A;
#[doc = "Field `DMA2LPEN` reader - DMA2 clock enable during Sleep mode"]
pub type DMA2LPEN_R = GPIOALPEN_R;
#[doc = "Field `DMA2LPEN` writer - DMA2 clock enable during Sleep mode"]
pub struct DMA2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA2LPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA2LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "DMA1 clock enable during Sleep mode"]
pub type DMA1LPEN_A = GPIOALPEN_A;
#[doc = "Field `DMA1LPEN` reader - DMA1 clock enable during Sleep mode"]
pub type DMA1LPEN_R = GPIOALPEN_R;
#[doc = "Field `DMA1LPEN` writer - DMA1 clock enable during Sleep mode"]
pub struct DMA1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::ENABLED)
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
#[doc = "SRAM clock enable during Sleep mode"]
pub type SRAMLPEN_A = GPIOALPEN_A;
#[doc = "Field `SRAMLPEN` reader - SRAM clock enable during Sleep mode"]
pub type SRAMLPEN_R = GPIOALPEN_R;
#[doc = "Field `SRAMLPEN` writer - SRAM clock enable during Sleep mode"]
pub struct SRAMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAMLPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAMLPEN_A::ENABLED)
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
#[doc = "FLITF clock enable during Sleep mode"]
pub type FLITFLPEN_A = GPIOALPEN_A;
#[doc = "Field `FLITFLPEN` reader - FLITF clock enable during Sleep mode"]
pub type FLITFLPEN_R = GPIOALPEN_R;
#[doc = "Field `FLITFLPEN` writer - FLITF clock enable during Sleep mode"]
pub struct FLITFLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLITFLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLITFLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLITFLPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLITFLPEN_A::ENABLED)
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
#[doc = "CRC clock enable during Sleep mode"]
pub type CRCLPEN_A = GPIOALPEN_A;
#[doc = "Field `CRCLPEN` reader - CRC clock enable during Sleep mode"]
pub type CRCLPEN_R = GPIOALPEN_R;
#[doc = "Field `CRCLPEN` writer - CRC clock enable during Sleep mode"]
pub struct CRCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCLPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCLPEN_A::ENABLED)
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
#[doc = "IO port G clock enable during Sleep mode"]
pub type GPIOGLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOGLPEN` reader - IO port G clock enable during Sleep mode"]
pub type GPIOGLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOGLPEN` writer - IO port G clock enable during Sleep mode"]
pub struct GPIOGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOGLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOGLPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "IO port F clock enable during Sleep mode"]
pub type GPIOFLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOFLPEN` reader - IO port F clock enable during Sleep mode"]
pub type GPIOFLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOFLPEN` writer - IO port F clock enable during Sleep mode"]
pub struct GPIOFLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOFLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOFLPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "IO port H clock enable during Sleep mode"]
pub type GPIOHLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOHLPEN` reader - IO port H clock enable during Sleep mode"]
pub type GPIOHLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOHLPEN` writer - IO port H clock enable during Sleep mode"]
pub struct GPIOHLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOHLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOHLPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "IO port E clock enable during Sleep mode"]
pub type GPIOELPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOELPEN` reader - IO port E clock enable during Sleep mode"]
pub type GPIOELPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOELPEN` writer - IO port E clock enable during Sleep mode"]
pub struct GPIOELPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOELPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOELPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOELPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
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
#[doc = "IO port D clock enable during Sleep mode"]
pub type GPIODLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIODLPEN` reader - IO port D clock enable during Sleep mode"]
pub type GPIODLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIODLPEN` writer - IO port D clock enable during Sleep mode"]
pub struct GPIODLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIODLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIODLPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
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
#[doc = "IO port C clock enable during Sleep mode"]
pub type GPIOCLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOCLPEN` reader - IO port C clock enable during Sleep mode"]
pub type GPIOCLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOCLPEN` writer - IO port C clock enable during Sleep mode"]
pub struct GPIOCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOCLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOCLPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
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
#[doc = "IO port B clock enable during Sleep mode"]
pub type GPIOBLPEN_A = GPIOALPEN_A;
#[doc = "Field `GPIOBLPEN` reader - IO port B clock enable during Sleep mode"]
pub type GPIOBLPEN_R = GPIOALPEN_R;
#[doc = "Field `GPIOBLPEN` writer - IO port B clock enable during Sleep mode"]
pub struct GPIOBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOBLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOBLPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
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
#[doc = "IO port A clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOALPEN_A {
    #[doc = "0: Clock disabled"]
    DISABLED = 0,
    #[doc = "1: Clock enabled"]
    ENABLED = 1,
}
impl From<GPIOALPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOALPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOALPEN` reader - IO port A clock enable during Sleep mode"]
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
#[doc = "Field `GPIOALPEN` writer - IO port A clock enable during Sleep mode"]
pub struct GPIOALPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOALPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOALPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOALPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
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
#[doc = "FSMC clock enable during Sleep mode"]
pub type FSMCLPEN_A = GPIOALPEN_A;
#[doc = "Field `FSMCLPEN` reader - FSMC clock enable during Sleep mode"]
pub type FSMCLPEN_R = GPIOALPEN_R;
#[doc = "Field `FSMCLPEN` writer - FSMC clock enable during Sleep mode"]
pub struct FSMCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSMCLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FSMCLPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FSMCLPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "AES clock enable during Sleep mode"]
pub type AESLPEN_A = GPIOALPEN_A;
#[doc = "Field `AESLPEN` reader - AES clock enable during Sleep mode"]
pub type AESLPEN_R = GPIOALPEN_R;
#[doc = "Field `AESLPEN` writer - AES clock enable during Sleep mode"]
pub struct AESLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AESLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AESLPEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AESLPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SRAM clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sramlpen(&self) -> SRAMLPEN_R {
        SRAMLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FLITF clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IO port G clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IO port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - IO port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 30 - FSMC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fsmclpen(&self) -> FSMCLPEN_R {
        FSMCLPEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 27 - AES clock enable during Sleep mode"]
    #[inline(always)]
    pub fn aeslpen(&self) -> AESLPEN_R {
        AESLPEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W {
        DMA2LPEN_W { w: self }
    }
    #[doc = "Bit 24 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W {
        DMA1LPEN_W { w: self }
    }
    #[doc = "Bit 16 - SRAM clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sramlpen(&mut self) -> SRAMLPEN_W {
        SRAMLPEN_W { w: self }
    }
    #[doc = "Bit 15 - FLITF clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W {
        FLITFLPEN_W { w: self }
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crclpen(&mut self) -> CRCLPEN_W {
        CRCLPEN_W { w: self }
    }
    #[doc = "Bit 7 - IO port G clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W {
        GPIOGLPEN_W { w: self }
    }
    #[doc = "Bit 6 - IO port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W {
        GPIOFLPEN_W { w: self }
    }
    #[doc = "Bit 5 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W {
        GPIOHLPEN_W { w: self }
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W {
        GPIOELPEN_W { w: self }
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W {
        GPIODLPEN_W { w: self }
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W {
        GPIOCLPEN_W { w: self }
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W {
        GPIOBLPEN_W { w: self }
    }
    #[doc = "Bit 0 - IO port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W {
        GPIOALPEN_W { w: self }
    }
    #[doc = "Bit 30 - FSMC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fsmclpen(&mut self) -> FSMCLPEN_W {
        FSMCLPEN_W { w: self }
    }
    #[doc = "Bit 27 - AES clock enable during Sleep mode"]
    #[inline(always)]
    pub fn aeslpen(&mut self) -> AESLPEN_W {
        AESLPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahblpenr](index.html) module"]
pub struct AHBLPENR_SPEC;
impl crate::RegisterSpec for AHBLPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahblpenr::R](R) reader structure"]
impl crate::Readable for AHBLPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahblpenr::W](W) writer structure"]
impl crate::Writable for AHBLPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBLPENR to value 0x0101_903f"]
impl crate::Resettable for AHBLPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101_903f
    }
}
