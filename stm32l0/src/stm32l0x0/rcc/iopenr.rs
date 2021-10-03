#[doc = "Register `IOPENR` reader"]
pub struct R(crate::R<IOPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOPENR` writer"]
pub struct W(crate::W<IOPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPENR_SPEC>;
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
impl From<crate::W<IOPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I/O port H clock enable bit"]
pub type IOPHEN_A = IOPAEN_A;
#[doc = "Field `IOPHEN` reader - I/O port H clock enable bit"]
pub type IOPHEN_R = IOPAEN_R;
#[doc = "Field `IOPHEN` writer - I/O port H clock enable bit"]
pub struct IOPHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPHEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::DISABLED)
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::ENABLED)
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
#[doc = "I/O port D clock enable bit"]
pub type IOPDEN_A = IOPAEN_A;
#[doc = "Field `IOPDEN` reader - I/O port D clock enable bit"]
pub type IOPDEN_R = IOPAEN_R;
#[doc = "Field `IOPDEN` writer - I/O port D clock enable bit"]
pub struct IOPDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPDEN_A::DISABLED)
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPDEN_A::ENABLED)
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
#[doc = "IO port A clock enable bit"]
pub type IOPCEN_A = IOPAEN_A;
#[doc = "Field `IOPCEN` reader - IO port A clock enable bit"]
pub type IOPCEN_R = IOPAEN_R;
#[doc = "Field `IOPCEN` writer - IO port A clock enable bit"]
pub struct IOPCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPCEN_A::DISABLED)
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPCEN_A::ENABLED)
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
#[doc = "IO port B clock enable bit"]
pub type IOPBEN_A = IOPAEN_A;
#[doc = "Field `IOPBEN` reader - IO port B clock enable bit"]
pub type IOPBEN_R = IOPAEN_R;
#[doc = "Field `IOPBEN` writer - IO port B clock enable bit"]
pub struct IOPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPBEN_A::DISABLED)
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPBEN_A::ENABLED)
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
#[doc = "IO port A clock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOPAEN_A {
    #[doc = "0: Port clock disabled"]
    DISABLED = 0,
    #[doc = "1: Port clock enabled"]
    ENABLED = 1,
}
impl From<IOPAEN_A> for bool {
    #[inline(always)]
    fn from(variant: IOPAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOPAEN` reader - IO port A clock enable bit"]
pub struct IOPAEN_R(crate::FieldReader<bool, IOPAEN_A>);
impl IOPAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOPAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOPAEN_A {
        match self.bits {
            false => IOPAEN_A::DISABLED,
            true => IOPAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == IOPAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IOPAEN_A::ENABLED
    }
}
impl core::ops::Deref for IOPAEN_R {
    type Target = crate::FieldReader<bool, IOPAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPAEN` writer - IO port A clock enable bit"]
pub struct IOPAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPAEN_A::DISABLED)
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPAEN_A::ENABLED)
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
#[doc = "IO port E clock enable bit"]
pub type IOPEEN_A = IOPAEN_A;
#[doc = "Field `IOPEEN` reader - IO port E clock enable bit"]
pub type IOPEEN_R = IOPAEN_R;
#[doc = "Field `IOPEEN` writer - IO port E clock enable bit"]
pub struct IOPEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPEEN_A::DISABLED)
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPEEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 7 - I/O port H clock enable bit"]
    #[inline(always)]
    pub fn iophen(&self) -> IOPHEN_R {
        IOPHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable bit"]
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IO port A clock enable bit"]
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable bit"]
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - IO port A clock enable bit"]
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable bit"]
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - I/O port H clock enable bit"]
    #[inline(always)]
    pub fn iophen(&mut self) -> IOPHEN_W {
        IOPHEN_W { w: self }
    }
    #[doc = "Bit 3 - I/O port D clock enable bit"]
    #[inline(always)]
    pub fn iopden(&mut self) -> IOPDEN_W {
        IOPDEN_W { w: self }
    }
    #[doc = "Bit 2 - IO port A clock enable bit"]
    #[inline(always)]
    pub fn iopcen(&mut self) -> IOPCEN_W {
        IOPCEN_W { w: self }
    }
    #[doc = "Bit 1 - IO port B clock enable bit"]
    #[inline(always)]
    pub fn iopben(&mut self) -> IOPBEN_W {
        IOPBEN_W { w: self }
    }
    #[doc = "Bit 0 - IO port A clock enable bit"]
    #[inline(always)]
    pub fn iopaen(&mut self) -> IOPAEN_W {
        IOPAEN_W { w: self }
    }
    #[doc = "Bit 4 - IO port E clock enable bit"]
    #[inline(always)]
    pub fn iopeen(&mut self) -> IOPEEN_W {
        IOPEEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iopenr](index.html) module"]
pub struct IOPENR_SPEC;
impl crate::RegisterSpec for IOPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iopenr::R](R) reader structure"]
impl crate::Readable for IOPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iopenr::W](W) writer structure"]
impl crate::Writable for IOPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOPENR to value 0"]
impl crate::Resettable for IOPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
