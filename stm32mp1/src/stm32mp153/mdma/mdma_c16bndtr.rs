#[doc = "Register `MDMA_C16BNDTR` reader"]
pub struct R(crate::R<MDMA_C16BNDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C16BNDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C16BNDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C16BNDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMA_C16BNDTR` writer"]
pub struct W(crate::W<MDMA_C16BNDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C16BNDTR_SPEC>;
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
impl From<crate::W<MDMA_C16BNDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C16BNDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNDT` reader - BNDT"]
pub struct BNDT_R(crate::FieldReader<u32, u32>);
impl BNDT_R {
    pub(crate) fn new(bits: u32) -> Self {
        BNDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BNDT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNDT` writer - BNDT"]
pub struct BNDT_W<'a> {
    w: &'a mut W,
}
impl<'a> BNDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
#[doc = "Field `BRSUM` reader - BRSUM"]
pub struct BRSUM_R(crate::FieldReader<bool, bool>);
impl BRSUM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRSUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRSUM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRSUM` writer - BRSUM"]
pub struct BRSUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSUM_W<'a> {
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
#[doc = "Field `BRDUM` reader - BRDUM"]
pub struct BRDUM_R(crate::FieldReader<bool, bool>);
impl BRDUM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRDUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRDUM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRDUM` writer - BRDUM"]
pub struct BRDUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BRDUM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `BRC` reader - BRC"]
pub struct BRC_R(crate::FieldReader<u16, u16>);
impl BRC_R {
    pub(crate) fn new(bits: u16) -> Self {
        BRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRC` writer - BRC"]
pub struct BRC_W<'a> {
    w: &'a mut W,
}
impl<'a> BRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | ((value as u32 & 0x0fff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - BNDT"]
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 18 - BRSUM"]
    #[inline(always)]
    pub fn brsum(&self) -> BRSUM_R {
        BRSUM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - BRDUM"]
    #[inline(always)]
    pub fn brdum(&self) -> BRDUM_R {
        BRDUM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:31 - BRC"]
    #[inline(always)]
    pub fn brc(&self) -> BRC_R {
        BRC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - BNDT"]
    #[inline(always)]
    pub fn bndt(&mut self) -> BNDT_W {
        BNDT_W { w: self }
    }
    #[doc = "Bit 18 - BRSUM"]
    #[inline(always)]
    pub fn brsum(&mut self) -> BRSUM_W {
        BRSUM_W { w: self }
    }
    #[doc = "Bit 19 - BRDUM"]
    #[inline(always)]
    pub fn brdum(&mut self) -> BRDUM_W {
        BRDUM_W { w: self }
    }
    #[doc = "Bits 20:31 - BRC"]
    #[inline(always)]
    pub fn brc(&mut self) -> BRC_W {
        BRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c16bndtr](index.html) module"]
pub struct MDMA_C16BNDTR_SPEC;
impl crate::RegisterSpec for MDMA_C16BNDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdma_c16bndtr::R](R) reader structure"]
impl crate::Readable for MDMA_C16BNDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdma_c16bndtr::W](W) writer structure"]
impl crate::Writable for MDMA_C16BNDTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDMA_C16BNDTR to value 0"]
impl crate::Resettable for MDMA_C16BNDTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
