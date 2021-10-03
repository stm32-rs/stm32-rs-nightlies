#[doc = "Register `DDRPHYC_DTPR1` reader"]
pub struct R(crate::R<DDRPHYC_DTPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DTPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DTPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DTPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DTPR1` writer"]
pub struct W(crate::W<DDRPHYC_DTPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DTPR1_SPEC>;
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
impl From<crate::W<DDRPHYC_DTPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DTPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAOND` reader - TAOND"]
pub struct TAOND_R(crate::FieldReader<u8, u8>);
impl TAOND_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAOND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAOND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAOND` writer - TAOND"]
pub struct TAOND_W<'a> {
    w: &'a mut W,
}
impl<'a> TAOND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TRTW` reader - TRTW"]
pub struct TRTW_R(crate::FieldReader<bool, bool>);
impl TRTW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRTW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRTW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRTW` writer - TRTW"]
pub struct TRTW_W<'a> {
    w: &'a mut W,
}
impl<'a> TRTW_W<'a> {
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
#[doc = "Field `TFAW` reader - TFAW"]
pub struct TFAW_R(crate::FieldReader<u8, u8>);
impl TFAW_R {
    pub(crate) fn new(bits: u8) -> Self {
        TFAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFAW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFAW` writer - TFAW"]
pub struct TFAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TFAW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 3)) | ((value as u32 & 0x3f) << 3);
        self.w
    }
}
#[doc = "Field `TMOD` reader - TMOD"]
pub struct TMOD_R(crate::FieldReader<u8, u8>);
impl TMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMOD` writer - TMOD"]
pub struct TMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `TRTODT` reader - TRTODT"]
pub struct TRTODT_R(crate::FieldReader<bool, bool>);
impl TRTODT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRTODT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRTODT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRTODT` writer - TRTODT"]
pub struct TRTODT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRTODT_W<'a> {
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
#[doc = "Field `TRFC` reader - TRFC"]
pub struct TRFC_R(crate::FieldReader<u8, u8>);
impl TRFC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRFC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRFC` writer - TRFC"]
pub struct TRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRFC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `TDQSCKMIN` reader - TDQSCKMIN"]
pub struct TDQSCKMIN_R(crate::FieldReader<u8, u8>);
impl TDQSCKMIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDQSCKMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDQSCKMIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDQSCKMIN` writer - TDQSCKMIN"]
pub struct TDQSCKMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDQSCKMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `TDQSCKMAX` reader - TDQSCKMAX"]
pub struct TDQSCKMAX_R(crate::FieldReader<u8, u8>);
impl TDQSCKMAX_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDQSCKMAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDQSCKMAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDQSCKMAX` writer - TDQSCKMAX"]
pub struct TDQSCKMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> TDQSCKMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - TAOND"]
    #[inline(always)]
    pub fn taond(&self) -> TAOND_R {
        TAOND_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - TRTW"]
    #[inline(always)]
    pub fn trtw(&self) -> TRTW_R {
        TRTW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:8 - TFAW"]
    #[inline(always)]
    pub fn tfaw(&self) -> TFAW_R {
        TFAW_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bits 9:10 - TMOD"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - TRTODT"]
    #[inline(always)]
    pub fn trtodt(&self) -> TRTODT_R {
        TRTODT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - TRFC"]
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - TDQSCKMIN"]
    #[inline(always)]
    pub fn tdqsckmin(&self) -> TDQSCKMIN_R {
        TDQSCKMIN_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 27:29 - TDQSCKMAX"]
    #[inline(always)]
    pub fn tdqsckmax(&self) -> TDQSCKMAX_R {
        TDQSCKMAX_R::new(((self.bits >> 27) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TAOND"]
    #[inline(always)]
    pub fn taond(&mut self) -> TAOND_W {
        TAOND_W { w: self }
    }
    #[doc = "Bit 2 - TRTW"]
    #[inline(always)]
    pub fn trtw(&mut self) -> TRTW_W {
        TRTW_W { w: self }
    }
    #[doc = "Bits 3:8 - TFAW"]
    #[inline(always)]
    pub fn tfaw(&mut self) -> TFAW_W {
        TFAW_W { w: self }
    }
    #[doc = "Bits 9:10 - TMOD"]
    #[inline(always)]
    pub fn tmod(&mut self) -> TMOD_W {
        TMOD_W { w: self }
    }
    #[doc = "Bit 11 - TRTODT"]
    #[inline(always)]
    pub fn trtodt(&mut self) -> TRTODT_W {
        TRTODT_W { w: self }
    }
    #[doc = "Bits 16:23 - TRFC"]
    #[inline(always)]
    pub fn trfc(&mut self) -> TRFC_W {
        TRFC_W { w: self }
    }
    #[doc = "Bits 24:26 - TDQSCKMIN"]
    #[inline(always)]
    pub fn tdqsckmin(&mut self) -> TDQSCKMIN_W {
        TDQSCKMIN_W { w: self }
    }
    #[doc = "Bits 27:29 - TDQSCKMAX"]
    #[inline(always)]
    pub fn tdqsckmax(&mut self) -> TDQSCKMAX_W {
        TDQSCKMAX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DTP register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtpr1](index.html) module"]
pub struct DDRPHYC_DTPR1_SPEC;
impl crate::RegisterSpec for DDRPHYC_DTPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dtpr1::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtpr1::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DTPR1 to value 0x0a03_0090"]
impl crate::Resettable for DDRPHYC_DTPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a03_0090
    }
}
