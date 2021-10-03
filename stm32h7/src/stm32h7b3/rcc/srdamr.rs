#[doc = "Register `SRDAMR` reader"]
pub struct R(crate::R<SRDAMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRDAMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRDAMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRDAMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRDAMR` writer"]
pub struct W(crate::W<SRDAMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRDAMR_SPEC>;
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
impl From<crate::W<SRDAMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRDAMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDMA2AMEN_A {
    #[doc = "0: Clock disabled in autonomous mode"]
    DISABLED = 0,
    #[doc = "1: Clock enabled in autonomous mode"]
    ENABLED = 1,
}
impl From<BDMA2AMEN_A> for bool {
    #[inline(always)]
    fn from(variant: BDMA2AMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDMA2AMEN` reader - SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub struct BDMA2AMEN_R(crate::FieldReader<bool, BDMA2AMEN_A>);
impl BDMA2AMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BDMA2AMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDMA2AMEN_A {
        match self.bits {
            false => BDMA2AMEN_A::DISABLED,
            true => BDMA2AMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BDMA2AMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BDMA2AMEN_A::ENABLED
    }
}
impl core::ops::Deref for BDMA2AMEN_R {
    type Target = crate::FieldReader<bool, BDMA2AMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDMA2AMEN` writer - SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub struct BDMA2AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMA2AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDMA2AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BDMA2AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BDMA2AMEN_A::ENABLED)
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
#[doc = "GPIO Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type GPIOAMEN_A = BDMA2AMEN_A;
#[doc = "Field `GPIOAMEN` reader - GPIO Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type GPIOAMEN_R = BDMA2AMEN_R;
#[doc = "Field `GPIOAMEN` writer - GPIO Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub struct GPIOAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOAMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOAMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOAMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOAMEN_A::ENABLED)
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
#[doc = "LPUART1 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type LPUART1AMEN_A = BDMA2AMEN_A;
#[doc = "Field `LPUART1AMEN` reader - LPUART1 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type LPUART1AMEN_R = BDMA2AMEN_R;
#[doc = "Field `LPUART1AMEN` writer - LPUART1 Autonomous mode enable Set and reset by software. Refer to for additional information."]
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
#[doc = "SPI6 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type SPI6AMEN_A = BDMA2AMEN_A;
#[doc = "Field `SPI6AMEN` reader - SPI6 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type SPI6AMEN_R = BDMA2AMEN_R;
#[doc = "Field `SPI6AMEN` writer - SPI6 Autonomous mode enable Set and reset by software. Refer to for additional information."]
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
#[doc = "I2C4 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type I2C4AMEN_A = BDMA2AMEN_A;
#[doc = "Field `I2C4AMEN` reader - I2C4 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type I2C4AMEN_R = BDMA2AMEN_R;
#[doc = "Field `I2C4AMEN` writer - I2C4 Autonomous mode enable Set and reset by software. Refer to for additional information."]
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
#[doc = "LPTIM2 Autonomous mode enable Set and reset by software. Refer to for additional information"]
pub type LPTIM2AMEN_A = BDMA2AMEN_A;
#[doc = "Field `LPTIM2AMEN` reader - LPTIM2 Autonomous mode enable Set and reset by software. Refer to for additional information"]
pub type LPTIM2AMEN_R = BDMA2AMEN_R;
#[doc = "Field `LPTIM2AMEN` writer - LPTIM2 Autonomous mode enable Set and reset by software. Refer to for additional information"]
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
#[doc = "LPTIM3 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type LPTIM3AMEN_A = BDMA2AMEN_A;
#[doc = "Field `LPTIM3AMEN` reader - LPTIM3 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type LPTIM3AMEN_R = BDMA2AMEN_R;
#[doc = "Field `LPTIM3AMEN` writer - LPTIM3 Autonomous mode enable Set and reset by software. Refer to for additional information."]
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
#[doc = "DAC2 (containing one converter) Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type DAC2AMEN_A = BDMA2AMEN_A;
#[doc = "Field `DAC2AMEN` reader - DAC2 (containing one converter) Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type DAC2AMEN_R = BDMA2AMEN_R;
#[doc = "Field `DAC2AMEN` writer - DAC2 (containing one converter) Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub struct DAC2AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC2AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC2AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAC2AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAC2AMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "COMP1 and 2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type COMP12AMEN_A = BDMA2AMEN_A;
#[doc = "Field `COMP12AMEN` reader - COMP1 and 2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type COMP12AMEN_R = BDMA2AMEN_R;
#[doc = "Field `COMP12AMEN` writer - COMP1 and 2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
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
#[doc = "VREF Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type VREFAMEN_A = BDMA2AMEN_A;
#[doc = "Field `VREFAMEN` reader - VREF Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type VREFAMEN_R = BDMA2AMEN_R;
#[doc = "Field `VREFAMEN` writer - VREF Autonomous mode enable Set and reset by software. Refer to for additional information."]
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
#[doc = "RTC Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type RTCAMEN_A = BDMA2AMEN_A;
#[doc = "Field `RTCAMEN` reader - RTC Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type RTCAMEN_R = BDMA2AMEN_R;
#[doc = "Field `RTCAMEN` writer - RTC Autonomous mode enable Set and reset by software. Refer to for additional information."]
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
#[doc = "Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type DTSAMEN_A = BDMA2AMEN_A;
#[doc = "Field `DTSAMEN` reader - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type DTSAMEN_R = BDMA2AMEN_R;
#[doc = "Field `DTSAMEN` writer - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub struct DTSAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSAMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTSAMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTSAMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTSAMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "DFSDM2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type DFSDM2AMEN_A = BDMA2AMEN_A;
#[doc = "Field `DFSDM2AMEN` reader - DFSDM2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type DFSDM2AMEN_R = BDMA2AMEN_R;
#[doc = "Field `DFSDM2AMEN` writer - DFSDM2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub struct DFSDM2AMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM2AMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFSDM2AMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DFSDM2AMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DFSDM2AMEN_A::ENABLED)
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
#[doc = "Backup RAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type BKPRAMAMEN_A = BDMA2AMEN_A;
#[doc = "Field `BKPRAMAMEN` reader - Backup RAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type BKPRAMAMEN_R = BDMA2AMEN_R;
#[doc = "Field `BKPRAMAMEN` writer - Backup RAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
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
#[doc = "SmartRun domain SRAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type SRDSRAMAMEN_A = BDMA2AMEN_A;
#[doc = "Field `SRDSRAMAMEN` reader - SmartRun domain SRAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type SRDSRAMAMEN_R = BDMA2AMEN_R;
#[doc = "Field `SRDSRAMAMEN` writer - SmartRun domain SRAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub struct SRDSRAMAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRDSRAMAMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRDSRAMAMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRDSRAMAMEN_A::DISABLED)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRDSRAMAMEN_A::ENABLED)
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
    #[doc = "Bit 0 - SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn bdma2amen(&self) -> BDMA2AMEN_R {
        BDMA2AMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn gpioamen(&self) -> GPIOAMEN_R {
        GPIOAMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn lpuart1amen(&self) -> LPUART1AMEN_R {
        LPUART1AMEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn spi6amen(&self) -> SPI6AMEN_R {
        SPI6AMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn i2c4amen(&self) -> I2C4AMEN_R {
        I2C4AMEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable Set and reset by software. Refer to for additional information"]
    #[inline(always)]
    pub fn lptim2amen(&self) -> LPTIM2AMEN_R {
        LPTIM2AMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn lptim3amen(&self) -> LPTIM3AMEN_R {
        LPTIM3AMEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DAC2 (containing one converter) Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn dac2amen(&self) -> DAC2AMEN_R {
        DAC2AMEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - COMP1 and 2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn comp12amen(&self) -> COMP12AMEN_R {
        COMP12AMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn vrefamen(&self) -> VREFAMEN_R {
        VREFAMEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn rtcamen(&self) -> RTCAMEN_R {
        RTCAMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn dtsamen(&self) -> DTSAMEN_R {
        DTSAMEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DFSDM2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn dfsdm2amen(&self) -> DFSDM2AMEN_R {
        DFSDM2AMEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn bkpramamen(&self) -> BKPRAMAMEN_R {
        BKPRAMAMEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SmartRun domain SRAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn srdsramamen(&self) -> SRDSRAMAMEN_R {
        SRDSRAMAMEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn bdma2amen(&mut self) -> BDMA2AMEN_W {
        BDMA2AMEN_W { w: self }
    }
    #[doc = "Bit 1 - GPIO Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn gpioamen(&mut self) -> GPIOAMEN_W {
        GPIOAMEN_W { w: self }
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn lpuart1amen(&mut self) -> LPUART1AMEN_W {
        LPUART1AMEN_W { w: self }
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn spi6amen(&mut self) -> SPI6AMEN_W {
        SPI6AMEN_W { w: self }
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn i2c4amen(&mut self) -> I2C4AMEN_W {
        I2C4AMEN_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable Set and reset by software. Refer to for additional information"]
    #[inline(always)]
    pub fn lptim2amen(&mut self) -> LPTIM2AMEN_W {
        LPTIM2AMEN_W { w: self }
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn lptim3amen(&mut self) -> LPTIM3AMEN_W {
        LPTIM3AMEN_W { w: self }
    }
    #[doc = "Bit 13 - DAC2 (containing one converter) Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn dac2amen(&mut self) -> DAC2AMEN_W {
        DAC2AMEN_W { w: self }
    }
    #[doc = "Bit 14 - COMP1 and 2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn comp12amen(&mut self) -> COMP12AMEN_W {
        COMP12AMEN_W { w: self }
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn vrefamen(&mut self) -> VREFAMEN_W {
        VREFAMEN_W { w: self }
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn rtcamen(&mut self) -> RTCAMEN_W {
        RTCAMEN_W { w: self }
    }
    #[doc = "Bit 26 - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn dtsamen(&mut self) -> DTSAMEN_W {
        DTSAMEN_W { w: self }
    }
    #[doc = "Bit 27 - DFSDM2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn dfsdm2amen(&mut self) -> DFSDM2AMEN_W {
        DFSDM2AMEN_W { w: self }
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn bkpramamen(&mut self) -> BKPRAMAMEN_W {
        BKPRAMAMEN_W { w: self }
    }
    #[doc = "Bit 29 - SmartRun domain SRAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn srdsramamen(&mut self) -> SRDSRAMAMEN_W {
        SRDSRAMAMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC SmartRun domain Autonomous mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdamr](index.html) module"]
pub struct SRDAMR_SPEC;
impl crate::RegisterSpec for SRDAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srdamr::R](R) reader structure"]
impl crate::Readable for SRDAMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srdamr::W](W) writer structure"]
impl crate::Writable for SRDAMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRDAMR to value 0"]
impl crate::Resettable for SRDAMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
