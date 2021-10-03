#[doc = "Register `SECSR` reader"]
pub struct R(crate::R<SECSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECSR` writer"]
pub struct W(crate::W<SECSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECSR_SPEC>;
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
impl From<crate::W<SECSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECEOP` reader - SECEOP"]
pub struct SECEOP_R(crate::FieldReader<bool, bool>);
impl SECEOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECEOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECEOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECEOP` writer - SECEOP"]
pub struct SECEOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SECEOP_W<'a> {
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
#[doc = "Field `SECOPERR` reader - SECOPERR"]
pub struct SECOPERR_R(crate::FieldReader<bool, bool>);
impl SECOPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECOPERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECOPERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECOPERR` writer - SECOPERR"]
pub struct SECOPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECOPERR_W<'a> {
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
#[doc = "Field `SECPROGERR` reader - SECPROGERR"]
pub struct SECPROGERR_R(crate::FieldReader<bool, bool>);
impl SECPROGERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECPROGERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECPROGERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECPROGERR` writer - SECPROGERR"]
pub struct SECPROGERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECPROGERR_W<'a> {
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
#[doc = "Field `SECWRPERR` reader - SECWRPERR"]
pub struct SECWRPERR_R(crate::FieldReader<bool, bool>);
impl SECWRPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECWRPERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECWRPERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECWRPERR` writer - SECWRPERR"]
pub struct SECWRPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECWRPERR_W<'a> {
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
#[doc = "Field `SECPGAERR` reader - SECPGAERR"]
pub struct SECPGAERR_R(crate::FieldReader<bool, bool>);
impl SECPGAERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECPGAERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECPGAERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECPGAERR` writer - SECPGAERR"]
pub struct SECPGAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECPGAERR_W<'a> {
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
#[doc = "Field `SECSIZERR` reader - SECSIZERR"]
pub struct SECSIZERR_R(crate::FieldReader<bool, bool>);
impl SECSIZERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECSIZERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECSIZERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECSIZERR` writer - SECSIZERR"]
pub struct SECSIZERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECSIZERR_W<'a> {
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
#[doc = "Field `SECPGSERR` reader - SECPGSERR"]
pub struct SECPGSERR_R(crate::FieldReader<bool, bool>);
impl SECPGSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECPGSERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECPGSERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECPGSERR` writer - SECPGSERR"]
pub struct SECPGSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECPGSERR_W<'a> {
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
#[doc = "Field `SECRDERR` reader - Secure read protection error"]
pub struct SECRDERR_R(crate::FieldReader<bool, bool>);
impl SECRDERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECRDERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECRDERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECRDERR` writer - Secure read protection error"]
pub struct SECRDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECRDERR_W<'a> {
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
#[doc = "Field `SECBSY` reader - SECBusy"]
pub struct SECBSY_R(crate::FieldReader<bool, bool>);
impl SECBSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECBSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECBSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SECEOP"]
    #[inline(always)]
    pub fn seceop(&self) -> SECEOP_R {
        SECEOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SECOPERR"]
    #[inline(always)]
    pub fn secoperr(&self) -> SECOPERR_R {
        SECOPERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SECPROGERR"]
    #[inline(always)]
    pub fn secprogerr(&self) -> SECPROGERR_R {
        SECPROGERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SECWRPERR"]
    #[inline(always)]
    pub fn secwrperr(&self) -> SECWRPERR_R {
        SECWRPERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SECPGAERR"]
    #[inline(always)]
    pub fn secpgaerr(&self) -> SECPGAERR_R {
        SECPGAERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SECSIZERR"]
    #[inline(always)]
    pub fn secsizerr(&self) -> SECSIZERR_R {
        SECSIZERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SECPGSERR"]
    #[inline(always)]
    pub fn secpgserr(&self) -> SECPGSERR_R {
        SECPGSERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Secure read protection error"]
    #[inline(always)]
    pub fn secrderr(&self) -> SECRDERR_R {
        SECRDERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SECBusy"]
    #[inline(always)]
    pub fn secbsy(&self) -> SECBSY_R {
        SECBSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SECEOP"]
    #[inline(always)]
    pub fn seceop(&mut self) -> SECEOP_W {
        SECEOP_W { w: self }
    }
    #[doc = "Bit 1 - SECOPERR"]
    #[inline(always)]
    pub fn secoperr(&mut self) -> SECOPERR_W {
        SECOPERR_W { w: self }
    }
    #[doc = "Bit 3 - SECPROGERR"]
    #[inline(always)]
    pub fn secprogerr(&mut self) -> SECPROGERR_W {
        SECPROGERR_W { w: self }
    }
    #[doc = "Bit 4 - SECWRPERR"]
    #[inline(always)]
    pub fn secwrperr(&mut self) -> SECWRPERR_W {
        SECWRPERR_W { w: self }
    }
    #[doc = "Bit 5 - SECPGAERR"]
    #[inline(always)]
    pub fn secpgaerr(&mut self) -> SECPGAERR_W {
        SECPGAERR_W { w: self }
    }
    #[doc = "Bit 6 - SECSIZERR"]
    #[inline(always)]
    pub fn secsizerr(&mut self) -> SECSIZERR_W {
        SECSIZERR_W { w: self }
    }
    #[doc = "Bit 7 - SECPGSERR"]
    #[inline(always)]
    pub fn secpgserr(&mut self) -> SECPGSERR_W {
        SECPGSERR_W { w: self }
    }
    #[doc = "Bit 14 - Secure read protection error"]
    #[inline(always)]
    pub fn secrderr(&mut self) -> SECRDERR_W {
        SECRDERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secsr](index.html) module"]
pub struct SECSR_SPEC;
impl crate::RegisterSpec for SECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secsr::R](R) reader structure"]
impl crate::Readable for SECSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secsr::W](W) writer structure"]
impl crate::Writable for SECSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECSR to value 0"]
impl crate::Resettable for SECSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
