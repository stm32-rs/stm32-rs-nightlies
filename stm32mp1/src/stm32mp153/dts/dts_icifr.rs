#[doc = "Register `DTS_ICIFR` reader"]
pub struct R(crate::R<DTS_ICIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_ICIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_ICIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_ICIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTS_ICIFR` writer"]
pub struct W(crate::W<DTS_ICIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTS_ICIFR_SPEC>;
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
impl From<crate::W<DTS_ICIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTS_ICIFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS1_CITEF` reader - TS1_CITEF"]
pub struct TS1_CITEF_R(crate::FieldReader<bool, bool>);
impl TS1_CITEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_CITEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_CITEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_CITEF` writer - TS1_CITEF"]
pub struct TS1_CITEF_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_CITEF_W<'a> {
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
#[doc = "Field `TS1_CITLF` reader - TS1_CITLF"]
pub struct TS1_CITLF_R(crate::FieldReader<bool, bool>);
impl TS1_CITLF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_CITLF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_CITLF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_CITLF` writer - TS1_CITLF"]
pub struct TS1_CITLF_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_CITLF_W<'a> {
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
#[doc = "Field `TS1_CITHF` reader - TS1_CITHF"]
pub struct TS1_CITHF_R(crate::FieldReader<bool, bool>);
impl TS1_CITHF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_CITHF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_CITHF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_CITHF` writer - TS1_CITHF"]
pub struct TS1_CITHF_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_CITHF_W<'a> {
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
#[doc = "Field `TS1_CAITEF` reader - TS1_CAITEF"]
pub struct TS1_CAITEF_R(crate::FieldReader<bool, bool>);
impl TS1_CAITEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_CAITEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_CAITEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_CAITEF` writer - TS1_CAITEF"]
pub struct TS1_CAITEF_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_CAITEF_W<'a> {
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
#[doc = "Field `TS1_CAITLF` reader - TS1_CAITLF"]
pub struct TS1_CAITLF_R(crate::FieldReader<bool, bool>);
impl TS1_CAITLF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_CAITLF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_CAITLF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_CAITLF` writer - TS1_CAITLF"]
pub struct TS1_CAITLF_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_CAITLF_W<'a> {
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
#[doc = "Field `TS1_CAITHF` reader - TS1_CAITHF"]
pub struct TS1_CAITHF_R(crate::FieldReader<bool, bool>);
impl TS1_CAITHF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_CAITHF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_CAITHF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_CAITHF` writer - TS1_CAITHF"]
pub struct TS1_CAITHF_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_CAITHF_W<'a> {
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
    #[doc = "Bit 0 - TS1_CITEF"]
    #[inline(always)]
    pub fn ts1_citef(&self) -> TS1_CITEF_R {
        TS1_CITEF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TS1_CITLF"]
    #[inline(always)]
    pub fn ts1_citlf(&self) -> TS1_CITLF_R {
        TS1_CITLF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TS1_CITHF"]
    #[inline(always)]
    pub fn ts1_cithf(&self) -> TS1_CITHF_R {
        TS1_CITHF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TS1_CAITEF"]
    #[inline(always)]
    pub fn ts1_caitef(&self) -> TS1_CAITEF_R {
        TS1_CAITEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TS1_CAITLF"]
    #[inline(always)]
    pub fn ts1_caitlf(&self) -> TS1_CAITLF_R {
        TS1_CAITLF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TS1_CAITHF"]
    #[inline(always)]
    pub fn ts1_caithf(&self) -> TS1_CAITHF_R {
        TS1_CAITHF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TS1_CITEF"]
    #[inline(always)]
    pub fn ts1_citef(&mut self) -> TS1_CITEF_W {
        TS1_CITEF_W { w: self }
    }
    #[doc = "Bit 1 - TS1_CITLF"]
    #[inline(always)]
    pub fn ts1_citlf(&mut self) -> TS1_CITLF_W {
        TS1_CITLF_W { w: self }
    }
    #[doc = "Bit 2 - TS1_CITHF"]
    #[inline(always)]
    pub fn ts1_cithf(&mut self) -> TS1_CITHF_W {
        TS1_CITHF_W { w: self }
    }
    #[doc = "Bit 4 - TS1_CAITEF"]
    #[inline(always)]
    pub fn ts1_caitef(&mut self) -> TS1_CAITEF_W {
        TS1_CAITEF_W { w: self }
    }
    #[doc = "Bit 5 - TS1_CAITLF"]
    #[inline(always)]
    pub fn ts1_caitlf(&mut self) -> TS1_CAITLF_W {
        TS1_CAITLF_W { w: self }
    }
    #[doc = "Bit 6 - TS1_CAITHF"]
    #[inline(always)]
    pub fn ts1_caithf(&mut self) -> TS1_CAITHF_W {
        TS1_CAITHF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTS_ICIFR is the control register for the interrupt flags.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_icifr](index.html) module"]
pub struct DTS_ICIFR_SPEC;
impl crate::RegisterSpec for DTS_ICIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_icifr::R](R) reader structure"]
impl crate::Readable for DTS_ICIFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dts_icifr::W](W) writer structure"]
impl crate::Writable for DTS_ICIFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTS_ICIFR to value 0"]
impl crate::Resettable for DTS_ICIFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
