#[doc = "Register `SMCR` reader"]
pub struct R(crate::R<SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCR` writer"]
pub struct W(crate::W<SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCR_SPEC>;
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
impl From<crate::W<SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DECPROT` reader - DECPROT"]
pub struct DECPROT_R(crate::FieldReader<bool, bool>);
impl DECPROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DECPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT` writer - DECPROT"]
pub struct DECPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT_W<'a> {
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
#[doc = "Field `INITDPROT` reader - INITDPROT"]
pub struct INITDPROT_R(crate::FieldReader<bool, bool>);
impl INITDPROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITDPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INITDPROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITDPROT` writer - INITDPROT"]
pub struct INITDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> INITDPROT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CALDPROT` reader - CALDPROT"]
pub struct CALDPROT_R(crate::FieldReader<bool, bool>);
impl CALDPROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALDPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALDPROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALDPROT` writer - CALDPROT"]
pub struct CALDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> CALDPROT_W<'a> {
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
#[doc = "Field `TSDPROT` reader - TSDPROT"]
pub struct TSDPROT_R(crate::FieldReader<bool, bool>);
impl TSDPROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSDPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSDPROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSDPROT` writer - TSDPROT"]
pub struct TSDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSDPROT_W<'a> {
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
#[doc = "Field `WUTDPROT` reader - WUTDPROT"]
pub struct WUTDPROT_R(crate::FieldReader<bool, bool>);
impl WUTDPROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTDPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUTDPROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUTDPROT` writer - WUTDPROT"]
pub struct WUTDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTDPROT_W<'a> {
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
#[doc = "Field `ALRBDPROT` reader - ALRBDPROT"]
pub struct ALRBDPROT_R(crate::FieldReader<bool, bool>);
impl ALRBDPROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBDPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRBDPROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRBDPROT` writer - ALRBDPROT"]
pub struct ALRBDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBDPROT_W<'a> {
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
#[doc = "Field `ALRADPROT` reader - ALRADPROT"]
pub struct ALRADPROT_R(crate::FieldReader<bool, bool>);
impl ALRADPROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRADPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRADPROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRADPROT` writer - ALRADPROT"]
pub struct ALRADPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRADPROT_W<'a> {
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
impl R {
    #[doc = "Bit 15 - DECPROT"]
    #[inline(always)]
    pub fn decprot(&self) -> DECPROT_R {
        DECPROT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - INITDPROT"]
    #[inline(always)]
    pub fn initdprot(&self) -> INITDPROT_R {
        INITDPROT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CALDPROT"]
    #[inline(always)]
    pub fn caldprot(&self) -> CALDPROT_R {
        CALDPROT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TSDPROT"]
    #[inline(always)]
    pub fn tsdprot(&self) -> TSDPROT_R {
        TSDPROT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WUTDPROT"]
    #[inline(always)]
    pub fn wutdprot(&self) -> WUTDPROT_R {
        WUTDPROT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ALRBDPROT"]
    #[inline(always)]
    pub fn alrbdprot(&self) -> ALRBDPROT_R {
        ALRBDPROT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ALRADPROT"]
    #[inline(always)]
    pub fn alradprot(&self) -> ALRADPROT_R {
        ALRADPROT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - DECPROT"]
    #[inline(always)]
    pub fn decprot(&mut self) -> DECPROT_W {
        DECPROT_W { w: self }
    }
    #[doc = "Bit 14 - INITDPROT"]
    #[inline(always)]
    pub fn initdprot(&mut self) -> INITDPROT_W {
        INITDPROT_W { w: self }
    }
    #[doc = "Bit 13 - CALDPROT"]
    #[inline(always)]
    pub fn caldprot(&mut self) -> CALDPROT_W {
        CALDPROT_W { w: self }
    }
    #[doc = "Bit 3 - TSDPROT"]
    #[inline(always)]
    pub fn tsdprot(&mut self) -> TSDPROT_W {
        TSDPROT_W { w: self }
    }
    #[doc = "Bit 2 - WUTDPROT"]
    #[inline(always)]
    pub fn wutdprot(&mut self) -> WUTDPROT_W {
        WUTDPROT_W { w: self }
    }
    #[doc = "Bit 1 - ALRBDPROT"]
    #[inline(always)]
    pub fn alrbdprot(&mut self) -> ALRBDPROT_W {
        ALRBDPROT_W { w: self }
    }
    #[doc = "Bit 0 - ALRADPROT"]
    #[inline(always)]
    pub fn alradprot(&mut self) -> ALRADPROT_W {
        ALRADPROT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC secure mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcr](index.html) module"]
pub struct SMCR_SPEC;
impl crate::RegisterSpec for SMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smcr::R](R) reader structure"]
impl crate::Readable for SMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcr::W](W) writer structure"]
impl crate::Writable for SMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMCR to value 0xe00f"]
impl crate::Resettable for SMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe00f
    }
}
