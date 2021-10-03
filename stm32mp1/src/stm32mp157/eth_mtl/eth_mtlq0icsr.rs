#[doc = "Register `ETH_MTLQ0ICSR` reader"]
pub struct R(crate::R<ETH_MTLQ0ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLQ0ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLQ0ICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLQ0ICSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLQ0ICSR` writer"]
pub struct W(crate::W<ETH_MTLQ0ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLQ0ICSR_SPEC>;
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
impl From<crate::W<ETH_MTLQ0ICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLQ0ICSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXUNFIS` reader - TXUNFIS"]
pub struct TXUNFIS_R(crate::FieldReader<bool, bool>);
impl TXUNFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABPSIS` reader - ABPSIS"]
pub struct ABPSIS_R(crate::FieldReader<bool, bool>);
impl ABPSIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABPSIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABPSIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABPSIS` writer - ABPSIS"]
pub struct ABPSIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ABPSIS_W<'a> {
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
#[doc = "Field `TXUIE` reader - TXUIE"]
pub struct TXUIE_R(crate::FieldReader<bool, bool>);
impl TXUIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUIE` writer - TXUIE"]
pub struct TXUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUIE_W<'a> {
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
#[doc = "Field `ABPSIE` reader - ABPSIE"]
pub struct ABPSIE_R(crate::FieldReader<bool, bool>);
impl ABPSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABPSIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABPSIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABPSIE` writer - ABPSIE"]
pub struct ABPSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABPSIE_W<'a> {
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
#[doc = "Field `RXOVFIS` reader - RXOVFIS"]
pub struct RXOVFIS_R(crate::FieldReader<bool, bool>);
impl RXOVFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVFIS` writer - RXOVFIS"]
pub struct RXOVFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVFIS_W<'a> {
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
#[doc = "Field `RXOIE` reader - RXOIE"]
pub struct RXOIE_R(crate::FieldReader<bool, bool>);
impl RXOIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOIE` writer - RXOIE"]
pub struct RXOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TXUNFIS"]
    #[inline(always)]
    pub fn txunfis(&self) -> TXUNFIS_R {
        TXUNFIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ABPSIS"]
    #[inline(always)]
    pub fn abpsis(&self) -> ABPSIS_R {
        ABPSIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TXUIE"]
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ABPSIE"]
    #[inline(always)]
    pub fn abpsie(&self) -> ABPSIE_R {
        ABPSIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RXOVFIS"]
    #[inline(always)]
    pub fn rxovfis(&self) -> RXOVFIS_R {
        RXOVFIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ABPSIS"]
    #[inline(always)]
    pub fn abpsis(&mut self) -> ABPSIS_W {
        ABPSIS_W { w: self }
    }
    #[doc = "Bit 8 - TXUIE"]
    #[inline(always)]
    pub fn txuie(&mut self) -> TXUIE_W {
        TXUIE_W { w: self }
    }
    #[doc = "Bit 9 - ABPSIE"]
    #[inline(always)]
    pub fn abpsie(&mut self) -> ABPSIE_W {
        ABPSIE_W { w: self }
    }
    #[doc = "Bit 16 - RXOVFIS"]
    #[inline(always)]
    pub fn rxovfis(&mut self) -> RXOVFIS_W {
        RXOVFIS_W { w: self }
    }
    #[doc = "Bit 24 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W {
        RXOIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Queue 0 interrupt control status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlq0icsr](index.html) module"]
pub struct ETH_MTLQ0ICSR_SPEC;
impl crate::RegisterSpec for ETH_MTLQ0ICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtlq0icsr::R](R) reader structure"]
impl crate::Readable for ETH_MTLQ0ICSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtlq0icsr::W](W) writer structure"]
impl crate::Writable for ETH_MTLQ0ICSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MTLQ0ICSR to value 0"]
impl crate::Resettable for ETH_MTLQ0ICSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
