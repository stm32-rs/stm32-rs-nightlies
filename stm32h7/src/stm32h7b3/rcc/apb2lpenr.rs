#[doc = "Register `APB2LPENR` reader"]
pub struct R(crate::R<APB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2LPENR` writer"]
pub struct W(crate::W<APB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2LPENR_SPEC>;
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
impl From<crate::W<APB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TIM1 peripheral clock enable during CSleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1LPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    ENABLED = 1,
}
impl From<TIM1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1LPEN` reader - TIM1 peripheral clock enable during CSleep mode Set and reset by software."]
pub struct TIM1LPEN_R(crate::FieldReader<bool, TIM1LPEN_A>);
impl TIM1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1LPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1LPEN_A {
        match self.bits {
            false => TIM1LPEN_A::DISABLED,
            true => TIM1LPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TIM1LPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TIM1LPEN_A::ENABLED
    }
}
impl core::ops::Deref for TIM1LPEN_R {
    type Target = crate::FieldReader<bool, TIM1LPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1LPEN` writer - TIM1 peripheral clock enable during CSleep mode Set and reset by software."]
pub struct TIM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::ENABLED)
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
#[doc = "TIM8 peripheral clock enable during CSleep mode Set and reset by software."]
pub type TIM8LPEN_A = TIM1LPEN_A;
#[doc = "Field `TIM8LPEN` reader - TIM8 peripheral clock enable during CSleep mode Set and reset by software."]
pub type TIM8LPEN_R = TIM1LPEN_R;
#[doc = "Field `TIM8LPEN` writer - TIM8 peripheral clock enable during CSleep mode Set and reset by software."]
pub struct TIM8LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM8LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM8LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM8LPEN_A::ENABLED)
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
#[doc = "USART1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub type USART1LPEN_A = TIM1LPEN_A;
#[doc = "Field `USART1LPEN` reader - USART1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub type USART1LPEN_R = TIM1LPEN_R;
#[doc = "Field `USART1LPEN` writer - USART1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub struct USART1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART1LPEN_A::ENABLED)
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
#[doc = "USART6 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART6 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub type USART6LPEN_A = TIM1LPEN_A;
#[doc = "Field `USART6LPEN` reader - USART6 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART6 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub type USART6LPEN_R = TIM1LPEN_R;
#[doc = "Field `USART6LPEN` writer - USART6 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART6 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub struct USART6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART6LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART6LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART6LPEN_A::ENABLED)
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
#[doc = "UART9 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the UART9 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub type UART9LPEN_A = TIM1LPEN_A;
#[doc = "Field `UART9LPEN` reader - UART9 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the UART9 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub type UART9LPEN_R = TIM1LPEN_R;
#[doc = "Field `UART9LPEN` writer - UART9 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the UART9 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub struct UART9LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART9LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART9LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UART9LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UART9LPEN_A::ENABLED)
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
#[doc = "USART10 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART10 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub type USART10LPEN_A = TIM1LPEN_A;
#[doc = "Field `USART10LPEN` reader - USART10 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART10 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub type USART10LPEN_R = TIM1LPEN_R;
#[doc = "Field `USART10LPEN` writer - USART10 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART10 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub struct USART10LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART10LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART10LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART10LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART10LPEN_A::ENABLED)
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
#[doc = "SPI1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI1 are: the kernel clock selected by I2S123SRC and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub type SPI1LPEN_A = TIM1LPEN_A;
#[doc = "Field `SPI1LPEN` reader - SPI1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI1 are: the kernel clock selected by I2S123SRC and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub type SPI1LPEN_R = TIM1LPEN_R;
#[doc = "Field `SPI1LPEN` writer - SPI1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI1 are: the kernel clock selected by I2S123SRC and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub struct SPI1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI1LPEN_A::ENABLED)
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
#[doc = "SPI4 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub type SPI4LPEN_A = TIM1LPEN_A;
#[doc = "Field `SPI4LPEN` reader - SPI4 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub type SPI4LPEN_R = TIM1LPEN_R;
#[doc = "Field `SPI4LPEN` writer - SPI4 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub struct SPI4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI4LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI4LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI4LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI4LPEN_A::ENABLED)
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
#[doc = "TIM15 peripheral clock enable during CSleep mode Set and reset by software."]
pub type TIM15LPEN_A = TIM1LPEN_A;
#[doc = "Field `TIM15LPEN` reader - TIM15 peripheral clock enable during CSleep mode Set and reset by software."]
pub type TIM15LPEN_R = TIM1LPEN_R;
#[doc = "Field `TIM15LPEN` writer - TIM15 peripheral clock enable during CSleep mode Set and reset by software."]
pub struct TIM15LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM15LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM15LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM15LPEN_A::ENABLED)
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
#[doc = "TIM16 peripheral clock enable during CSleep mode Set and reset by software."]
pub type TIM16LPEN_A = TIM1LPEN_A;
#[doc = "Field `TIM16LPEN` reader - TIM16 peripheral clock enable during CSleep mode Set and reset by software."]
pub type TIM16LPEN_R = TIM1LPEN_R;
#[doc = "Field `TIM16LPEN` writer - TIM16 peripheral clock enable during CSleep mode Set and reset by software."]
pub struct TIM16LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM16LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM16LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM16LPEN_A::ENABLED)
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
#[doc = "TIM17 peripheral clock enable during CSleep mode Set and reset by software."]
pub type TIM17LPEN_A = TIM1LPEN_A;
#[doc = "Field `TIM17LPEN` reader - TIM17 peripheral clock enable during CSleep mode Set and reset by software."]
pub type TIM17LPEN_R = TIM1LPEN_R;
#[doc = "Field `TIM17LPEN` writer - TIM17 peripheral clock enable during CSleep mode Set and reset by software."]
pub struct TIM17LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM17LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM17LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM17LPEN_A::ENABLED)
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
#[doc = "SPI5 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub type SPI5LPEN_A = TIM1LPEN_A;
#[doc = "Field `SPI5LPEN` reader - SPI5 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub type SPI5LPEN_R = TIM1LPEN_R;
#[doc = "Field `SPI5LPEN` writer - SPI5 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
pub struct SPI5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI5LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI5LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI5LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI5LPEN_A::ENABLED)
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
#[doc = "SAI1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI1 are: the kernel clock selected by SAI1SEL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub type SAI1LPEN_A = TIM1LPEN_A;
#[doc = "Field `SAI1LPEN` reader - SAI1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI1 are: the kernel clock selected by SAI1SEL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub type SAI1LPEN_R = TIM1LPEN_R;
#[doc = "Field `SAI1LPEN` writer - SAI1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI1 are: the kernel clock selected by SAI1SEL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub struct SAI1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAI1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAI1LPEN_A::ENABLED)
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
#[doc = "SAI2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI2 are the kernel clock selected by SAI23EL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub type SAI2LPEN_A = TIM1LPEN_A;
#[doc = "Field `SAI2LPEN` reader - SAI2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI2 are the kernel clock selected by SAI23EL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub type SAI2LPEN_R = TIM1LPEN_R;
#[doc = "Field `SAI2LPEN` writer - SAI2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI2 are the kernel clock selected by SAI23EL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
pub struct SAI2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAI2LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAI2LPEN_A::ENABLED)
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
#[doc = "DFSDM1 peripheral clocks enable during CSleep mode Set and reset by software. DFSDM1 peripheral clocks are the kernel clocks selected by SAI1SEL and DFSDM1SEL and provided to Aclk and clk inputs respectively, and the rcc_pclk2 bus interface clock."]
pub type DFSDM1LPEN_A = TIM1LPEN_A;
#[doc = "Field `DFSDM1LPEN` reader - DFSDM1 peripheral clocks enable during CSleep mode Set and reset by software. DFSDM1 peripheral clocks are the kernel clocks selected by SAI1SEL and DFSDM1SEL and provided to Aclk and clk inputs respectively, and the rcc_pclk2 bus interface clock."]
pub type DFSDM1LPEN_R = TIM1LPEN_R;
#[doc = "Field `DFSDM1LPEN` writer - DFSDM1 peripheral clocks enable during CSleep mode Set and reset by software. DFSDM1 peripheral clocks are the kernel clocks selected by SAI1SEL and DFSDM1SEL and provided to Aclk and clk inputs respectively, and the rcc_pclk2 bus interface clock."]
pub struct DFSDM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFSDM1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DFSDM1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DFSDM1LPEN_A::ENABLED)
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
    #[doc = "Bit 0 - TIM1 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USART6 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART6 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART9 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the UART9 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn uart9lpen(&self) -> UART9LPEN_R {
        UART9LPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USART10 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART10 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn usart10lpen(&self) -> USART10LPEN_R {
        USART10LPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI1 are: the kernel clock selected by I2S123SRC and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SPI4 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim15lpen(&self) -> TIM15LPEN_R {
        TIM15LPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim16lpen(&self) -> TIM16LPEN_R {
        TIM16LPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim17lpen(&self) -> TIM17LPEN_R {
        TIM17LPEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SPI5 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SAI1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI1 are: the kernel clock selected by SAI1SEL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SAI2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI2 are the kernel clock selected by SAI23EL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DFSDM1 peripheral clocks enable during CSleep mode Set and reset by software. DFSDM1 peripheral clocks are the kernel clocks selected by SAI1SEL and DFSDM1SEL and provided to Aclk and clk inputs respectively, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn dfsdm1lpen(&self) -> DFSDM1LPEN_R {
        DFSDM1LPEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W {
        TIM1LPEN_W { w: self }
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W {
        TIM8LPEN_W { w: self }
    }
    #[doc = "Bit 4 - USART1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART1 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W {
        USART1LPEN_W { w: self }
    }
    #[doc = "Bit 5 - USART6 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART6 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W {
        USART6LPEN_W { w: self }
    }
    #[doc = "Bit 6 - UART9 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the UART9 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn uart9lpen(&mut self) -> UART9LPEN_W {
        UART9LPEN_W { w: self }
    }
    #[doc = "Bit 7 - USART10 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the USART10 are the kernel clock selected by USART16910SEL and provided to usart_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn usart10lpen(&mut self) -> USART10LPEN_W {
        USART10LPEN_W { w: self }
    }
    #[doc = "Bit 12 - SPI1 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI1 are: the kernel clock selected by I2S123SRC and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W {
        SPI1LPEN_W { w: self }
    }
    #[doc = "Bit 13 - SPI4 peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI4 are: the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W {
        SPI4LPEN_W { w: self }
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim15lpen(&mut self) -> TIM15LPEN_W {
        TIM15LPEN_W { w: self }
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim16lpen(&mut self) -> TIM16LPEN_W {
        TIM16LPEN_W { w: self }
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim17lpen(&mut self) -> TIM17LPEN_W {
        TIM17LPEN_W { w: self }
    }
    #[doc = "Bit 20 - SPI5 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI5 are the kernel clock selected by SPI45SEL and provided to spi_ker_ck input, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W {
        SPI5LPEN_W { w: self }
    }
    #[doc = "Bit 22 - SAI1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI1 are: the kernel clock selected by SAI1SEL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W {
        SAI1LPEN_W { w: self }
    }
    #[doc = "Bit 23 - SAI2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SAI2 are the kernel clock selected by SAI23EL and provided to sai_a_ker_ck and sai_b_ker_ck inputs, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W {
        SAI2LPEN_W { w: self }
    }
    #[doc = "Bit 30 - DFSDM1 peripheral clocks enable during CSleep mode Set and reset by software. DFSDM1 peripheral clocks are the kernel clocks selected by SAI1SEL and DFSDM1SEL and provided to Aclk and clk inputs respectively, and the rcc_pclk2 bus interface clock."]
    #[inline(always)]
    pub fn dfsdm1lpen(&mut self) -> DFSDM1LPEN_W {
        DFSDM1LPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2lpenr](index.html) module"]
pub struct APB2LPENR_SPEC;
impl crate::RegisterSpec for APB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2lpenr::R](R) reader structure"]
impl crate::Readable for APB2LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2lpenr::W](W) writer structure"]
impl crate::Writable for APB2LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2LPENR to value 0x40d7_30f3"]
impl crate::Resettable for APB2LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40d7_30f3
    }
}