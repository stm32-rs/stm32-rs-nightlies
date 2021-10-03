#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TAMP1IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1IE_A {
    #[doc = "0: Tamper x interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Tampoer x interrupt enabled"]
    ENABLED = 1,
}
impl From<TAMP1IE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1IE` reader - TAMP1IE"]
pub struct TAMP1IE_R(crate::FieldReader<bool, TAMP1IE_A>);
impl TAMP1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1IE_A {
        match self.bits {
            false => TAMP1IE_A::DISABLED,
            true => TAMP1IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TAMP1IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TAMP1IE_A::ENABLED
    }
}
impl core::ops::Deref for TAMP1IE_R {
    type Target = crate::FieldReader<bool, TAMP1IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1IE` writer - TAMP1IE"]
pub struct TAMP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP1IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP1IE_A::DISABLED)
    }
    #[doc = "Tampoer x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP1IE_A::ENABLED)
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
#[doc = "TAMP2IE"]
pub type TAMP2IE_A = TAMP1IE_A;
#[doc = "Field `TAMP2IE` reader - TAMP2IE"]
pub type TAMP2IE_R = TAMP1IE_R;
#[doc = "Field `TAMP2IE` writer - TAMP2IE"]
pub struct TAMP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP2IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP2IE_A::DISABLED)
    }
    #[doc = "Tampoer x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP2IE_A::ENABLED)
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
#[doc = "TAMP3IE"]
pub type TAMP3IE_A = TAMP1IE_A;
#[doc = "Field `TAMP3IE` reader - TAMP3IE"]
pub type TAMP3IE_R = TAMP1IE_R;
#[doc = "Field `TAMP3IE` writer - TAMP3IE"]
pub struct TAMP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP3IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP3IE_A::DISABLED)
    }
    #[doc = "Tampoer x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP3IE_A::ENABLED)
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
#[doc = "ITAMP3IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITAMP3IE_A {
    #[doc = "0: Internal tamper x interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Internal tamper x interrupt enabled"]
    ENABLED = 1,
}
impl From<ITAMP3IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3IE` reader - ITAMP3IE"]
pub struct ITAMP3IE_R(crate::FieldReader<bool, ITAMP3IE_A>);
impl ITAMP3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP3IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITAMP3IE_A {
        match self.bits {
            false => ITAMP3IE_A::DISABLED,
            true => ITAMP3IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ITAMP3IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ITAMP3IE_A::ENABLED
    }
}
impl core::ops::Deref for ITAMP3IE_R {
    type Target = crate::FieldReader<bool, ITAMP3IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP3IE` writer - ITAMP3IE"]
pub struct ITAMP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP3IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITAMP3IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal tamper x interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITAMP3IE_A::DISABLED)
    }
    #[doc = "Internal tamper x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITAMP3IE_A::ENABLED)
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
#[doc = "ITAMP5IE"]
pub type ITAMP5IE_A = ITAMP3IE_A;
#[doc = "Field `ITAMP5IE` reader - ITAMP5IE"]
pub type ITAMP5IE_R = ITAMP3IE_R;
#[doc = "Field `ITAMP5IE` writer - ITAMP5IE"]
pub struct ITAMP5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP5IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITAMP5IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal tamper x interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITAMP5IE_A::DISABLED)
    }
    #[doc = "Internal tamper x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITAMP5IE_A::ENABLED)
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
#[doc = "ITAMP6IE"]
pub type ITAMP6IE_A = ITAMP3IE_A;
#[doc = "Field `ITAMP6IE` reader - ITAMP6IE"]
pub type ITAMP6IE_R = ITAMP3IE_R;
#[doc = "Field `ITAMP6IE` writer - ITAMP6IE"]
pub struct ITAMP6IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP6IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITAMP6IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal tamper x interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITAMP6IE_A::DISABLED)
    }
    #[doc = "Internal tamper x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITAMP6IE_A::ENABLED)
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
#[doc = "ITAMP8IE"]
pub type ITAMP8IE_A = ITAMP3IE_A;
#[doc = "Field `ITAMP8IE` reader - ITAMP8IE"]
pub type ITAMP8IE_R = ITAMP3IE_R;
#[doc = "Field `ITAMP8IE` writer - ITAMP8IE"]
pub struct ITAMP8IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP8IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITAMP8IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal tamper x interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITAMP8IE_A::DISABLED)
    }
    #[doc = "Internal tamper x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITAMP8IE_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - TAMP1IE"]
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TAMP2IE"]
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TAMP3IE"]
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ITAMP3IE"]
    #[inline(always)]
    pub fn itamp3ie(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ITAMP5IE"]
    #[inline(always)]
    pub fn itamp5ie(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ITAMP6IE"]
    #[inline(always)]
    pub fn itamp6ie(&self) -> ITAMP6IE_R {
        ITAMP6IE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ITAMP8IE"]
    #[inline(always)]
    pub fn itamp8ie(&self) -> ITAMP8IE_R {
        ITAMP8IE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1IE"]
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W {
        TAMP1IE_W { w: self }
    }
    #[doc = "Bit 1 - TAMP2IE"]
    #[inline(always)]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W {
        TAMP2IE_W { w: self }
    }
    #[doc = "Bit 2 - TAMP3IE"]
    #[inline(always)]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W {
        TAMP3IE_W { w: self }
    }
    #[doc = "Bit 18 - ITAMP3IE"]
    #[inline(always)]
    pub fn itamp3ie(&mut self) -> ITAMP3IE_W {
        ITAMP3IE_W { w: self }
    }
    #[doc = "Bit 20 - ITAMP5IE"]
    #[inline(always)]
    pub fn itamp5ie(&mut self) -> ITAMP5IE_W {
        ITAMP5IE_W { w: self }
    }
    #[doc = "Bit 21 - ITAMP6IE"]
    #[inline(always)]
    pub fn itamp6ie(&mut self) -> ITAMP6IE_W {
        ITAMP6IE_W { w: self }
    }
    #[doc = "Bit 23 - ITAMP8IE"]
    #[inline(always)]
    pub fn itamp8ie(&mut self) -> ITAMP8IE_W {
        ITAMP8IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
