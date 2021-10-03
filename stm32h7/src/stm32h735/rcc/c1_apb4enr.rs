#[doc = "Register `C1_APB4ENR` reader"]
pub struct R(crate::R<C1_APB4ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_APB4ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_APB4ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_APB4ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_APB4ENR` writer"]
pub struct W(crate::W<C1_APB4ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_APB4ENR_SPEC>;
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
impl From<crate::W<C1_APB4ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_APB4ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SYSCFG peripheral clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCFGEN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<SYSCFGEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGEN` reader - SYSCFG peripheral clock enable"]
pub struct SYSCFGEN_R(crate::FieldReader<bool, SYSCFGEN_A>);
impl SYSCFGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGEN_A {
        match self.bits {
            false => SYSCFGEN_A::DISABLED,
            true => SYSCFGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SYSCFGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SYSCFGEN_A::ENABLED
    }
}
impl core::ops::Deref for SYSCFGEN_R {
    type Target = crate::FieldReader<bool, SYSCFGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGEN` writer - SYSCFG peripheral clock enable"]
pub struct SYSCFGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::ENABLED)
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
#[doc = "LPUART1 Peripheral Clocks Enable"]
pub type LPUART1EN_A = SYSCFGEN_A;
#[doc = "Field `LPUART1EN` reader - LPUART1 Peripheral Clocks Enable"]
pub type LPUART1EN_R = SYSCFGEN_R;
#[doc = "Field `LPUART1EN` writer - LPUART1 Peripheral Clocks Enable"]
pub struct LPUART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPUART1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPUART1EN_A::ENABLED)
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
#[doc = "SPI6 Peripheral Clocks Enable"]
pub type SPI6EN_A = SYSCFGEN_A;
#[doc = "Field `SPI6EN` reader - SPI6 Peripheral Clocks Enable"]
pub type SPI6EN_R = SYSCFGEN_R;
#[doc = "Field `SPI6EN` writer - SPI6 Peripheral Clocks Enable"]
pub struct SPI6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI6EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI6EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI6EN_A::ENABLED)
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
#[doc = "I2C4 Peripheral Clocks Enable"]
pub type I2C4EN_A = SYSCFGEN_A;
#[doc = "Field `I2C4EN` reader - I2C4 Peripheral Clocks Enable"]
pub type I2C4EN_R = SYSCFGEN_R;
#[doc = "Field `I2C4EN` writer - I2C4 Peripheral Clocks Enable"]
pub struct I2C4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C4EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C4EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C4EN_A::ENABLED)
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
#[doc = "LPTIM2 Peripheral Clocks Enable"]
pub type LPTIM2EN_A = SYSCFGEN_A;
#[doc = "Field `LPTIM2EN` reader - LPTIM2 Peripheral Clocks Enable"]
pub type LPTIM2EN_R = SYSCFGEN_R;
#[doc = "Field `LPTIM2EN` writer - LPTIM2 Peripheral Clocks Enable"]
pub struct LPTIM2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM2EN_A::ENABLED)
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
#[doc = "LPTIM3 Peripheral Clocks Enable"]
pub type LPTIM3EN_A = SYSCFGEN_A;
#[doc = "Field `LPTIM3EN` reader - LPTIM3 Peripheral Clocks Enable"]
pub type LPTIM3EN_R = SYSCFGEN_R;
#[doc = "Field `LPTIM3EN` writer - LPTIM3 Peripheral Clocks Enable"]
pub struct LPTIM3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM3EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM3EN_A::ENABLED)
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
#[doc = "LPTIM4 Peripheral Clocks Enable"]
pub type LPTIM4EN_A = SYSCFGEN_A;
#[doc = "Field `LPTIM4EN` reader - LPTIM4 Peripheral Clocks Enable"]
pub type LPTIM4EN_R = SYSCFGEN_R;
#[doc = "Field `LPTIM4EN` writer - LPTIM4 Peripheral Clocks Enable"]
pub struct LPTIM4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM4EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM4EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM4EN_A::ENABLED)
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
#[doc = "LPTIM5 Peripheral Clocks Enable"]
pub type LPTIM5EN_A = SYSCFGEN_A;
#[doc = "Field `LPTIM5EN` reader - LPTIM5 Peripheral Clocks Enable"]
pub type LPTIM5EN_R = SYSCFGEN_R;
#[doc = "Field `LPTIM5EN` writer - LPTIM5 Peripheral Clocks Enable"]
pub struct LPTIM5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM5EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM5EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM5EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM5EN_A::ENABLED)
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
#[doc = "COMP1/2 peripheral clock enable"]
pub type COMP12EN_A = SYSCFGEN_A;
#[doc = "Field `COMP12EN` reader - COMP1/2 peripheral clock enable"]
pub type COMP12EN_R = SYSCFGEN_R;
#[doc = "Field `COMP12EN` writer - COMP1/2 peripheral clock enable"]
pub struct COMP12EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP12EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP12EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP12EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP12EN_A::ENABLED)
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
#[doc = "VREF peripheral clock enable"]
pub type VREFEN_A = SYSCFGEN_A;
#[doc = "Field `VREFEN` reader - VREF peripheral clock enable"]
pub type VREFEN_R = SYSCFGEN_R;
#[doc = "Field `VREFEN` writer - VREF peripheral clock enable"]
pub struct VREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREFEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREFEN_A::ENABLED)
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
#[doc = "RTC APB Clock Enable"]
pub type RTCAPBEN_A = SYSCFGEN_A;
#[doc = "Field `RTCAPBEN` reader - RTC APB Clock Enable"]
pub type RTCAPBEN_R = SYSCFGEN_R;
#[doc = "Field `RTCAPBEN` writer - RTC APB Clock Enable"]
pub struct RTCAPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAPBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCAPBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCAPBEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCAPBEN_A::ENABLED)
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
#[doc = "SAI4 Peripheral Clocks Enable"]
pub type SAI4EN_A = SYSCFGEN_A;
#[doc = "Field `SAI4EN` reader - SAI4 Peripheral Clocks Enable"]
pub type SAI4EN_R = SYSCFGEN_R;
#[doc = "Field `SAI4EN` writer - SAI4 Peripheral Clocks Enable"]
pub struct SAI4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI4EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAI4EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAI4EN_A::ENABLED)
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
    #[doc = "Bit 1 - SYSCFG peripheral clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim5en(&self) -> LPTIM5EN_R {
        LPTIM5EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable"]
    #[inline(always)]
    pub fn comp12en(&self) -> COMP12EN_R {
        COMP12EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai4en(&self) -> SAI4EN_R {
        SAI4EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W {
        SYSCFGEN_W { w: self }
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W {
        LPUART1EN_W { w: self }
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi6en(&mut self) -> SPI6EN_W {
        SPI6EN_W { w: self }
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W {
        I2C4EN_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W {
        LPTIM2EN_W { w: self }
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W {
        LPTIM3EN_W { w: self }
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W {
        LPTIM4EN_W { w: self }
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim5en(&mut self) -> LPTIM5EN_W {
        LPTIM5EN_W { w: self }
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable"]
    #[inline(always)]
    pub fn comp12en(&mut self) -> COMP12EN_W {
        COMP12EN_W { w: self }
    }
    #[doc = "Bit 15 - VREF peripheral clock enable"]
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W {
        VREFEN_W { w: self }
    }
    #[doc = "Bit 16 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W {
        RTCAPBEN_W { w: self }
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai4en(&mut self) -> SAI4EN_W {
        SAI4EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB4 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb4enr](index.html) module"]
pub struct C1_APB4ENR_SPEC;
impl crate::RegisterSpec for C1_APB4ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_apb4enr::R](R) reader structure"]
impl crate::Readable for C1_APB4ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_apb4enr::W](W) writer structure"]
impl crate::Writable for C1_APB4ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1_APB4ENR to value 0x0001_0000"]
impl crate::Resettable for C1_APB4ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
