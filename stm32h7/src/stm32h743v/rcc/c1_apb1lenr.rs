#[doc = "Register `C1_APB1LENR` reader"]
pub struct R(crate::R<C1_APB1LENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_APB1LENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_APB1LENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_APB1LENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_APB1LENR` writer"]
pub struct W(crate::W<C1_APB1LENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_APB1LENR_SPEC>;
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
impl From<crate::W<C1_APB1LENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_APB1LENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TIM peripheral clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2EN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<TIM2EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2EN` reader - TIM peripheral clock enable"]
pub struct TIM2EN_R(crate::FieldReader<bool, TIM2EN_A>);
impl TIM2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2EN_A {
        match self.bits {
            false => TIM2EN_A::DISABLED,
            true => TIM2EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TIM2EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TIM2EN_A::ENABLED
    }
}
impl core::ops::Deref for TIM2EN_R {
    type Target = crate::FieldReader<bool, TIM2EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2EN` writer - TIM peripheral clock enable"]
pub struct TIM2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2EN_A::ENABLED)
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
#[doc = "TIM peripheral clock enable"]
pub type TIM3EN_A = TIM2EN_A;
#[doc = "Field `TIM3EN` reader - TIM peripheral clock enable"]
pub type TIM3EN_R = TIM2EN_R;
#[doc = "Field `TIM3EN` writer - TIM peripheral clock enable"]
pub struct TIM3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM3EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM3EN_A::ENABLED)
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
#[doc = "TIM peripheral clock enable"]
pub type TIM4EN_A = TIM2EN_A;
#[doc = "Field `TIM4EN` reader - TIM peripheral clock enable"]
pub type TIM4EN_R = TIM2EN_R;
#[doc = "Field `TIM4EN` writer - TIM peripheral clock enable"]
pub struct TIM4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM4EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM4EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM4EN_A::ENABLED)
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
#[doc = "TIM peripheral clock enable"]
pub type TIM5EN_A = TIM2EN_A;
#[doc = "Field `TIM5EN` reader - TIM peripheral clock enable"]
pub type TIM5EN_R = TIM2EN_R;
#[doc = "Field `TIM5EN` writer - TIM peripheral clock enable"]
pub struct TIM5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM5EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM5EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM5EN_A::ENABLED)
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
#[doc = "TIM peripheral clock enable"]
pub type TIM6EN_A = TIM2EN_A;
#[doc = "Field `TIM6EN` reader - TIM peripheral clock enable"]
pub type TIM6EN_R = TIM2EN_R;
#[doc = "Field `TIM6EN` writer - TIM peripheral clock enable"]
pub struct TIM6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM6EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM6EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM6EN_A::ENABLED)
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
#[doc = "TIM peripheral clock enable"]
pub type TIM7EN_A = TIM2EN_A;
#[doc = "Field `TIM7EN` reader - TIM peripheral clock enable"]
pub type TIM7EN_R = TIM2EN_R;
#[doc = "Field `TIM7EN` writer - TIM peripheral clock enable"]
pub struct TIM7EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM7EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM7EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM7EN_A::ENABLED)
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
#[doc = "TIM peripheral clock enable"]
pub type TIM12EN_A = TIM2EN_A;
#[doc = "Field `TIM12EN` reader - TIM peripheral clock enable"]
pub type TIM12EN_R = TIM2EN_R;
#[doc = "Field `TIM12EN` writer - TIM peripheral clock enable"]
pub struct TIM12EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM12EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM12EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM12EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM12EN_A::ENABLED)
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
#[doc = "TIM peripheral clock enable"]
pub type TIM13EN_A = TIM2EN_A;
#[doc = "Field `TIM13EN` reader - TIM peripheral clock enable"]
pub type TIM13EN_R = TIM2EN_R;
#[doc = "Field `TIM13EN` writer - TIM peripheral clock enable"]
pub struct TIM13EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM13EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM13EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM13EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM13EN_A::ENABLED)
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
#[doc = "TIM peripheral clock enable"]
pub type TIM14EN_A = TIM2EN_A;
#[doc = "Field `TIM14EN` reader - TIM peripheral clock enable"]
pub type TIM14EN_R = TIM2EN_R;
#[doc = "Field `TIM14EN` writer - TIM peripheral clock enable"]
pub struct TIM14EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM14EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM14EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM14EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM14EN_A::ENABLED)
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
#[doc = "LPTIM1 Peripheral Clocks Enable"]
pub type LPTIM1EN_A = TIM2EN_A;
#[doc = "Field `LPTIM1EN` reader - LPTIM1 Peripheral Clocks Enable"]
pub type LPTIM1EN_R = TIM2EN_R;
#[doc = "Field `LPTIM1EN` writer - LPTIM1 Peripheral Clocks Enable"]
pub struct LPTIM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM1EN_A::ENABLED)
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
#[doc = "SPI2 Peripheral Clocks Enable"]
pub type SPI2EN_A = TIM2EN_A;
#[doc = "Field `SPI2EN` reader - SPI2 Peripheral Clocks Enable"]
pub type SPI2EN_R = TIM2EN_R;
#[doc = "Field `SPI2EN` writer - SPI2 Peripheral Clocks Enable"]
pub struct SPI2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI2EN_A::ENABLED)
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
#[doc = "SPI3 Peripheral Clocks Enable"]
pub type SPI3EN_A = TIM2EN_A;
#[doc = "Field `SPI3EN` reader - SPI3 Peripheral Clocks Enable"]
pub type SPI3EN_R = TIM2EN_R;
#[doc = "Field `SPI3EN` writer - SPI3 Peripheral Clocks Enable"]
pub struct SPI3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI3EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI3EN_A::ENABLED)
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
#[doc = "SPDIFRX Peripheral Clocks Enable"]
pub type SPDIFRXEN_A = TIM2EN_A;
#[doc = "Field `SPDIFRXEN` reader - SPDIFRX Peripheral Clocks Enable"]
pub type SPDIFRXEN_R = TIM2EN_R;
#[doc = "Field `SPDIFRXEN` writer - SPDIFRX Peripheral Clocks Enable"]
pub struct SPDIFRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIFRXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDIFRXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPDIFRXEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPDIFRXEN_A::ENABLED)
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
#[doc = "USART2 Peripheral Clocks Enable"]
pub type USART2EN_A = TIM2EN_A;
#[doc = "Field `USART2EN` reader - USART2 Peripheral Clocks Enable"]
pub type USART2EN_R = TIM2EN_R;
#[doc = "Field `USART2EN` writer - USART2 Peripheral Clocks Enable"]
pub struct USART2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART2EN_A::ENABLED)
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
#[doc = "USART3 Peripheral Clocks Enable"]
pub type USART3EN_A = TIM2EN_A;
#[doc = "Field `USART3EN` reader - USART3 Peripheral Clocks Enable"]
pub type USART3EN_R = TIM2EN_R;
#[doc = "Field `USART3EN` writer - USART3 Peripheral Clocks Enable"]
pub struct USART3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART3EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART3EN_A::ENABLED)
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
#[doc = "UART4 Peripheral Clocks Enable"]
pub type UART4EN_A = TIM2EN_A;
#[doc = "Field `UART4EN` reader - UART4 Peripheral Clocks Enable"]
pub type UART4EN_R = TIM2EN_R;
#[doc = "Field `UART4EN` writer - UART4 Peripheral Clocks Enable"]
pub struct UART4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART4EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UART4EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UART4EN_A::ENABLED)
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
#[doc = "UART5 Peripheral Clocks Enable"]
pub type UART5EN_A = TIM2EN_A;
#[doc = "Field `UART5EN` reader - UART5 Peripheral Clocks Enable"]
pub type UART5EN_R = TIM2EN_R;
#[doc = "Field `UART5EN` writer - UART5 Peripheral Clocks Enable"]
pub struct UART5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART5EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UART5EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UART5EN_A::ENABLED)
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
#[doc = "I2C1 Peripheral Clocks Enable"]
pub type I2C1EN_A = TIM2EN_A;
#[doc = "Field `I2C1EN` reader - I2C1 Peripheral Clocks Enable"]
pub type I2C1EN_R = TIM2EN_R;
#[doc = "Field `I2C1EN` writer - I2C1 Peripheral Clocks Enable"]
pub struct I2C1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C1EN_A::ENABLED)
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
#[doc = "I2C2 Peripheral Clocks Enable"]
pub type I2C2EN_A = TIM2EN_A;
#[doc = "Field `I2C2EN` reader - I2C2 Peripheral Clocks Enable"]
pub type I2C2EN_R = TIM2EN_R;
#[doc = "Field `I2C2EN` writer - I2C2 Peripheral Clocks Enable"]
pub struct I2C2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C2EN_A::ENABLED)
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
#[doc = "I2C3 Peripheral Clocks Enable"]
pub type I2C3EN_A = TIM2EN_A;
#[doc = "Field `I2C3EN` reader - I2C3 Peripheral Clocks Enable"]
pub type I2C3EN_R = TIM2EN_R;
#[doc = "Field `I2C3EN` writer - I2C3 Peripheral Clocks Enable"]
pub struct I2C3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C3EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C3EN_A::ENABLED)
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
#[doc = "HDMI-CEC peripheral clock enable"]
pub type CECEN_A = TIM2EN_A;
#[doc = "Field `CECEN` reader - HDMI-CEC peripheral clock enable"]
pub type CECEN_R = TIM2EN_R;
#[doc = "Field `CECEN` writer - HDMI-CEC peripheral clock enable"]
pub struct CECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CECEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CECEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CECEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CECEN_A::ENABLED)
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
#[doc = "DAC1&2 peripheral clock enable"]
pub type DAC12EN_A = TIM2EN_A;
#[doc = "Field `DAC12EN` reader - DAC1&2 peripheral clock enable"]
pub type DAC12EN_R = TIM2EN_R;
#[doc = "Field `DAC12EN` writer - DAC1&2 peripheral clock enable"]
pub struct DAC12EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC12EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC12EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAC12EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAC12EN_A::ENABLED)
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
#[doc = "UART7 Peripheral Clocks Enable"]
pub type UART7EN_A = TIM2EN_A;
#[doc = "Field `UART7EN` reader - UART7 Peripheral Clocks Enable"]
pub type UART7EN_R = TIM2EN_R;
#[doc = "Field `UART7EN` writer - UART7 Peripheral Clocks Enable"]
pub struct UART7EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART7EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART7EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UART7EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UART7EN_A::ENABLED)
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
#[doc = "UART8 Peripheral Clocks Enable"]
pub type UART8EN_A = TIM2EN_A;
#[doc = "Field `UART8EN` reader - UART8 Peripheral Clocks Enable"]
pub type UART8EN_R = TIM2EN_R;
#[doc = "Field `UART8EN` writer - UART8 Peripheral Clocks Enable"]
pub struct UART8EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART8EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART8EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UART8EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UART8EN_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim12en(&self) -> TIM12EN_R {
        TIM12EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim13en(&self) -> TIM13EN_R {
        TIM13EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SPI3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPDIFRX Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spdifrxen(&self) -> SPDIFRXEN_R {
        SPDIFRXEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USART2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USART3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - UART4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - UART5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - I2C3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 27 - HDMI-CEC peripheral clock enable"]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DAC1&2 peripheral clock enable"]
    #[inline(always)]
    pub fn dac12en(&self) -> DAC12EN_R {
        DAC12EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - UART7 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart7en(&self) -> UART7EN_R {
        UART7EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - UART8 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart8en(&self) -> UART8EN_R {
        UART8EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W {
        TIM2EN_W { w: self }
    }
    #[doc = "Bit 1 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W {
        TIM3EN_W { w: self }
    }
    #[doc = "Bit 2 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim4en(&mut self) -> TIM4EN_W {
        TIM4EN_W { w: self }
    }
    #[doc = "Bit 3 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim5en(&mut self) -> TIM5EN_W {
        TIM5EN_W { w: self }
    }
    #[doc = "Bit 4 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W {
        TIM6EN_W { w: self }
    }
    #[doc = "Bit 5 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim7en(&mut self) -> TIM7EN_W {
        TIM7EN_W { w: self }
    }
    #[doc = "Bit 6 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim12en(&mut self) -> TIM12EN_W {
        TIM12EN_W { w: self }
    }
    #[doc = "Bit 7 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim13en(&mut self) -> TIM13EN_W {
        TIM13EN_W { w: self }
    }
    #[doc = "Bit 8 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim14en(&mut self) -> TIM14EN_W {
        TIM14EN_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W {
        LPTIM1EN_W { w: self }
    }
    #[doc = "Bit 14 - SPI2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W {
        SPI2EN_W { w: self }
    }
    #[doc = "Bit 15 - SPI3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W {
        SPI3EN_W { w: self }
    }
    #[doc = "Bit 16 - SPDIFRX Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spdifrxen(&mut self) -> SPDIFRXEN_W {
        SPDIFRXEN_W { w: self }
    }
    #[doc = "Bit 17 - USART2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W {
        USART2EN_W { w: self }
    }
    #[doc = "Bit 18 - USART3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart3en(&mut self) -> USART3EN_W {
        USART3EN_W { w: self }
    }
    #[doc = "Bit 19 - UART4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart4en(&mut self) -> UART4EN_W {
        UART4EN_W { w: self }
    }
    #[doc = "Bit 20 - UART5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart5en(&mut self) -> UART5EN_W {
        UART5EN_W { w: self }
    }
    #[doc = "Bit 21 - I2C1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W {
        I2C1EN_W { w: self }
    }
    #[doc = "Bit 22 - I2C2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W {
        I2C2EN_W { w: self }
    }
    #[doc = "Bit 23 - I2C3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W {
        I2C3EN_W { w: self }
    }
    #[doc = "Bit 27 - HDMI-CEC peripheral clock enable"]
    #[inline(always)]
    pub fn cecen(&mut self) -> CECEN_W {
        CECEN_W { w: self }
    }
    #[doc = "Bit 29 - DAC1&2 peripheral clock enable"]
    #[inline(always)]
    pub fn dac12en(&mut self) -> DAC12EN_W {
        DAC12EN_W { w: self }
    }
    #[doc = "Bit 30 - UART7 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart7en(&mut self) -> UART7EN_W {
        UART7EN_W { w: self }
    }
    #[doc = "Bit 31 - UART8 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart8en(&mut self) -> UART8EN_W {
        UART8EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb1lenr](index.html) module"]
pub struct C1_APB1LENR_SPEC;
impl crate::RegisterSpec for C1_APB1LENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_apb1lenr::R](R) reader structure"]
impl crate::Readable for C1_APB1LENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_apb1lenr::W](W) writer structure"]
impl crate::Writable for C1_APB1LENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1_APB1LENR to value 0"]
impl crate::Resettable for C1_APB1LENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
