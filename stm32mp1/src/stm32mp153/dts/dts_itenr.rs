#[doc = "Register `DTS_ITENR` reader"]
pub struct R(crate::R<DTS_ITENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_ITENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_ITENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_ITENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTS_ITENR` writer"]
pub struct W(crate::W<DTS_ITENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTS_ITENR_SPEC>;
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
impl From<crate::W<DTS_ITENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTS_ITENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS1_ITEEN` reader - TS1_ITEEN"]
pub struct TS1_ITEEN_R(crate::FieldReader<bool, bool>);
impl TS1_ITEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_ITEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_ITEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_ITEEN` writer - TS1_ITEEN"]
pub struct TS1_ITEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_ITEEN_W<'a> {
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
#[doc = "Field `TS1_ITLEN` reader - TS1_ITLEN"]
pub struct TS1_ITLEN_R(crate::FieldReader<bool, bool>);
impl TS1_ITLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_ITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_ITLEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_ITLEN` writer - TS1_ITLEN"]
pub struct TS1_ITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_ITLEN_W<'a> {
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
#[doc = "Field `TS1_ITHEN` reader - TS1_ITHEN"]
pub struct TS1_ITHEN_R(crate::FieldReader<bool, bool>);
impl TS1_ITHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_ITHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_ITHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_ITHEN` writer - TS1_ITHEN"]
pub struct TS1_ITHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_ITHEN_W<'a> {
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
#[doc = "Field `TS1_AITEEN` reader - TS1_AITEEN"]
pub struct TS1_AITEEN_R(crate::FieldReader<bool, bool>);
impl TS1_AITEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_AITEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_AITEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_AITEEN` writer - TS1_AITEEN"]
pub struct TS1_AITEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_AITEEN_W<'a> {
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
#[doc = "Field `TS1_AITLEN` reader - TS1_AITLEN"]
pub struct TS1_AITLEN_R(crate::FieldReader<bool, bool>);
impl TS1_AITLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_AITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_AITLEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_AITLEN` writer - TS1_AITLEN"]
pub struct TS1_AITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_AITLEN_W<'a> {
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
#[doc = "Field `TS1_AITHEN` reader - TS1_AITHEN"]
pub struct TS1_AITHEN_R(crate::FieldReader<bool, bool>);
impl TS1_AITHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_AITHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_AITHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_AITHEN` writer - TS1_AITHEN"]
pub struct TS1_AITHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_AITHEN_W<'a> {
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
    #[doc = "Bit 0 - TS1_ITEEN"]
    #[inline(always)]
    pub fn ts1_iteen(&self) -> TS1_ITEEN_R {
        TS1_ITEEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TS1_ITLEN"]
    #[inline(always)]
    pub fn ts1_itlen(&self) -> TS1_ITLEN_R {
        TS1_ITLEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TS1_ITHEN"]
    #[inline(always)]
    pub fn ts1_ithen(&self) -> TS1_ITHEN_R {
        TS1_ITHEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TS1_AITEEN"]
    #[inline(always)]
    pub fn ts1_aiteen(&self) -> TS1_AITEEN_R {
        TS1_AITEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TS1_AITLEN"]
    #[inline(always)]
    pub fn ts1_aitlen(&self) -> TS1_AITLEN_R {
        TS1_AITLEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TS1_AITHEN"]
    #[inline(always)]
    pub fn ts1_aithen(&self) -> TS1_AITHEN_R {
        TS1_AITHEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TS1_ITEEN"]
    #[inline(always)]
    pub fn ts1_iteen(&mut self) -> TS1_ITEEN_W {
        TS1_ITEEN_W { w: self }
    }
    #[doc = "Bit 1 - TS1_ITLEN"]
    #[inline(always)]
    pub fn ts1_itlen(&mut self) -> TS1_ITLEN_W {
        TS1_ITLEN_W { w: self }
    }
    #[doc = "Bit 2 - TS1_ITHEN"]
    #[inline(always)]
    pub fn ts1_ithen(&mut self) -> TS1_ITHEN_W {
        TS1_ITHEN_W { w: self }
    }
    #[doc = "Bit 4 - TS1_AITEEN"]
    #[inline(always)]
    pub fn ts1_aiteen(&mut self) -> TS1_AITEEN_W {
        TS1_AITEEN_W { w: self }
    }
    #[doc = "Bit 5 - TS1_AITLEN"]
    #[inline(always)]
    pub fn ts1_aitlen(&mut self) -> TS1_AITLEN_W {
        TS1_AITLEN_W { w: self }
    }
    #[doc = "Bit 6 - TS1_AITHEN"]
    #[inline(always)]
    pub fn ts1_aithen(&mut self) -> TS1_AITHEN_W {
        TS1_AITHEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temperature sensor interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_itenr](index.html) module"]
pub struct DTS_ITENR_SPEC;
impl crate::RegisterSpec for DTS_ITENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_itenr::R](R) reader structure"]
impl crate::Readable for DTS_ITENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dts_itenr::W](W) writer structure"]
impl crate::Writable for DTS_ITENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTS_ITENR to value 0"]
impl crate::Resettable for DTS_ITENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
