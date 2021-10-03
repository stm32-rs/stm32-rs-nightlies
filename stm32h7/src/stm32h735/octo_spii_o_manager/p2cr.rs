#[doc = "Register `P2CR` reader"]
pub struct R(crate::R<P2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2CR` writer"]
pub struct W(crate::W<P2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2CR_SPEC>;
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
impl From<crate::W<P2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKEN` reader - CLKEN"]
pub struct CLKEN_R(crate::FieldReader<bool, bool>);
impl CLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKEN` writer - CLKEN"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
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
#[doc = "Field `CLKSRC` reader - CLKSRC"]
pub struct CLKSRC_R(crate::FieldReader<bool, bool>);
impl CLKSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKSRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSRC` writer - CLKSRC"]
pub struct CLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSRC_W<'a> {
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
#[doc = "Field `DQSEN` reader - DQSEN"]
pub struct DQSEN_R(crate::FieldReader<bool, bool>);
impl DQSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DQSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSEN` writer - DQSEN"]
pub struct DQSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSEN_W<'a> {
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
#[doc = "Field `DQSSRC` reader - DQSSRC"]
pub struct DQSSRC_R(crate::FieldReader<bool, bool>);
impl DQSSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DQSSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSSRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSSRC` writer - DQSSRC"]
pub struct DQSSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSSRC_W<'a> {
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
#[doc = "Field `NCSEN` reader - NCSEN"]
pub struct NCSEN_R(crate::FieldReader<bool, bool>);
impl NCSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        NCSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCSEN` writer - NCSEN"]
pub struct NCSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NCSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `NCSSRC` reader - NCSSRC"]
pub struct NCSSRC_R(crate::FieldReader<bool, bool>);
impl NCSSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        NCSSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCSSRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCSSRC` writer - NCSSRC"]
pub struct NCSSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> NCSSRC_W<'a> {
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
#[doc = "Field `IOLEN` reader - IOLEN"]
pub struct IOLEN_R(crate::FieldReader<bool, bool>);
impl IOLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOLEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOLEN` writer - IOLEN"]
pub struct IOLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOLEN_W<'a> {
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
#[doc = "Field `IOLSRC` reader - IOLSRC"]
pub struct IOLSRC_R(crate::FieldReader<u8, u8>);
impl IOLSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        IOLSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOLSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOLSRC` writer - IOLSRC"]
pub struct IOLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> IOLSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `IOHEN` reader - IOHEN"]
pub struct IOHEN_R(crate::FieldReader<bool, bool>);
impl IOHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOHEN` writer - IOHEN"]
pub struct IOHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOHEN_W<'a> {
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
#[doc = "Field `IOHSRC` reader - IOHSRC"]
pub struct IOHSRC_R(crate::FieldReader<u8, u8>);
impl IOHSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        IOHSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOHSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOHSRC` writer - IOHSRC"]
pub struct IOHSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> IOHSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CLKEN"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CLKSRC"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DQSEN"]
    #[inline(always)]
    pub fn dqsen(&self) -> DQSEN_R {
        DQSEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DQSSRC"]
    #[inline(always)]
    pub fn dqssrc(&self) -> DQSSRC_R {
        DQSSRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - NCSEN"]
    #[inline(always)]
    pub fn ncsen(&self) -> NCSEN_R {
        NCSEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - NCSSRC"]
    #[inline(always)]
    pub fn ncssrc(&self) -> NCSSRC_R {
        NCSSRC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IOLEN"]
    #[inline(always)]
    pub fn iolen(&self) -> IOLEN_R {
        IOLEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - IOLSRC"]
    #[inline(always)]
    pub fn iolsrc(&self) -> IOLSRC_R {
        IOLSRC_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 24 - IOHEN"]
    #[inline(always)]
    pub fn iohen(&self) -> IOHEN_R {
        IOHEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - IOHSRC"]
    #[inline(always)]
    pub fn iohsrc(&self) -> IOHSRC_R {
        IOHSRC_R::new(((self.bits >> 25) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CLKEN"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Bit 1 - CLKSRC"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W {
        CLKSRC_W { w: self }
    }
    #[doc = "Bit 4 - DQSEN"]
    #[inline(always)]
    pub fn dqsen(&mut self) -> DQSEN_W {
        DQSEN_W { w: self }
    }
    #[doc = "Bit 5 - DQSSRC"]
    #[inline(always)]
    pub fn dqssrc(&mut self) -> DQSSRC_W {
        DQSSRC_W { w: self }
    }
    #[doc = "Bit 8 - NCSEN"]
    #[inline(always)]
    pub fn ncsen(&mut self) -> NCSEN_W {
        NCSEN_W { w: self }
    }
    #[doc = "Bit 9 - NCSSRC"]
    #[inline(always)]
    pub fn ncssrc(&mut self) -> NCSSRC_W {
        NCSSRC_W { w: self }
    }
    #[doc = "Bit 16 - IOLEN"]
    #[inline(always)]
    pub fn iolen(&mut self) -> IOLEN_W {
        IOLEN_W { w: self }
    }
    #[doc = "Bits 17:18 - IOLSRC"]
    #[inline(always)]
    pub fn iolsrc(&mut self) -> IOLSRC_W {
        IOLSRC_W { w: self }
    }
    #[doc = "Bit 24 - IOHEN"]
    #[inline(always)]
    pub fn iohen(&mut self) -> IOHEN_W {
        IOHEN_W { w: self }
    }
    #[doc = "Bits 25:26 - IOHSRC"]
    #[inline(always)]
    pub fn iohsrc(&mut self) -> IOHSRC_W {
        IOHSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OctoSPI IO Manager Port 2 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2cr](index.html) module"]
pub struct P2CR_SPEC;
impl crate::RegisterSpec for P2CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [p2cr::R](R) reader structure"]
impl crate::Readable for P2CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2cr::W](W) writer structure"]
impl crate::Writable for P2CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2CR to value 0x0705_0333"]
impl crate::Resettable for P2CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0705_0333
    }
}
