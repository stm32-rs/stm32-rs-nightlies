#[doc = "Register `APB1SMENR` reader"]
pub struct R(crate::R<APB1SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1SMENR` writer"]
pub struct W(crate::W<APB1SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1SMENR_SPEC>;
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
impl From<crate::W<APB1SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low power timer clock enable during sleep mode bit"]
pub type LPTIM1SMEN_A = TIM2SMEN_A;
#[doc = "Field `LPTIM1SMEN` reader - Low power timer clock enable during sleep mode bit"]
pub type LPTIM1SMEN_R = TIM2SMEN_R;
#[doc = "Field `LPTIM1SMEN` writer - Low power timer clock enable during sleep mode bit"]
pub struct LPTIM1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM1SMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM1SMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "DAC interface clock enable during sleep mode bit"]
pub type DACSMEN_A = TIM2SMEN_A;
#[doc = "Field `DACSMEN` reader - DAC interface clock enable during sleep mode bit"]
pub type DACSMEN_R = TIM2SMEN_R;
#[doc = "Field `DACSMEN` writer - DAC interface clock enable during sleep mode bit"]
pub struct DACSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACSMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACSMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACSMEN_A::ENABLED)
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
#[doc = "Power interface clock enable during sleep mode bit"]
pub type PWRSMEN_A = TIM2SMEN_A;
#[doc = "Field `PWRSMEN` reader - Power interface clock enable during sleep mode bit"]
pub type PWRSMEN_R = TIM2SMEN_R;
#[doc = "Field `PWRSMEN` writer - Power interface clock enable during sleep mode bit"]
pub struct PWRSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWRSMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWRSMEN_A::ENABLED)
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
#[doc = "Clock recovery system clock enable during sleep mode bit"]
pub type CRSSMEN_A = TIM2SMEN_A;
#[doc = "Field `CRSSMEN` reader - Clock recovery system clock enable during sleep mode bit"]
pub type CRSSMEN_R = TIM2SMEN_R;
#[doc = "Field `CRSSMEN` writer - Clock recovery system clock enable during sleep mode bit"]
pub struct CRSSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSSMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRSSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRSSMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRSSMEN_A::ENABLED)
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
#[doc = "USB clock enable during sleep mode bit"]
pub type USBSMEN_A = TIM2SMEN_A;
#[doc = "Field `USBSMEN` reader - USB clock enable during sleep mode bit"]
pub type USBSMEN_R = TIM2SMEN_R;
#[doc = "Field `USBSMEN` writer - USB clock enable during sleep mode bit"]
pub struct USBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBSMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBSMEN_A::ENABLED)
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
#[doc = "I2C2 clock enable during sleep mode bit"]
pub type I2C2SMEN_A = TIM2SMEN_A;
#[doc = "Field `I2C2SMEN` reader - I2C2 clock enable during sleep mode bit"]
pub type I2C2SMEN_R = TIM2SMEN_R;
#[doc = "Field `I2C2SMEN` writer - I2C2 clock enable during sleep mode bit"]
pub struct I2C2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C2SMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C2SMEN_A::ENABLED)
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
#[doc = "I2C1 clock enable during sleep mode bit"]
pub type I2C1SMEN_A = TIM2SMEN_A;
#[doc = "Field `I2C1SMEN` reader - I2C1 clock enable during sleep mode bit"]
pub type I2C1SMEN_R = TIM2SMEN_R;
#[doc = "Field `I2C1SMEN` writer - I2C1 clock enable during sleep mode bit"]
pub struct I2C1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C1SMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C1SMEN_A::ENABLED)
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
#[doc = "LPUART1 clock enable during sleep mode bit"]
pub type LPUART1SMEN_A = TIM2SMEN_A;
#[doc = "Field `LPUART1SMEN` reader - LPUART1 clock enable during sleep mode bit"]
pub type LPUART1SMEN_R = TIM2SMEN_R;
#[doc = "Field `LPUART1SMEN` writer - LPUART1 clock enable during sleep mode bit"]
pub struct LPUART1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPUART1SMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPUART1SMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "UART2 clock enable during sleep mode bit"]
pub type USART2SMEN_A = TIM2SMEN_A;
#[doc = "Field `USART2SMEN` reader - UART2 clock enable during sleep mode bit"]
pub type USART2SMEN_R = TIM2SMEN_R;
#[doc = "Field `USART2SMEN` writer - UART2 clock enable during sleep mode bit"]
pub struct USART2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART2SMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART2SMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "SPI2 clock enable during sleep mode bit"]
pub type SPI2SMEN_A = TIM2SMEN_A;
#[doc = "Field `SPI2SMEN` reader - SPI2 clock enable during sleep mode bit"]
pub type SPI2SMEN_R = TIM2SMEN_R;
#[doc = "Field `SPI2SMEN` writer - SPI2 clock enable during sleep mode bit"]
pub struct SPI2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI2SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI2SMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI2SMEN_A::ENABLED)
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
#[doc = "Window watchdog clock enable during sleep mode bit"]
pub type WWDGSMEN_A = TIM2SMEN_A;
#[doc = "Field `WWDGSMEN` reader - Window watchdog clock enable during sleep mode bit"]
pub type WWDGSMEN_R = TIM2SMEN_R;
#[doc = "Field `WWDGSMEN` writer - Window watchdog clock enable during sleep mode bit"]
pub struct WWDGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGSMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDGSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WWDGSMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WWDGSMEN_A::ENABLED)
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
#[doc = "Timer 6 clock enable during sleep mode bit"]
pub type TIM6SMEN_A = TIM2SMEN_A;
#[doc = "Field `TIM6SMEN` reader - Timer 6 clock enable during sleep mode bit"]
pub type TIM6SMEN_R = TIM2SMEN_R;
#[doc = "Field `TIM6SMEN` writer - Timer 6 clock enable during sleep mode bit"]
pub struct TIM6SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM6SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM6SMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM6SMEN_A::ENABLED)
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
#[doc = "Timer2 clock enable during sleep mode bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2SMEN_A {
    #[doc = "0: Clock disabled"]
    DISABLED = 0,
    #[doc = "1: Clock enabled"]
    ENABLED = 1,
}
impl From<TIM2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2SMEN` reader - Timer2 clock enable during sleep mode bit"]
pub struct TIM2SMEN_R(crate::FieldReader<bool, TIM2SMEN_A>);
impl TIM2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2SMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2SMEN_A {
        match self.bits {
            false => TIM2SMEN_A::DISABLED,
            true => TIM2SMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TIM2SMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TIM2SMEN_A::ENABLED
    }
}
impl core::ops::Deref for TIM2SMEN_R {
    type Target = crate::FieldReader<bool, TIM2SMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2SMEN` writer - Timer2 clock enable during sleep mode bit"]
pub struct TIM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM2SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2SMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2SMEN_A::ENABLED)
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
#[doc = "Timer3 clock enable during Sleep mode bit"]
pub type TIM3SMEN_A = TIM2SMEN_A;
#[doc = "Field `TIM3SMEN` reader - Timer3 clock enable during Sleep mode bit"]
pub type TIM3SMEN_R = TIM2SMEN_R;
#[doc = "Field `TIM3SMEN` writer - Timer3 clock enable during Sleep mode bit"]
pub struct TIM3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM3SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM3SMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM3SMEN_A::ENABLED)
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
#[doc = "Timer 7 clock enable during Sleep mode bit"]
pub type TIM7SMEN_A = TIM2SMEN_A;
#[doc = "Field `TIM7SMEN` reader - Timer 7 clock enable during Sleep mode bit"]
pub type TIM7SMEN_R = TIM2SMEN_R;
#[doc = "Field `TIM7SMEN` writer - Timer 7 clock enable during Sleep mode bit"]
pub struct TIM7SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM7SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM7SMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM7SMEN_A::ENABLED)
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
#[doc = "USART4 clock enable during Sleep mode bit"]
pub type USART4SMEN_A = TIM2SMEN_A;
#[doc = "Field `USART4SMEN` reader - USART4 clock enable during Sleep mode bit"]
pub type USART4SMEN_R = TIM2SMEN_R;
#[doc = "Field `USART4SMEN` writer - USART4 clock enable during Sleep mode bit"]
pub struct USART4SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART4SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART4SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART4SMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART4SMEN_A::ENABLED)
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
#[doc = "USART5 clock enable during Sleep mode bit"]
pub type USART5SMEN_A = TIM2SMEN_A;
#[doc = "Field `USART5SMEN` reader - USART5 clock enable during Sleep mode bit"]
pub type USART5SMEN_R = TIM2SMEN_R;
#[doc = "Field `USART5SMEN` writer - USART5 clock enable during Sleep mode bit"]
pub struct USART5SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART5SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART5SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART5SMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART5SMEN_A::ENABLED)
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
#[doc = "2C3 clock enable during Sleep mode bit"]
pub type I2C3SMEN_A = TIM2SMEN_A;
#[doc = "Field `I2C3SMEN` reader - 2C3 clock enable during Sleep mode bit"]
pub type I2C3SMEN_R = TIM2SMEN_R;
#[doc = "Field `I2C3SMEN` writer - 2C3 clock enable during Sleep mode bit"]
pub struct I2C3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C3SMEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C3SMEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 31 - Low power timer clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DAC interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn dacsmen(&self) -> DACSMEN_R {
        DACSMEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Clock recovery system clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn crssmen(&self) -> CRSSMEN_R {
        CRSSMEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 23 - USB clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 18 - LPUART1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - UART2 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer 6 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Timer2 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer3 clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer 7 clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn usart4smen(&self) -> USART4SMEN_R {
        USART4SMEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - USART5 clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn usart5smen(&self) -> USART5SMEN_R {
        USART5SMEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 2C3 clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Low power timer clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W {
        LPTIM1SMEN_W { w: self }
    }
    #[doc = "Bit 29 - DAC interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn dacsmen(&mut self) -> DACSMEN_W {
        DACSMEN_W { w: self }
    }
    #[doc = "Bit 28 - Power interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W {
        PWRSMEN_W { w: self }
    }
    #[doc = "Bit 27 - Clock recovery system clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn crssmen(&mut self) -> CRSSMEN_W {
        CRSSMEN_W { w: self }
    }
    #[doc = "Bit 23 - USB clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn usbsmen(&mut self) -> USBSMEN_W {
        USBSMEN_W { w: self }
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W {
        I2C2SMEN_W { w: self }
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W {
        I2C1SMEN_W { w: self }
    }
    #[doc = "Bit 18 - LPUART1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W {
        LPUART1SMEN_W { w: self }
    }
    #[doc = "Bit 17 - UART2 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W {
        USART2SMEN_W { w: self }
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W {
        SPI2SMEN_W { w: self }
    }
    #[doc = "Bit 11 - Window watchdog clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W {
        WWDGSMEN_W { w: self }
    }
    #[doc = "Bit 4 - Timer 6 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W {
        TIM6SMEN_W { w: self }
    }
    #[doc = "Bit 0 - Timer2 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W {
        TIM2SMEN_W { w: self }
    }
    #[doc = "Bit 1 - Timer3 clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W {
        TIM3SMEN_W { w: self }
    }
    #[doc = "Bit 5 - Timer 7 clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W {
        TIM7SMEN_W { w: self }
    }
    #[doc = "Bit 19 - USART4 clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn usart4smen(&mut self) -> USART4SMEN_W {
        USART4SMEN_W { w: self }
    }
    #[doc = "Bit 20 - USART5 clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn usart5smen(&mut self) -> USART5SMEN_W {
        USART5SMEN_W { w: self }
    }
    #[doc = "Bit 30 - 2C3 clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W {
        I2C3SMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable in sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1smenr](index.html) module"]
pub struct APB1SMENR_SPEC;
impl crate::RegisterSpec for APB1SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1smenr::R](R) reader structure"]
impl crate::Readable for APB1SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1smenr::W](W) writer structure"]
impl crate::Writable for APB1SMENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1SMENR to value 0xb8e6_4a11"]
impl crate::Resettable for APB1SMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb8e6_4a11
    }
}
