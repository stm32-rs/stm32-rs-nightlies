#[doc = "Register `LPMCSR` reader"]
pub struct R(crate::R<LPMCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMCSR` writer"]
pub struct W(crate::W<LPMCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMCSR_SPEC>;
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
impl From<crate::W<LPMCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LPM support enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMEN_A {
    #[doc = "0: enable the LPM support within the USB device"]
    DISABLED = 0,
    #[doc = "1: no LPM transactions are handled"]
    ENABLED = 1,
}
impl From<LPMEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMEN` reader - LPM support enable"]
pub struct LPMEN_R(crate::FieldReader<bool, LPMEN_A>);
impl LPMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMEN_A {
        match self.bits {
            false => LPMEN_A::DISABLED,
            true => LPMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LPMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LPMEN_A::ENABLED
    }
}
impl core::ops::Deref for LPMEN_R {
    type Target = crate::FieldReader<bool, LPMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMEN` writer - LPM support enable"]
pub struct LPMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable the LPM support within the USB device"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPMEN_A::DISABLED)
    }
    #[doc = "no LPM transactions are handled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPMEN_A::ENABLED)
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
#[doc = "LPM Token acknowledge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMACK_A {
    #[doc = "0: the valid LPM Token will be NYET"]
    NYET = 0,
    #[doc = "1: the valid LPM Token will be ACK"]
    ACK = 1,
}
impl From<LPMACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPMACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMACK` reader - LPM Token acknowledge enable"]
pub struct LPMACK_R(crate::FieldReader<bool, LPMACK_A>);
impl LPMACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPMACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMACK_A {
        match self.bits {
            false => LPMACK_A::NYET,
            true => LPMACK_A::ACK,
        }
    }
    #[doc = "Checks if the value of the field is `NYET`"]
    #[inline(always)]
    pub fn is_nyet(&self) -> bool {
        **self == LPMACK_A::NYET
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        **self == LPMACK_A::ACK
    }
}
impl core::ops::Deref for LPMACK_R {
    type Target = crate::FieldReader<bool, LPMACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMACK` writer - LPM Token acknowledge enable"]
pub struct LPMACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the valid LPM Token will be NYET"]
    #[inline(always)]
    pub fn nyet(self) -> &'a mut W {
        self.variant(LPMACK_A::NYET)
    }
    #[doc = "the valid LPM Token will be ACK"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(LPMACK_A::ACK)
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
#[doc = "Field `REMWAKE` reader - bRemoteWake value"]
pub struct REMWAKE_R(crate::FieldReader<bool, bool>);
impl REMWAKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REMWAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REMWAKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BESL` reader - BESL value"]
pub struct BESL_R(crate::FieldReader<u8, u8>);
impl BESL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BESL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BESL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPM Token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - bRemoteWake value"]
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - BESL value"]
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&mut self) -> LPMEN_W {
        LPMEN_W { w: self }
    }
    #[doc = "Bit 1 - LPM Token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&mut self) -> LPMACK_W {
        LPMACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPM control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmcsr](index.html) module"]
pub struct LPMCSR_SPEC;
impl crate::RegisterSpec for LPMCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmcsr::R](R) reader structure"]
impl crate::Readable for LPMCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmcsr::W](W) writer structure"]
impl crate::Writable for LPMCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPMCSR to value 0"]
impl crate::Resettable for LPMCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
