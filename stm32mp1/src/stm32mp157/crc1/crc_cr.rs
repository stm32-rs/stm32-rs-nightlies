#[doc = "Register `CRC_CR` reader"]
pub struct R(crate::R<CRC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_CR` writer"]
pub struct W(crate::W<CRC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_CR_SPEC>;
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
impl From<crate::W<CRC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET` reader - RESET"]
pub struct RESET_R(crate::FieldReader<bool, bool>);
impl RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - RESET"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
#[doc = "Field `POLYSIZE` reader - POLYSIZE"]
pub struct POLYSIZE_R(crate::FieldReader<u8, u8>);
impl POLYSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        POLYSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLYSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLYSIZE` writer - POLYSIZE"]
pub struct POLYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> POLYSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `REV_IN` reader - REV_IN"]
pub struct REV_IN_R(crate::FieldReader<u8, u8>);
impl REV_IN_R {
    pub(crate) fn new(bits: u8) -> Self {
        REV_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV_IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV_IN` writer - REV_IN"]
pub struct REV_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `REV_OUT` reader - REV_OUT"]
pub struct REV_OUT_R(crate::FieldReader<bool, bool>);
impl REV_OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV_OUT` writer - REV_OUT"]
pub struct REV_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_OUT_W<'a> {
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
    #[doc = "Bit 0 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - POLYSIZE"]
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - REV_IN"]
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - REV_OUT"]
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bits 3:4 - POLYSIZE"]
    #[inline(always)]
    pub fn polysize(&mut self) -> POLYSIZE_W {
        POLYSIZE_W { w: self }
    }
    #[doc = "Bits 5:6 - REV_IN"]
    #[inline(always)]
    pub fn rev_in(&mut self) -> REV_IN_W {
        REV_IN_W { w: self }
    }
    #[doc = "Bit 7 - REV_OUT"]
    #[inline(always)]
    pub fn rev_out(&mut self) -> REV_OUT_W {
        REV_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_cr](index.html) module"]
pub struct CRC_CR_SPEC;
impl crate::RegisterSpec for CRC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_cr::R](R) reader structure"]
impl crate::Readable for CRC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_cr::W](W) writer structure"]
impl crate::Writable for CRC_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC_CR to value 0"]
impl crate::Resettable for CRC_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
