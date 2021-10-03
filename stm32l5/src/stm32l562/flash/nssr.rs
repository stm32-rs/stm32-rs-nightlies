#[doc = "Register `NSSR` reader"]
pub struct R(crate::R<NSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSSR` writer"]
pub struct W(crate::W<NSSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSSR_SPEC>;
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
impl From<crate::W<NSSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSEOP` reader - NSEOP"]
pub struct NSEOP_R(crate::FieldReader<bool, bool>);
impl NSEOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSEOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSEOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSEOP` writer - NSEOP"]
pub struct NSEOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NSEOP_W<'a> {
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
#[doc = "Field `NSOPERR` reader - NSOPERR"]
pub struct NSOPERR_R(crate::FieldReader<bool, bool>);
impl NSOPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSOPERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSOPERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSOPERR` writer - NSOPERR"]
pub struct NSOPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSOPERR_W<'a> {
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
#[doc = "Field `NSPROGERR` reader - NSPROGERR"]
pub struct NSPROGERR_R(crate::FieldReader<bool, bool>);
impl NSPROGERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSPROGERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSPROGERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSPROGERR` writer - NSPROGERR"]
pub struct NSPROGERR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSPROGERR_W<'a> {
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
#[doc = "Field `NSWRPERR` reader - NSWRPERR"]
pub struct NSWRPERR_R(crate::FieldReader<bool, bool>);
impl NSWRPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSWRPERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSWRPERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSWRPERR` writer - NSWRPERR"]
pub struct NSWRPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSWRPERR_W<'a> {
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
#[doc = "Field `NSPGAERR` reader - NSPGAERR"]
pub struct NSPGAERR_R(crate::FieldReader<bool, bool>);
impl NSPGAERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSPGAERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSPGAERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSPGAERR` writer - NSPGAERR"]
pub struct NSPGAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSPGAERR_W<'a> {
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
#[doc = "Field `NSSIZERR` reader - NSSIZERR"]
pub struct NSSIZERR_R(crate::FieldReader<bool, bool>);
impl NSSIZERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSSIZERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSSIZERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSSIZERR` writer - NSSIZERR"]
pub struct NSSIZERR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSSIZERR_W<'a> {
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
#[doc = "Field `NSPGSERR` reader - NSPGSERR"]
pub struct NSPGSERR_R(crate::FieldReader<bool, bool>);
impl NSPGSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSPGSERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSPGSERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSPGSERR` writer - NSPGSERR"]
pub struct NSPGSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSPGSERR_W<'a> {
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
#[doc = "Field `OPTWERR` reader - OPTWERR"]
pub struct OPTWERR_R(crate::FieldReader<bool, bool>);
impl OPTWERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTWERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTWERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTWERR` writer - OPTWERR"]
pub struct OPTWERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTWERR_W<'a> {
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
#[doc = "Field `OPTVERR` reader - OPTVERR"]
pub struct OPTVERR_R(crate::FieldReader<bool, bool>);
impl OPTVERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTVERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTVERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTVERR` writer - OPTVERR"]
pub struct OPTVERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTVERR_W<'a> {
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
#[doc = "Field `NSBSY` reader - NSBusy"]
pub struct NSBSY_R(crate::FieldReader<bool, bool>);
impl NSBSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSBSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSBSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - NSEOP"]
    #[inline(always)]
    pub fn nseop(&self) -> NSEOP_R {
        NSEOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NSOPERR"]
    #[inline(always)]
    pub fn nsoperr(&self) -> NSOPERR_R {
        NSOPERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NSPROGERR"]
    #[inline(always)]
    pub fn nsprogerr(&self) -> NSPROGERR_R {
        NSPROGERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NSWRPERR"]
    #[inline(always)]
    pub fn nswrperr(&self) -> NSWRPERR_R {
        NSWRPERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - NSPGAERR"]
    #[inline(always)]
    pub fn nspgaerr(&self) -> NSPGAERR_R {
        NSPGAERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - NSSIZERR"]
    #[inline(always)]
    pub fn nssizerr(&self) -> NSSIZERR_R {
        NSSIZERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - NSPGSERR"]
    #[inline(always)]
    pub fn nspgserr(&self) -> NSPGSERR_R {
        NSPGSERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 13 - OPTWERR"]
    #[inline(always)]
    pub fn optwerr(&self) -> OPTWERR_R {
        OPTWERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - OPTVERR"]
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - NSBusy"]
    #[inline(always)]
    pub fn nsbsy(&self) -> NSBSY_R {
        NSBSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NSEOP"]
    #[inline(always)]
    pub fn nseop(&mut self) -> NSEOP_W {
        NSEOP_W { w: self }
    }
    #[doc = "Bit 1 - NSOPERR"]
    #[inline(always)]
    pub fn nsoperr(&mut self) -> NSOPERR_W {
        NSOPERR_W { w: self }
    }
    #[doc = "Bit 3 - NSPROGERR"]
    #[inline(always)]
    pub fn nsprogerr(&mut self) -> NSPROGERR_W {
        NSPROGERR_W { w: self }
    }
    #[doc = "Bit 4 - NSWRPERR"]
    #[inline(always)]
    pub fn nswrperr(&mut self) -> NSWRPERR_W {
        NSWRPERR_W { w: self }
    }
    #[doc = "Bit 5 - NSPGAERR"]
    #[inline(always)]
    pub fn nspgaerr(&mut self) -> NSPGAERR_W {
        NSPGAERR_W { w: self }
    }
    #[doc = "Bit 6 - NSSIZERR"]
    #[inline(always)]
    pub fn nssizerr(&mut self) -> NSSIZERR_W {
        NSSIZERR_W { w: self }
    }
    #[doc = "Bit 7 - NSPGSERR"]
    #[inline(always)]
    pub fn nspgserr(&mut self) -> NSPGSERR_W {
        NSPGSERR_W { w: self }
    }
    #[doc = "Bit 13 - OPTWERR"]
    #[inline(always)]
    pub fn optwerr(&mut self) -> OPTWERR_W {
        OPTWERR_W { w: self }
    }
    #[doc = "Bit 15 - OPTVERR"]
    #[inline(always)]
    pub fn optverr(&mut self) -> OPTVERR_W {
        OPTVERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nssr](index.html) module"]
pub struct NSSR_SPEC;
impl crate::RegisterSpec for NSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nssr::R](R) reader structure"]
impl crate::Readable for NSSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nssr::W](W) writer structure"]
impl crate::Writable for NSSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NSSR to value 0"]
impl crate::Resettable for NSSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
