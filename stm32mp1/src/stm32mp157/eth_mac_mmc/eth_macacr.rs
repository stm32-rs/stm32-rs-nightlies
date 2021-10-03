#[doc = "Register `ETH_MACACR` reader"]
pub struct R(crate::R<ETH_MACACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACACR` writer"]
pub struct W(crate::W<ETH_MACACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACACR_SPEC>;
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
impl From<crate::W<ETH_MACACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATSFC` reader - ATSFC"]
pub struct ATSFC_R(crate::FieldReader<bool, bool>);
impl ATSFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATSFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATSFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATSFC` writer - ATSFC"]
pub struct ATSFC_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSFC_W<'a> {
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
#[doc = "Field `ATSEN0` reader - ATSEN0"]
pub struct ATSEN0_R(crate::FieldReader<bool, bool>);
impl ATSEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATSEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATSEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATSEN0` writer - ATSEN0"]
pub struct ATSEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSEN0_W<'a> {
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
#[doc = "Field `ATSEN1` reader - ATSEN1"]
pub struct ATSEN1_R(crate::FieldReader<bool, bool>);
impl ATSEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATSEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATSEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATSEN1` writer - ATSEN1"]
pub struct ATSEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSEN1_W<'a> {
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
#[doc = "Field `ATSEN2` reader - ATSEN2"]
pub struct ATSEN2_R(crate::FieldReader<bool, bool>);
impl ATSEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATSEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATSEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATSEN2` writer - ATSEN2"]
pub struct ATSEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSEN2_W<'a> {
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
#[doc = "Field `ATSEN3` reader - ATSEN3"]
pub struct ATSEN3_R(crate::FieldReader<bool, bool>);
impl ATSEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATSEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATSEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATSEN3` writer - ATSEN3"]
pub struct ATSEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSEN3_W<'a> {
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
    #[doc = "Bit 0 - ATSFC"]
    #[inline(always)]
    pub fn atsfc(&self) -> ATSFC_R {
        ATSFC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - ATSEN0"]
    #[inline(always)]
    pub fn atsen0(&self) -> ATSEN0_R {
        ATSEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ATSEN1"]
    #[inline(always)]
    pub fn atsen1(&self) -> ATSEN1_R {
        ATSEN1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ATSEN2"]
    #[inline(always)]
    pub fn atsen2(&self) -> ATSEN2_R {
        ATSEN2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ATSEN3"]
    #[inline(always)]
    pub fn atsen3(&self) -> ATSEN3_R {
        ATSEN3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ATSFC"]
    #[inline(always)]
    pub fn atsfc(&mut self) -> ATSFC_W {
        ATSFC_W { w: self }
    }
    #[doc = "Bit 4 - ATSEN0"]
    #[inline(always)]
    pub fn atsen0(&mut self) -> ATSEN0_W {
        ATSEN0_W { w: self }
    }
    #[doc = "Bit 5 - ATSEN1"]
    #[inline(always)]
    pub fn atsen1(&mut self) -> ATSEN1_W {
        ATSEN1_W { w: self }
    }
    #[doc = "Bit 6 - ATSEN2"]
    #[inline(always)]
    pub fn atsen2(&mut self) -> ATSEN2_W {
        ATSEN2_W { w: self }
    }
    #[doc = "Bit 7 - ATSEN3"]
    #[inline(always)]
    pub fn atsen3(&mut self) -> ATSEN3_W {
        ATSEN3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macacr](index.html) module"]
pub struct ETH_MACACR_SPEC;
impl crate::RegisterSpec for ETH_MACACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macacr::R](R) reader structure"]
impl crate::Readable for ETH_MACACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macacr::W](W) writer structure"]
impl crate::Writable for ETH_MACACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACACR to value 0"]
impl crate::Resettable for ETH_MACACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
