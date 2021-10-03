#[doc = "Register `APB1LRSTR` reader"]
pub struct R(crate::R<APB1LRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1LRSTR` writer"]
pub struct W(crate::W<APB1LRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LRSTR_SPEC>;
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
impl From<crate::W<APB1LRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TIM block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2RST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<TIM2RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2RST` reader - TIM block reset"]
pub struct TIM2RST_R(crate::FieldReader<bool, TIM2RST_A>);
impl TIM2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIM2RST_A> {
        match self.bits {
            true => Some(TIM2RST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == TIM2RST_A::RESET
    }
}
impl core::ops::Deref for TIM2RST_R {
    type Target = crate::FieldReader<bool, TIM2RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2RST` writer - TIM block reset"]
pub struct TIM2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::RESET)
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
#[doc = "TIM block reset"]
pub type TIM3RST_A = TIM2RST_A;
#[doc = "Field `TIM3RST` reader - TIM block reset"]
pub type TIM3RST_R = TIM2RST_R;
#[doc = "Field `TIM3RST` writer - TIM block reset"]
pub struct TIM3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM3RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM3RST_A::RESET)
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
#[doc = "TIM block reset"]
pub type TIM4RST_A = TIM2RST_A;
#[doc = "Field `TIM4RST` reader - TIM block reset"]
pub type TIM4RST_R = TIM2RST_R;
#[doc = "Field `TIM4RST` writer - TIM block reset"]
pub struct TIM4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM4RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM4RST_A::RESET)
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
#[doc = "TIM block reset"]
pub type TIM5RST_A = TIM2RST_A;
#[doc = "Field `TIM5RST` reader - TIM block reset"]
pub type TIM5RST_R = TIM2RST_R;
#[doc = "Field `TIM5RST` writer - TIM block reset"]
pub struct TIM5RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM5RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM5RST_A::RESET)
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
#[doc = "TIM block reset"]
pub type TIM6RST_A = TIM2RST_A;
#[doc = "Field `TIM6RST` reader - TIM block reset"]
pub type TIM6RST_R = TIM2RST_R;
#[doc = "Field `TIM6RST` writer - TIM block reset"]
pub struct TIM6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM6RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM6RST_A::RESET)
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
#[doc = "TIM block reset"]
pub type TIM7RST_A = TIM2RST_A;
#[doc = "Field `TIM7RST` reader - TIM block reset"]
pub type TIM7RST_R = TIM2RST_R;
#[doc = "Field `TIM7RST` writer - TIM block reset"]
pub struct TIM7RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM7RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM7RST_A::RESET)
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
#[doc = "TIM block reset"]
pub type TIM12RST_A = TIM2RST_A;
#[doc = "Field `TIM12RST` reader - TIM block reset"]
pub type TIM12RST_R = TIM2RST_R;
#[doc = "Field `TIM12RST` writer - TIM block reset"]
pub struct TIM12RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM12RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM12RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM12RST_A::RESET)
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
#[doc = "TIM block reset"]
pub type TIM13RST_A = TIM2RST_A;
#[doc = "Field `TIM13RST` reader - TIM block reset"]
pub type TIM13RST_R = TIM2RST_R;
#[doc = "Field `TIM13RST` writer - TIM block reset"]
pub struct TIM13RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM13RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM13RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM13RST_A::RESET)
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
#[doc = "TIM block reset"]
pub type TIM14RST_A = TIM2RST_A;
#[doc = "Field `TIM14RST` reader - TIM block reset"]
pub type TIM14RST_R = TIM2RST_R;
#[doc = "Field `TIM14RST` writer - TIM block reset"]
pub struct TIM14RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM14RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM14RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM14RST_A::RESET)
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
#[doc = "TIM block reset"]
pub type LPTIM1RST_A = TIM2RST_A;
#[doc = "Field `LPTIM1RST` reader - TIM block reset"]
pub type LPTIM1RST_R = TIM2RST_R;
#[doc = "Field `LPTIM1RST` writer - TIM block reset"]
pub struct LPTIM1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPTIM1RST_A::RESET)
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
#[doc = "SPI2 block reset"]
pub type SPI2RST_A = TIM2RST_A;
#[doc = "Field `SPI2RST` reader - SPI2 block reset"]
pub type SPI2RST_R = TIM2RST_R;
#[doc = "Field `SPI2RST` writer - SPI2 block reset"]
pub struct SPI2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI2RST_A::RESET)
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
#[doc = "SPI3 block reset"]
pub type SPI3RST_A = TIM2RST_A;
#[doc = "Field `SPI3RST` reader - SPI3 block reset"]
pub type SPI3RST_R = TIM2RST_R;
#[doc = "Field `SPI3RST` writer - SPI3 block reset"]
pub struct SPI3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI3RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI3RST_A::RESET)
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
#[doc = "SPDIFRX block reset"]
pub type SPDIFRXRST_A = TIM2RST_A;
#[doc = "Field `SPDIFRXRST` reader - SPDIFRX block reset"]
pub type SPDIFRXRST_R = TIM2RST_R;
#[doc = "Field `SPDIFRXRST` writer - SPDIFRX block reset"]
pub struct SPDIFRXRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIFRXRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDIFRXRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPDIFRXRST_A::RESET)
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
#[doc = "USART2 block reset"]
pub type USART2RST_A = TIM2RST_A;
#[doc = "Field `USART2RST` reader - USART2 block reset"]
pub type USART2RST_R = TIM2RST_R;
#[doc = "Field `USART2RST` writer - USART2 block reset"]
pub struct USART2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART2RST_A::RESET)
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
#[doc = "USART3 block reset"]
pub type USART3RST_A = TIM2RST_A;
#[doc = "Field `USART3RST` reader - USART3 block reset"]
pub type USART3RST_R = TIM2RST_R;
#[doc = "Field `USART3RST` writer - USART3 block reset"]
pub struct USART3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART3RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART3RST_A::RESET)
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
#[doc = "UART4 block reset"]
pub type UART4RST_A = TIM2RST_A;
#[doc = "Field `UART4RST` reader - UART4 block reset"]
pub type UART4RST_R = TIM2RST_R;
#[doc = "Field `UART4RST` writer - UART4 block reset"]
pub struct UART4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART4RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(UART4RST_A::RESET)
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
#[doc = "UART5 block reset"]
pub type UART5RST_A = TIM2RST_A;
#[doc = "Field `UART5RST` reader - UART5 block reset"]
pub type UART5RST_R = TIM2RST_R;
#[doc = "Field `UART5RST` writer - UART5 block reset"]
pub struct UART5RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART5RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(UART5RST_A::RESET)
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
#[doc = "I2C1 block reset"]
pub type I2C1RST_A = TIM2RST_A;
#[doc = "Field `I2C1RST` reader - I2C1 block reset"]
pub type I2C1RST_R = TIM2RST_R;
#[doc = "Field `I2C1RST` writer - I2C1 block reset"]
pub struct I2C1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C1RST_A::RESET)
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
#[doc = "I2C2 block reset"]
pub type I2C2RST_A = TIM2RST_A;
#[doc = "Field `I2C2RST` reader - I2C2 block reset"]
pub type I2C2RST_R = TIM2RST_R;
#[doc = "Field `I2C2RST` writer - I2C2 block reset"]
pub struct I2C2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C2RST_A::RESET)
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
#[doc = "I2C3 block reset"]
pub type I2C3RST_A = TIM2RST_A;
#[doc = "Field `I2C3RST` reader - I2C3 block reset"]
pub type I2C3RST_R = TIM2RST_R;
#[doc = "Field `I2C3RST` writer - I2C3 block reset"]
pub struct I2C3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C3RST_A::RESET)
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
#[doc = "HDMI-CEC block reset"]
pub type CECRST_A = TIM2RST_A;
#[doc = "Field `CECRST` reader - HDMI-CEC block reset"]
pub type CECRST_R = TIM2RST_R;
#[doc = "Field `CECRST` writer - HDMI-CEC block reset"]
pub struct CECRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CECRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CECRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CECRST_A::RESET)
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
#[doc = "DAC1 and 2 Blocks Reset"]
pub type DAC12RST_A = TIM2RST_A;
#[doc = "Field `DAC12RST` reader - DAC1 and 2 Blocks Reset"]
pub type DAC12RST_R = TIM2RST_R;
#[doc = "Field `DAC12RST` writer - DAC1 and 2 Blocks Reset"]
pub struct DAC12RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC12RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC12RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DAC12RST_A::RESET)
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
#[doc = "UART7 block reset"]
pub type UART7RST_A = TIM2RST_A;
#[doc = "Field `UART7RST` reader - UART7 block reset"]
pub type UART7RST_R = TIM2RST_R;
#[doc = "Field `UART7RST` writer - UART7 block reset"]
pub struct UART7RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART7RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART7RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(UART7RST_A::RESET)
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
#[doc = "UART8 block reset"]
pub type UART8RST_A = TIM2RST_A;
#[doc = "Field `UART8RST` reader - UART8 block reset"]
pub type UART8RST_R = TIM2RST_R;
#[doc = "Field `UART8RST` writer - UART8 block reset"]
pub struct UART8RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART8RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART8RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(UART8RST_A::RESET)
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
#[doc = "I2C5 block reset"]
pub type I2C5RST_A = TIM2RST_A;
#[doc = "Field `I2C5RST` reader - I2C5 block reset"]
pub type I2C5RST_R = TIM2RST_R;
#[doc = "Field `I2C5RST` writer - I2C5 block reset"]
pub struct I2C5RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C5RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C5RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C5RST_A::RESET)
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
impl R {
    #[doc = "Bit 0 - TIM block reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM block reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM block reset"]
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM block reset"]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM block reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM block reset"]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TIM block reset"]
    #[inline(always)]
    pub fn tim12rst(&self) -> TIM12RST_R {
        TIM12RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TIM block reset"]
    #[inline(always)]
    pub fn tim13rst(&self) -> TIM13RST_R {
        TIM13RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIM block reset"]
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TIM block reset"]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI2 block reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SPI3 block reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPDIFRX block reset"]
    #[inline(always)]
    pub fn spdifrxrst(&self) -> SPDIFRXRST_R {
        SPDIFRXRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USART2 block reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USART3 block reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - UART4 block reset"]
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - UART5 block reset"]
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1 block reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C2 block reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - I2C3 block reset"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 27 - HDMI-CEC block reset"]
    #[inline(always)]
    pub fn cecrst(&self) -> CECRST_R {
        CECRST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DAC1 and 2 Blocks Reset"]
    #[inline(always)]
    pub fn dac12rst(&self) -> DAC12RST_R {
        DAC12RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - UART7 block reset"]
    #[inline(always)]
    pub fn uart7rst(&self) -> UART7RST_R {
        UART7RST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - UART8 block reset"]
    #[inline(always)]
    pub fn uart8rst(&self) -> UART8RST_R {
        UART8RST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 25 - I2C5 block reset"]
    #[inline(always)]
    pub fn i2c5rst(&self) -> I2C5RST_R {
        I2C5RST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM block reset"]
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W {
        TIM2RST_W { w: self }
    }
    #[doc = "Bit 1 - TIM block reset"]
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W {
        TIM3RST_W { w: self }
    }
    #[doc = "Bit 2 - TIM block reset"]
    #[inline(always)]
    pub fn tim4rst(&mut self) -> TIM4RST_W {
        TIM4RST_W { w: self }
    }
    #[doc = "Bit 3 - TIM block reset"]
    #[inline(always)]
    pub fn tim5rst(&mut self) -> TIM5RST_W {
        TIM5RST_W { w: self }
    }
    #[doc = "Bit 4 - TIM block reset"]
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W {
        TIM6RST_W { w: self }
    }
    #[doc = "Bit 5 - TIM block reset"]
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W {
        TIM7RST_W { w: self }
    }
    #[doc = "Bit 6 - TIM block reset"]
    #[inline(always)]
    pub fn tim12rst(&mut self) -> TIM12RST_W {
        TIM12RST_W { w: self }
    }
    #[doc = "Bit 7 - TIM block reset"]
    #[inline(always)]
    pub fn tim13rst(&mut self) -> TIM13RST_W {
        TIM13RST_W { w: self }
    }
    #[doc = "Bit 8 - TIM block reset"]
    #[inline(always)]
    pub fn tim14rst(&mut self) -> TIM14RST_W {
        TIM14RST_W { w: self }
    }
    #[doc = "Bit 9 - TIM block reset"]
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W {
        LPTIM1RST_W { w: self }
    }
    #[doc = "Bit 14 - SPI2 block reset"]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W {
        SPI2RST_W { w: self }
    }
    #[doc = "Bit 15 - SPI3 block reset"]
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W {
        SPI3RST_W { w: self }
    }
    #[doc = "Bit 16 - SPDIFRX block reset"]
    #[inline(always)]
    pub fn spdifrxrst(&mut self) -> SPDIFRXRST_W {
        SPDIFRXRST_W { w: self }
    }
    #[doc = "Bit 17 - USART2 block reset"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W {
        USART2RST_W { w: self }
    }
    #[doc = "Bit 18 - USART3 block reset"]
    #[inline(always)]
    pub fn usart3rst(&mut self) -> USART3RST_W {
        USART3RST_W { w: self }
    }
    #[doc = "Bit 19 - UART4 block reset"]
    #[inline(always)]
    pub fn uart4rst(&mut self) -> UART4RST_W {
        UART4RST_W { w: self }
    }
    #[doc = "Bit 20 - UART5 block reset"]
    #[inline(always)]
    pub fn uart5rst(&mut self) -> UART5RST_W {
        UART5RST_W { w: self }
    }
    #[doc = "Bit 21 - I2C1 block reset"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W {
        I2C1RST_W { w: self }
    }
    #[doc = "Bit 22 - I2C2 block reset"]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W {
        I2C2RST_W { w: self }
    }
    #[doc = "Bit 23 - I2C3 block reset"]
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W {
        I2C3RST_W { w: self }
    }
    #[doc = "Bit 27 - HDMI-CEC block reset"]
    #[inline(always)]
    pub fn cecrst(&mut self) -> CECRST_W {
        CECRST_W { w: self }
    }
    #[doc = "Bit 29 - DAC1 and 2 Blocks Reset"]
    #[inline(always)]
    pub fn dac12rst(&mut self) -> DAC12RST_W {
        DAC12RST_W { w: self }
    }
    #[doc = "Bit 30 - UART7 block reset"]
    #[inline(always)]
    pub fn uart7rst(&mut self) -> UART7RST_W {
        UART7RST_W { w: self }
    }
    #[doc = "Bit 31 - UART8 block reset"]
    #[inline(always)]
    pub fn uart8rst(&mut self) -> UART8RST_W {
        UART8RST_W { w: self }
    }
    #[doc = "Bit 25 - I2C5 block reset"]
    #[inline(always)]
    pub fn i2c5rst(&mut self) -> I2C5RST_W {
        I2C5RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lrstr](index.html) module"]
pub struct APB1LRSTR_SPEC;
impl crate::RegisterSpec for APB1LRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1lrstr::R](R) reader structure"]
impl crate::Readable for APB1LRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1lrstr::W](W) writer structure"]
impl crate::Writable for APB1LRSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1LRSTR to value 0"]
impl crate::Resettable for APB1LRSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
