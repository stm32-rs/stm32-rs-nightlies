#[doc = "Register `AHB2ENR` reader"]
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2ENR` writer"]
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
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
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IO port A clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOAEN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<GPIOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOAEN` reader - IO port A clock enable"]
pub struct GPIOAEN_R(crate::FieldReader<bool, GPIOAEN_A>);
impl GPIOAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOAEN_A {
        match self.bits {
            false => GPIOAEN_A::DISABLED,
            true => GPIOAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == GPIOAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == GPIOAEN_A::ENABLED
    }
}
impl core::ops::Deref for GPIOAEN_R {
    type Target = crate::FieldReader<bool, GPIOAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOAEN` writer - IO port A clock enable"]
pub struct GPIOAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::ENABLED)
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
#[doc = "IO port B clock enable"]
pub type GPIOBEN_A = GPIOAEN_A;
#[doc = "Field `GPIOBEN` reader - IO port B clock enable"]
pub type GPIOBEN_R = GPIOAEN_R;
#[doc = "Field `GPIOBEN` writer - IO port B clock enable"]
pub struct GPIOBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOBEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOBEN_A::ENABLED)
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
#[doc = "IO port C clock enable"]
pub type GPIOCEN_A = GPIOAEN_A;
#[doc = "Field `GPIOCEN` reader - IO port C clock enable"]
pub type GPIOCEN_R = GPIOAEN_R;
#[doc = "Field `GPIOCEN` writer - IO port C clock enable"]
pub struct GPIOCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOCEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOCEN_A::ENABLED)
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
#[doc = "IO port D clock enable"]
pub type GPIODEN_A = GPIOAEN_A;
#[doc = "Field `GPIODEN` reader - IO port D clock enable"]
pub type GPIODEN_R = GPIOAEN_R;
#[doc = "Field `GPIODEN` writer - IO port D clock enable"]
pub struct GPIODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIODEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIODEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIODEN_A::ENABLED)
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
#[doc = "IO port E clock enable"]
pub type GPIOEEN_A = GPIOAEN_A;
#[doc = "Field `GPIOEEN` reader - IO port E clock enable"]
pub type GPIOEEN_R = GPIOAEN_R;
#[doc = "Field `GPIOEEN` writer - IO port E clock enable"]
pub struct GPIOEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOEEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOEEN_A::ENABLED)
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
#[doc = "IO port F clock enable"]
pub type GPIOFEN_A = GPIOAEN_A;
#[doc = "Field `GPIOFEN` reader - IO port F clock enable"]
pub type GPIOFEN_R = GPIOAEN_R;
#[doc = "Field `GPIOFEN` writer - IO port F clock enable"]
pub struct GPIOFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOFEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOFEN_A::ENABLED)
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
#[doc = "IO port G clock enable"]
pub type GPIOGEN_A = GPIOAEN_A;
#[doc = "Field `GPIOGEN` reader - IO port G clock enable"]
pub type GPIOGEN_R = GPIOAEN_R;
#[doc = "Field `GPIOGEN` writer - IO port G clock enable"]
pub struct GPIOGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOGEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOGEN_A::ENABLED)
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
#[doc = "ADC clock enable"]
pub type ADC12EN_A = GPIOAEN_A;
#[doc = "Field `ADC12EN` reader - ADC clock enable"]
pub type ADC12EN_R = GPIOAEN_R;
#[doc = "Field `ADC12EN` writer - ADC clock enable"]
pub struct ADC12EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC12EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC12EN_A::ENABLED)
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
#[doc = "DCMI clock enable"]
pub type ADC345EN_A = GPIOAEN_A;
#[doc = "Field `ADC345EN` reader - DCMI clock enable"]
pub type ADC345EN_R = GPIOAEN_R;
#[doc = "Field `ADC345EN` writer - DCMI clock enable"]
pub struct ADC345EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC345EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC345EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC345EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC345EN_A::ENABLED)
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
#[doc = "AES accelerator clock enable"]
pub type DAC1EN_A = GPIOAEN_A;
#[doc = "Field `DAC1EN` reader - AES accelerator clock enable"]
pub type DAC1EN_R = GPIOAEN_R;
#[doc = "Field `DAC1EN` writer - AES accelerator clock enable"]
pub struct DAC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAC1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAC1EN_A::ENABLED)
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
#[doc = "HASH clock enable"]
pub type DAC2EN_A = GPIOAEN_A;
#[doc = "Field `DAC2EN` reader - HASH clock enable"]
pub type DAC2EN_R = GPIOAEN_R;
#[doc = "Field `DAC2EN` writer - HASH clock enable"]
pub struct DAC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAC2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAC2EN_A::ENABLED)
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
#[doc = "Random Number Generator clock enable"]
pub type DAC3EN_A = GPIOAEN_A;
#[doc = "Field `DAC3EN` reader - Random Number Generator clock enable"]
pub type DAC3EN_R = GPIOAEN_R;
#[doc = "Field `DAC3EN` writer - Random Number Generator clock enable"]
pub struct DAC3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAC3EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAC3EN_A::ENABLED)
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
#[doc = "DAC4 clock enable"]
pub type DAC4EN_A = GPIOAEN_A;
#[doc = "Field `DAC4EN` reader - DAC4 clock enable"]
pub type DAC4EN_R = GPIOAEN_R;
#[doc = "Field `DAC4EN` writer - DAC4 clock enable"]
pub struct DAC4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC4EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAC4EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAC4EN_A::ENABLED)
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
#[doc = "AES clock enable"]
pub type AESEN_A = GPIOAEN_A;
#[doc = "Field `AESEN` reader - AES clock enable"]
pub type AESEN_R = GPIOAEN_R;
#[doc = "Field `AESEN` writer - AES clock enable"]
pub struct AESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AESEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AESEN_A::ENABLED)
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
#[doc = "Random Number Generator clock enable"]
pub type RNGEN_A = GPIOAEN_A;
#[doc = "Field `RNGEN` reader - Random Number Generator clock enable"]
pub type RNGEN_R = GPIOAEN_R;
#[doc = "Field `RNGEN` writer - Random Number Generator clock enable"]
pub struct RNGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RNGEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RNGEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DCMI clock enable"]
    #[inline(always)]
    pub fn adc345en(&self) -> ADC345EN_R {
        ADC345EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    pub fn dac2en(&self) -> DAC2EN_R {
        DAC2EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn dac3en(&self) -> DAC3EN_R {
        DAC3EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DAC4 clock enable"]
    #[inline(always)]
    pub fn dac4en(&self) -> DAC4EN_R {
        DAC4EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - AES clock enable"]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W {
        GPIOAEN_W { w: self }
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W {
        GPIOBEN_W { w: self }
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W {
        GPIOCEN_W { w: self }
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W {
        GPIODEN_W { w: self }
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W {
        GPIOEEN_W { w: self }
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W {
        GPIOFEN_W { w: self }
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W {
        GPIOGEN_W { w: self }
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    pub fn adc12en(&mut self) -> ADC12EN_W {
        ADC12EN_W { w: self }
    }
    #[doc = "Bit 14 - DCMI clock enable"]
    #[inline(always)]
    pub fn adc345en(&mut self) -> ADC345EN_W {
        ADC345EN_W { w: self }
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    pub fn dac1en(&mut self) -> DAC1EN_W {
        DAC1EN_W { w: self }
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    pub fn dac2en(&mut self) -> DAC2EN_W {
        DAC2EN_W { w: self }
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn dac3en(&mut self) -> DAC3EN_W {
        DAC3EN_W { w: self }
    }
    #[doc = "Bit 19 - DAC4 clock enable"]
    #[inline(always)]
    pub fn dac4en(&mut self) -> DAC4EN_W {
        DAC4EN_W { w: self }
    }
    #[doc = "Bit 24 - AES clock enable"]
    #[inline(always)]
    pub fn aesen(&mut self) -> AESEN_W {
        AESEN_W { w: self }
    }
    #[doc = "Bit 26 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W {
        RNGEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2enr](index.html) module"]
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2enr::R](R) reader structure"]
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2enr::W](W) writer structure"]
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
