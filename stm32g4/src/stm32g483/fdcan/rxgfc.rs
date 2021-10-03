#[doc = "Register `RXGFC` reader"]
pub struct R(crate::R<RXGFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXGFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXGFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXGFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXGFC` writer"]
pub struct W(crate::W<RXGFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXGFC_SPEC>;
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
impl From<crate::W<RXGFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXGFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RRFE` reader - RRFE"]
pub struct RRFE_R(crate::FieldReader<bool, bool>);
impl RRFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRFE` writer - RRFE"]
pub struct RRFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRFE_W<'a> {
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
#[doc = "Field `RRFS` reader - RRFS"]
pub struct RRFS_R(crate::FieldReader<bool, bool>);
impl RRFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRFS` writer - RRFS"]
pub struct RRFS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRFS_W<'a> {
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
#[doc = "Field `ANFE` writer - ANFE"]
pub struct ANFE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `ANFS` writer - ANFS"]
pub struct ANFS_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `LSE` reader - List size extended"]
pub struct LSE_R(crate::FieldReader<u8, u8>);
impl LSE_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSE` writer - List size extended"]
pub struct LSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `LSS` reader - List size standard"]
pub struct LSS_R(crate::FieldReader<u8, u8>);
impl LSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSS` writer - List size standard"]
pub struct LSS_W<'a> {
    w: &'a mut W,
}
impl<'a> LSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `F0OM` reader - FIFO 0 operation mode"]
pub struct F0OM_R(crate::FieldReader<bool, bool>);
impl F0OM_R {
    pub(crate) fn new(bits: bool) -> Self {
        F0OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0OM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0OM` writer - FIFO 0 operation mode"]
pub struct F0OM_W<'a> {
    w: &'a mut W,
}
impl<'a> F0OM_W<'a> {
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
#[doc = "Field `F1OM` reader - FIFO 1 operation mode"]
pub struct F1OM_R(crate::FieldReader<bool, bool>);
impl F1OM_R {
    pub(crate) fn new(bits: bool) -> Self {
        F1OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1OM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1OM` writer - FIFO 1 operation mode"]
pub struct F1OM_W<'a> {
    w: &'a mut W,
}
impl<'a> F1OM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - List size extended"]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - List size standard"]
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - FIFO 0 operation mode"]
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FIFO 1 operation mode"]
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    pub fn rrfe(&mut self) -> RRFE_W {
        RRFE_W { w: self }
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    pub fn rrfs(&mut self) -> RRFS_W {
        RRFS_W { w: self }
    }
    #[doc = "Bits 2:3 - ANFE"]
    #[inline(always)]
    pub fn anfe(&mut self) -> ANFE_W {
        ANFE_W { w: self }
    }
    #[doc = "Bits 4:5 - ANFS"]
    #[inline(always)]
    pub fn anfs(&mut self) -> ANFS_W {
        ANFS_W { w: self }
    }
    #[doc = "Bits 24:27 - List size extended"]
    #[inline(always)]
    pub fn lse(&mut self) -> LSE_W {
        LSE_W { w: self }
    }
    #[doc = "Bits 16:20 - List size standard"]
    #[inline(always)]
    pub fn lss(&mut self) -> LSS_W {
        LSS_W { w: self }
    }
    #[doc = "Bit 9 - FIFO 0 operation mode"]
    #[inline(always)]
    pub fn f0om(&mut self) -> F0OM_W {
        F0OM_W { w: self }
    }
    #[doc = "Bit 8 - FIFO 1 operation mode"]
    #[inline(always)]
    pub fn f1om(&mut self) -> F1OM_W {
        F1OM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxgfc](index.html) module"]
pub struct RXGFC_SPEC;
impl crate::RegisterSpec for RXGFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxgfc::R](R) reader structure"]
impl crate::Readable for RXGFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxgfc::W](W) writer structure"]
impl crate::Writable for RXGFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXGFC to value 0"]
impl crate::Resettable for RXGFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
