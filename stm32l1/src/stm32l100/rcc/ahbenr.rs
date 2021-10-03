#[doc = "Register `AHBENR` reader"]
pub struct R(crate::R<AHBENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBENR` writer"]
pub struct W(crate::W<AHBENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBENR_SPEC>;
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
impl From<crate::W<AHBENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FSMCEN"]
pub type FSMCEN_A = GPIOPAEN_A;
#[doc = "Field `FSMCEN` reader - FSMCEN"]
pub type FSMCEN_R = GPIOPAEN_R;
#[doc = "Field `FSMCEN` writer - FSMCEN"]
pub struct FSMCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSMCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FSMCEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FSMCEN_A::ENABLED)
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
#[doc = "DMA2 clock enable"]
pub type DMA2EN_A = GPIOPAEN_A;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub type DMA2EN_R = GPIOPAEN_R;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub struct DMA2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA2EN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA2EN_A::ENABLED)
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
#[doc = "DMA1 clock enable"]
pub type DMA1EN_A = GPIOPAEN_A;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type DMA1EN_R = GPIOPAEN_R;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub struct DMA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::ENABLED)
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
#[doc = "FLITF clock enable"]
pub type FLITFEN_A = GPIOPAEN_A;
#[doc = "Field `FLITFEN` reader - FLITF clock enable"]
pub type FLITFEN_R = GPIOPAEN_R;
#[doc = "Field `FLITFEN` writer - FLITF clock enable"]
pub struct FLITFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLITFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLITFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLITFEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLITFEN_A::ENABLED)
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
#[doc = "CRC clock enable"]
pub type CRCEN_A = GPIOPAEN_A;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CRCEN_R = GPIOPAEN_R;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCEN_A::ENABLED)
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
#[doc = "IO port G clock enable"]
pub type GPIOPGEN_A = GPIOPAEN_A;
#[doc = "Field `GPIOPGEN` reader - IO port G clock enable"]
pub type GPIOPGEN_R = GPIOPAEN_R;
#[doc = "Field `GPIOPGEN` writer - IO port G clock enable"]
pub struct GPIOPGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOPGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOPGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOPGEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOPGEN_A::ENABLED)
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
#[doc = "IO port F clock enable"]
pub type GPIOPFEN_A = GPIOPAEN_A;
#[doc = "Field `GPIOPFEN` reader - IO port F clock enable"]
pub type GPIOPFEN_R = GPIOPAEN_R;
#[doc = "Field `GPIOPFEN` writer - IO port F clock enable"]
pub struct GPIOPFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOPFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOPFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOPFEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOPFEN_A::ENABLED)
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
#[doc = "IO port H clock enable"]
pub type GPIOPHEN_A = GPIOPAEN_A;
#[doc = "Field `GPIOPHEN` reader - IO port H clock enable"]
pub type GPIOPHEN_R = GPIOPAEN_R;
#[doc = "Field `GPIOPHEN` writer - IO port H clock enable"]
pub struct GPIOPHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOPHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOPHEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOPHEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOPHEN_A::ENABLED)
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
#[doc = "IO port E clock enable"]
pub type GPIOPEEN_A = GPIOPAEN_A;
#[doc = "Field `GPIOPEEN` reader - IO port E clock enable"]
pub type GPIOPEEN_R = GPIOPAEN_R;
#[doc = "Field `GPIOPEEN` writer - IO port E clock enable"]
pub struct GPIOPEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOPEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOPEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOPEEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOPEEN_A::ENABLED)
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
#[doc = "IO port D clock enable"]
pub type GPIOPDEN_A = GPIOPAEN_A;
#[doc = "Field `GPIOPDEN` reader - IO port D clock enable"]
pub type GPIOPDEN_R = GPIOPAEN_R;
#[doc = "Field `GPIOPDEN` writer - IO port D clock enable"]
pub struct GPIOPDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOPDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOPDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOPDEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOPDEN_A::ENABLED)
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
#[doc = "IO port C clock enable"]
pub type GPIOPCEN_A = GPIOPAEN_A;
#[doc = "Field `GPIOPCEN` reader - IO port C clock enable"]
pub type GPIOPCEN_R = GPIOPAEN_R;
#[doc = "Field `GPIOPCEN` writer - IO port C clock enable"]
pub struct GPIOPCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOPCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOPCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOPCEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOPCEN_A::ENABLED)
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
#[doc = "IO port B clock enable"]
pub type GPIOPBEN_A = GPIOPAEN_A;
#[doc = "Field `GPIOPBEN` reader - IO port B clock enable"]
pub type GPIOPBEN_R = GPIOPAEN_R;
#[doc = "Field `GPIOPBEN` writer - IO port B clock enable"]
pub struct GPIOPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOPBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOPBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOPBEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOPBEN_A::ENABLED)
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
#[doc = "IO port A clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOPAEN_A {
    #[doc = "0: Clock disabled"]
    DISABLED = 0,
    #[doc = "1: Clock enabled"]
    ENABLED = 1,
}
impl From<GPIOPAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOPAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOPAEN` reader - IO port A clock enable"]
pub struct GPIOPAEN_R(crate::FieldReader<bool, GPIOPAEN_A>);
impl GPIOPAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOPAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOPAEN_A {
        match self.bits {
            false => GPIOPAEN_A::DISABLED,
            true => GPIOPAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == GPIOPAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == GPIOPAEN_A::ENABLED
    }
}
impl core::ops::Deref for GPIOPAEN_R {
    type Target = crate::FieldReader<bool, GPIOPAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOPAEN` writer - IO port A clock enable"]
pub struct GPIOPAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOPAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOPAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOPAEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOPAEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 30 - FSMCEN"]
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiopgen(&self) -> GPIOPGEN_R {
        GPIOPGEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiopfen(&self) -> GPIOPFEN_R {
        GPIOPFEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiophen(&self) -> GPIOPHEN_R {
        GPIOPHEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpiopeen(&self) -> GPIOPEEN_R {
        GPIOPEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpiopden(&self) -> GPIOPDEN_R {
        GPIOPDEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiopcen(&self) -> GPIOPCEN_R {
        GPIOPCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpiopben(&self) -> GPIOPBEN_R {
        GPIOPBEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpiopaen(&self) -> GPIOPAEN_R {
        GPIOPAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - FSMCEN"]
    #[inline(always)]
    pub fn fsmcen(&mut self) -> FSMCEN_W {
        FSMCEN_W { w: self }
    }
    #[doc = "Bit 25 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W {
        DMA2EN_W { w: self }
    }
    #[doc = "Bit 24 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W {
        DMA1EN_W { w: self }
    }
    #[doc = "Bit 15 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&mut self) -> FLITFEN_W {
        FLITFEN_W { w: self }
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    #[doc = "Bit 7 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiopgen(&mut self) -> GPIOPGEN_W {
        GPIOPGEN_W { w: self }
    }
    #[doc = "Bit 6 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiopfen(&mut self) -> GPIOPFEN_W {
        GPIOPFEN_W { w: self }
    }
    #[doc = "Bit 5 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiophen(&mut self) -> GPIOPHEN_W {
        GPIOPHEN_W { w: self }
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpiopeen(&mut self) -> GPIOPEEN_W {
        GPIOPEEN_W { w: self }
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpiopden(&mut self) -> GPIOPDEN_W {
        GPIOPDEN_W { w: self }
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiopcen(&mut self) -> GPIOPCEN_W {
        GPIOPCEN_W { w: self }
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpiopben(&mut self) -> GPIOPBEN_W {
        GPIOPBEN_W { w: self }
    }
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpiopaen(&mut self) -> GPIOPAEN_W {
        GPIOPAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbenr](index.html) module"]
pub struct AHBENR_SPEC;
impl crate::RegisterSpec for AHBENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbenr::R](R) reader structure"]
impl crate::Readable for AHBENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbenr::W](W) writer structure"]
impl crate::Writable for AHBENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBENR to value 0x8000"]
impl crate::Resettable for AHBENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
