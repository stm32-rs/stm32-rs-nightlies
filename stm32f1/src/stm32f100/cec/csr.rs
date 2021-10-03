#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSOM` reader - Tx start of message"]
pub struct TSOM_R(crate::FieldReader<bool, bool>);
impl TSOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSOM` writer - Tx start of message"]
pub struct TSOM_W<'a> {
    w: &'a mut W,
}
impl<'a> TSOM_W<'a> {
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
#[doc = "Field `TEOM` reader - Tx end of message"]
pub struct TEOM_R(crate::FieldReader<bool, bool>);
impl TEOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEOM` writer - Tx end of message"]
pub struct TEOM_W<'a> {
    w: &'a mut W,
}
impl<'a> TEOM_W<'a> {
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
#[doc = "Field `TERR` reader - Tx error"]
pub struct TERR_R(crate::FieldReader<bool, bool>);
impl TERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERR` writer - Tx error"]
pub struct TERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TERR_W<'a> {
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
#[doc = "Field `TBTRF` reader - Tx byte transfer request or block transfer finished"]
pub struct TBTRF_R(crate::FieldReader<bool, bool>);
impl TBTRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBTRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBTRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBTRF` writer - Tx byte transfer request or block transfer finished"]
pub struct TBTRF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTRF_W<'a> {
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
#[doc = "Field `RSOM` reader - Rx start of message"]
pub struct RSOM_R(crate::FieldReader<bool, bool>);
impl RSOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSOM` writer - Rx start of message"]
pub struct RSOM_W<'a> {
    w: &'a mut W,
}
impl<'a> RSOM_W<'a> {
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
#[doc = "Field `REOM` reader - Rx end of message"]
pub struct REOM_R(crate::FieldReader<bool, bool>);
impl REOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        REOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REOM` writer - Rx end of message"]
pub struct REOM_W<'a> {
    w: &'a mut W,
}
impl<'a> REOM_W<'a> {
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
#[doc = "Field `RERR` reader - Rx error"]
pub struct RERR_R(crate::FieldReader<bool, bool>);
impl RERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RERR` writer - Rx error"]
pub struct RERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RERR_W<'a> {
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
#[doc = "Field `RBTF` reader - Rx byte/block transfer finished"]
pub struct RBTF_R(crate::FieldReader<bool, bool>);
impl RBTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBTF` writer - Rx byte/block transfer finished"]
pub struct RBTF_W<'a> {
    w: &'a mut W,
}
impl<'a> RBTF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Tx start of message"]
    #[inline(always)]
    pub fn tsom(&self) -> TSOM_R {
        TSOM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tx end of message"]
    #[inline(always)]
    pub fn teom(&self) -> TEOM_R {
        TEOM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tx error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Tx byte transfer request or block transfer finished"]
    #[inline(always)]
    pub fn tbtrf(&self) -> TBTRF_R {
        TBTRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx start of message"]
    #[inline(always)]
    pub fn rsom(&self) -> RSOM_R {
        RSOM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx end of message"]
    #[inline(always)]
    pub fn reom(&self) -> REOM_R {
        REOM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx error"]
    #[inline(always)]
    pub fn rerr(&self) -> RERR_R {
        RERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx byte/block transfer finished"]
    #[inline(always)]
    pub fn rbtf(&self) -> RBTF_R {
        RBTF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tx start of message"]
    #[inline(always)]
    pub fn tsom(&mut self) -> TSOM_W {
        TSOM_W { w: self }
    }
    #[doc = "Bit 1 - Tx end of message"]
    #[inline(always)]
    pub fn teom(&mut self) -> TEOM_W {
        TEOM_W { w: self }
    }
    #[doc = "Bit 2 - Tx error"]
    #[inline(always)]
    pub fn terr(&mut self) -> TERR_W {
        TERR_W { w: self }
    }
    #[doc = "Bit 3 - Tx byte transfer request or block transfer finished"]
    #[inline(always)]
    pub fn tbtrf(&mut self) -> TBTRF_W {
        TBTRF_W { w: self }
    }
    #[doc = "Bit 4 - Rx start of message"]
    #[inline(always)]
    pub fn rsom(&mut self) -> RSOM_W {
        RSOM_W { w: self }
    }
    #[doc = "Bit 5 - Rx end of message"]
    #[inline(always)]
    pub fn reom(&mut self) -> REOM_W {
        REOM_W { w: self }
    }
    #[doc = "Bit 6 - Rx error"]
    #[inline(always)]
    pub fn rerr(&mut self) -> RERR_W {
        RERR_W { w: self }
    }
    #[doc = "Bit 7 - Rx byte/block transfer finished"]
    #[inline(always)]
    pub fn rbtf(&mut self) -> RBTF_W {
        RBTF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CEC control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
