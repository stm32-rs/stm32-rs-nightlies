#[doc = "Register `SPDIFRX_IMR` reader"]
pub struct R(crate::R<SPDIFRX_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDIFRX_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDIFRX_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDIFRX_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPDIFRX_IMR` writer"]
pub struct W(crate::W<SPDIFRX_IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPDIFRX_IMR_SPEC>;
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
impl From<crate::W<SPDIFRX_IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPDIFRX_IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXNEIE` reader - RXNEIE"]
pub struct RXNEIE_R(crate::FieldReader<bool, bool>);
impl RXNEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXNEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXNEIE` writer - RXNEIE"]
pub struct RXNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEIE_W<'a> {
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
#[doc = "Field `CSRNEIE` reader - CSRNEIE"]
pub struct CSRNEIE_R(crate::FieldReader<bool, bool>);
impl CSRNEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSRNEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSRNEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSRNEIE` writer - CSRNEIE"]
pub struct CSRNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRNEIE_W<'a> {
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
#[doc = "Field `PERRIE` reader - PERRIE"]
pub struct PERRIE_R(crate::FieldReader<bool, bool>);
impl PERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERRIE` writer - PERRIE"]
pub struct PERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PERRIE_W<'a> {
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
#[doc = "Field `OVRIE` reader - OVRIE"]
pub struct OVRIE_R(crate::FieldReader<bool, bool>);
impl OVRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRIE` writer - OVRIE"]
pub struct OVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRIE_W<'a> {
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
#[doc = "Field `SBLKIE` reader - SBLKIE"]
pub struct SBLKIE_R(crate::FieldReader<bool, bool>);
impl SBLKIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBLKIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBLKIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBLKIE` writer - SBLKIE"]
pub struct SBLKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SBLKIE_W<'a> {
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
#[doc = "Field `SYNCDIE` reader - SYNCDIE"]
pub struct SYNCDIE_R(crate::FieldReader<bool, bool>);
impl SYNCDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCDIE` writer - SYNCDIE"]
pub struct SYNCDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCDIE_W<'a> {
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
#[doc = "Field `IFEIE` reader - IFEIE"]
pub struct IFEIE_R(crate::FieldReader<bool, bool>);
impl IFEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFEIE` writer - IFEIE"]
pub struct IFEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IFEIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RXNEIE"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CSRNEIE"]
    #[inline(always)]
    pub fn csrneie(&self) -> CSRNEIE_R {
        CSRNEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PERRIE"]
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SBLKIE"]
    #[inline(always)]
    pub fn sblkie(&self) -> SBLKIE_R {
        SBLKIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SYNCDIE"]
    #[inline(always)]
    pub fn syncdie(&self) -> SYNCDIE_R {
        SYNCDIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IFEIE"]
    #[inline(always)]
    pub fn ifeie(&self) -> IFEIE_R {
        IFEIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXNEIE"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W {
        RXNEIE_W { w: self }
    }
    #[doc = "Bit 1 - CSRNEIE"]
    #[inline(always)]
    pub fn csrneie(&mut self) -> CSRNEIE_W {
        CSRNEIE_W { w: self }
    }
    #[doc = "Bit 2 - PERRIE"]
    #[inline(always)]
    pub fn perrie(&mut self) -> PERRIE_W {
        PERRIE_W { w: self }
    }
    #[doc = "Bit 3 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W {
        OVRIE_W { w: self }
    }
    #[doc = "Bit 4 - SBLKIE"]
    #[inline(always)]
    pub fn sblkie(&mut self) -> SBLKIE_W {
        SBLKIE_W { w: self }
    }
    #[doc = "Bit 5 - SYNCDIE"]
    #[inline(always)]
    pub fn syncdie(&mut self) -> SYNCDIE_W {
        SYNCDIE_W { w: self }
    }
    #[doc = "Bit 6 - IFEIE"]
    #[inline(always)]
    pub fn ifeie(&mut self) -> IFEIE_W {
        IFEIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_imr](index.html) module"]
pub struct SPDIFRX_IMR_SPEC;
impl crate::RegisterSpec for SPDIFRX_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdifrx_imr::R](R) reader structure"]
impl crate::Readable for SPDIFRX_IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spdifrx_imr::W](W) writer structure"]
impl crate::Writable for SPDIFRX_IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPDIFRX_IMR to value 0"]
impl crate::Resettable for SPDIFRX_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
