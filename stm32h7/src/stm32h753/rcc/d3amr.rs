#[doc = "Register `D3AMR` reader"]
pub struct R(crate::R<D3AMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3AMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3AMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3AMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3AMR` writer"]
pub struct W(crate::W<D3AMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3AMR_SPEC>;
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
impl From<crate::W<D3AMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3AMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BDMA and DMAMUX Autonomous mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDMAAMEN_A {
    #[doc = "0: Clock disabled in autonomous mode"]
    DISABLED = 0,
    #[doc = "1: Clock enabled in autonomous mode"]
    ENABLED = 1,
}
impl From<BDMAAMEN_A> for bool {
    #[inline(always)]
    fn from(variant: BDMAAMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDMAAMEN` reader - BDMA and DMAMUX Autonomous mode enable"]
pub struct BDMAAMEN_R(crate::FieldReader<bool, BDMAAMEN_A>);
impl BDMAAMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BDMAAMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDMAAMEN_A {
        match self.bits {
            false => BDMAAMEN_A::DISABLED,
            true => BDMAAMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BDMAAMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BDMAAMEN_A::ENABLED
    }
}
impl core::ops::Deref for BDMAAMEN_R {
    type Target = crate::FieldReader<bool, BDMAAMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDMAAMEN` writer - BDMA and DMAMUX Autonomous mode enable"]
pub struct BDMAAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMAAMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDMAAMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BDMAAMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BDMAAMEN_A::ENABLED)
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
#[doc = "LPUART1 Autonomous mode enable"]
pub type LPUART1AMEN_A = BDMAAMEN_A;
#[doc = "Field `LPUART1AMEN` reader - LPUART1 Autonomous mode enable"]
pub type LPUART1AMEN_R = BDMAAMEN_R;
#[doc = "Field `LPUART1AMEN` writer - LPUART1 Autonomous mode enable"]
pub struct LPUART1AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPUART1AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPUART1AMEN_A::ENABLED)
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
#[doc = "SPI6 Autonomous mode enable"]
pub type SPI6AMEN_A = BDMAAMEN_A;
#[doc = "Field `SPI6AMEN` reader - SPI6 Autonomous mode enable"]
pub type SPI6AMEN_R = BDMAAMEN_R;
#[doc = "Field `SPI6AMEN` writer - SPI6 Autonomous mode enable"]
pub struct SPI6AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI6AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI6AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI6AMEN_A::ENABLED)
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
#[doc = "I2C4 Autonomous mode enable"]
pub type I2C4AMEN_A = BDMAAMEN_A;
#[doc = "Field `I2C4AMEN` reader - I2C4 Autonomous mode enable"]
pub type I2C4AMEN_R = BDMAAMEN_R;
#[doc = "Field `I2C4AMEN` writer - I2C4 Autonomous mode enable"]
pub struct I2C4AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C4AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C4AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C4AMEN_A::ENABLED)
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
#[doc = "LPTIM2 Autonomous mode enable"]
pub type LPTIM2AMEN_A = BDMAAMEN_A;
#[doc = "Field `LPTIM2AMEN` reader - LPTIM2 Autonomous mode enable"]
pub type LPTIM2AMEN_R = BDMAAMEN_R;
#[doc = "Field `LPTIM2AMEN` writer - LPTIM2 Autonomous mode enable"]
pub struct LPTIM2AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM2AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM2AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM2AMEN_A::ENABLED)
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
#[doc = "LPTIM3 Autonomous mode enable"]
pub type LPTIM3AMEN_A = BDMAAMEN_A;
#[doc = "Field `LPTIM3AMEN` reader - LPTIM3 Autonomous mode enable"]
pub type LPTIM3AMEN_R = BDMAAMEN_R;
#[doc = "Field `LPTIM3AMEN` writer - LPTIM3 Autonomous mode enable"]
pub struct LPTIM3AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM3AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM3AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM3AMEN_A::ENABLED)
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
#[doc = "LPTIM4 Autonomous mode enable"]
pub type LPTIM4AMEN_A = BDMAAMEN_A;
#[doc = "Field `LPTIM4AMEN` reader - LPTIM4 Autonomous mode enable"]
pub type LPTIM4AMEN_R = BDMAAMEN_R;
#[doc = "Field `LPTIM4AMEN` writer - LPTIM4 Autonomous mode enable"]
pub struct LPTIM4AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM4AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM4AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM4AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM4AMEN_A::ENABLED)
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
#[doc = "LPTIM5 Autonomous mode enable"]
pub type LPTIM5AMEN_A = BDMAAMEN_A;
#[doc = "Field `LPTIM5AMEN` reader - LPTIM5 Autonomous mode enable"]
pub type LPTIM5AMEN_R = BDMAAMEN_R;
#[doc = "Field `LPTIM5AMEN` writer - LPTIM5 Autonomous mode enable"]
pub struct LPTIM5AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM5AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM5AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM5AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM5AMEN_A::ENABLED)
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
#[doc = "COMP12 Autonomous mode enable"]
pub type COMP12AMEN_A = BDMAAMEN_A;
#[doc = "Field `COMP12AMEN` reader - COMP12 Autonomous mode enable"]
pub type COMP12AMEN_R = BDMAAMEN_R;
#[doc = "Field `COMP12AMEN` writer - COMP12 Autonomous mode enable"]
pub struct COMP12AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP12AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP12AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP12AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP12AMEN_A::ENABLED)
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
#[doc = "VREF Autonomous mode enable"]
pub type VREFAMEN_A = BDMAAMEN_A;
#[doc = "Field `VREFAMEN` reader - VREF Autonomous mode enable"]
pub type VREFAMEN_R = BDMAAMEN_R;
#[doc = "Field `VREFAMEN` writer - VREF Autonomous mode enable"]
pub struct VREFAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFAMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFAMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREFAMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREFAMEN_A::ENABLED)
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
#[doc = "RTC Autonomous mode enable"]
pub type RTCAMEN_A = BDMAAMEN_A;
#[doc = "Field `RTCAMEN` reader - RTC Autonomous mode enable"]
pub type RTCAMEN_R = BDMAAMEN_R;
#[doc = "Field `RTCAMEN` writer - RTC Autonomous mode enable"]
pub struct RTCAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCAMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCAMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCAMEN_A::ENABLED)
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
#[doc = "CRC Autonomous mode enable"]
pub type CRCAMEN_A = BDMAAMEN_A;
#[doc = "Field `CRCAMEN` reader - CRC Autonomous mode enable"]
pub type CRCAMEN_R = BDMAAMEN_R;
#[doc = "Field `CRCAMEN` writer - CRC Autonomous mode enable"]
pub struct CRCAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCAMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCAMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCAMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCAMEN_A::ENABLED)
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
#[doc = "SAI4 Autonomous mode enable"]
pub type SAI4AMEN_A = BDMAAMEN_A;
#[doc = "Field `SAI4AMEN` reader - SAI4 Autonomous mode enable"]
pub type SAI4AMEN_R = BDMAAMEN_R;
#[doc = "Field `SAI4AMEN` writer - SAI4 Autonomous mode enable"]
pub struct SAI4AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI4AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI4AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAI4AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAI4AMEN_A::ENABLED)
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
#[doc = "ADC3 Autonomous mode enable"]
pub type ADC3AMEN_A = BDMAAMEN_A;
#[doc = "Field `ADC3AMEN` reader - ADC3 Autonomous mode enable"]
pub type ADC3AMEN_R = BDMAAMEN_R;
#[doc = "Field `ADC3AMEN` writer - ADC3 Autonomous mode enable"]
pub struct ADC3AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC3AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC3AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC3AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC3AMEN_A::ENABLED)
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
#[doc = "Backup RAM Autonomous mode enable"]
pub type BKPRAMAMEN_A = BDMAAMEN_A;
#[doc = "Field `BKPRAMAMEN` reader - Backup RAM Autonomous mode enable"]
pub type BKPRAMAMEN_R = BDMAAMEN_R;
#[doc = "Field `BKPRAMAMEN` writer - Backup RAM Autonomous mode enable"]
pub struct BKPRAMAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPRAMAMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKPRAMAMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BKPRAMAMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BKPRAMAMEN_A::ENABLED)
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
#[doc = "SRAM4 Autonomous mode enable"]
pub type SRAM4AMEN_A = BDMAAMEN_A;
#[doc = "Field `SRAM4AMEN` reader - SRAM4 Autonomous mode enable"]
pub type SRAM4AMEN_R = BDMAAMEN_R;
#[doc = "Field `SRAM4AMEN` writer - SRAM4 Autonomous mode enable"]
pub struct SRAM4AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM4AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM4AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAM4AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAM4AMEN_A::ENABLED)
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
    #[doc = "Bit 0 - BDMA and DMAMUX Autonomous mode enable"]
    #[inline(always)]
    pub fn bdmaamen(&self) -> BDMAAMEN_R {
        BDMAAMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable"]
    #[inline(always)]
    pub fn lpuart1amen(&self) -> LPUART1AMEN_R {
        LPUART1AMEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable"]
    #[inline(always)]
    pub fn spi6amen(&self) -> SPI6AMEN_R {
        SPI6AMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable"]
    #[inline(always)]
    pub fn i2c4amen(&self) -> I2C4AMEN_R {
        I2C4AMEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim2amen(&self) -> LPTIM2AMEN_R {
        LPTIM2AMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim3amen(&self) -> LPTIM3AMEN_R {
        LPTIM3AMEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim4amen(&self) -> LPTIM4AMEN_R {
        LPTIM4AMEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim5amen(&self) -> LPTIM5AMEN_R {
        LPTIM5AMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - COMP12 Autonomous mode enable"]
    #[inline(always)]
    pub fn comp12amen(&self) -> COMP12AMEN_R {
        COMP12AMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable"]
    #[inline(always)]
    pub fn vrefamen(&self) -> VREFAMEN_R {
        VREFAMEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable"]
    #[inline(always)]
    pub fn rtcamen(&self) -> RTCAMEN_R {
        RTCAMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CRC Autonomous mode enable"]
    #[inline(always)]
    pub fn crcamen(&self) -> CRCAMEN_R {
        CRCAMEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SAI4 Autonomous mode enable"]
    #[inline(always)]
    pub fn sai4amen(&self) -> SAI4AMEN_R {
        SAI4AMEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC3 Autonomous mode enable"]
    #[inline(always)]
    pub fn adc3amen(&self) -> ADC3AMEN_R {
        ADC3AMEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable"]
    #[inline(always)]
    pub fn bkpramamen(&self) -> BKPRAMAMEN_R {
        BKPRAMAMEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SRAM4 Autonomous mode enable"]
    #[inline(always)]
    pub fn sram4amen(&self) -> SRAM4AMEN_R {
        SRAM4AMEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BDMA and DMAMUX Autonomous mode enable"]
    #[inline(always)]
    pub fn bdmaamen(&mut self) -> BDMAAMEN_W {
        BDMAAMEN_W { w: self }
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable"]
    #[inline(always)]
    pub fn lpuart1amen(&mut self) -> LPUART1AMEN_W {
        LPUART1AMEN_W { w: self }
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable"]
    #[inline(always)]
    pub fn spi6amen(&mut self) -> SPI6AMEN_W {
        SPI6AMEN_W { w: self }
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable"]
    #[inline(always)]
    pub fn i2c4amen(&mut self) -> I2C4AMEN_W {
        I2C4AMEN_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim2amen(&mut self) -> LPTIM2AMEN_W {
        LPTIM2AMEN_W { w: self }
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim3amen(&mut self) -> LPTIM3AMEN_W {
        LPTIM3AMEN_W { w: self }
    }
    #[doc = "Bit 11 - LPTIM4 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim4amen(&mut self) -> LPTIM4AMEN_W {
        LPTIM4AMEN_W { w: self }
    }
    #[doc = "Bit 12 - LPTIM5 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim5amen(&mut self) -> LPTIM5AMEN_W {
        LPTIM5AMEN_W { w: self }
    }
    #[doc = "Bit 14 - COMP12 Autonomous mode enable"]
    #[inline(always)]
    pub fn comp12amen(&mut self) -> COMP12AMEN_W {
        COMP12AMEN_W { w: self }
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable"]
    #[inline(always)]
    pub fn vrefamen(&mut self) -> VREFAMEN_W {
        VREFAMEN_W { w: self }
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable"]
    #[inline(always)]
    pub fn rtcamen(&mut self) -> RTCAMEN_W {
        RTCAMEN_W { w: self }
    }
    #[doc = "Bit 19 - CRC Autonomous mode enable"]
    #[inline(always)]
    pub fn crcamen(&mut self) -> CRCAMEN_W {
        CRCAMEN_W { w: self }
    }
    #[doc = "Bit 21 - SAI4 Autonomous mode enable"]
    #[inline(always)]
    pub fn sai4amen(&mut self) -> SAI4AMEN_W {
        SAI4AMEN_W { w: self }
    }
    #[doc = "Bit 24 - ADC3 Autonomous mode enable"]
    #[inline(always)]
    pub fn adc3amen(&mut self) -> ADC3AMEN_W {
        ADC3AMEN_W { w: self }
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable"]
    #[inline(always)]
    pub fn bkpramamen(&mut self) -> BKPRAMAMEN_W {
        BKPRAMAMEN_W { w: self }
    }
    #[doc = "Bit 29 - SRAM4 Autonomous mode enable"]
    #[inline(always)]
    pub fn sram4amen(&mut self) -> SRAM4AMEN_W {
        SRAM4AMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC D3 Autonomous mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3amr](index.html) module"]
pub struct D3AMR_SPEC;
impl crate::RegisterSpec for D3AMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3amr::R](R) reader structure"]
impl crate::Readable for D3AMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3amr::W](W) writer structure"]
impl crate::Writable for D3AMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D3AMR to value 0"]
impl crate::Resettable for D3AMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
