#[doc = "Register `JPEG_SR` reader"]
pub struct R(crate::R<JPEG_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JPEG_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JPEG_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JPEG_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IFTF` reader - Input FIFO Threshold Flag"]
pub struct IFTF_R(crate::FieldReader<bool, bool>);
impl IFTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFNFF` reader - Input FIFO Not Full Flag"]
pub struct IFNFF_R(crate::FieldReader<bool, bool>);
impl IFNFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFNFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFNFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFTF` reader - Output FIFO Threshold Flag"]
pub struct OFTF_R(crate::FieldReader<bool, bool>);
impl OFTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFNEF` reader - Output FIFO Not Empty Flag"]
pub struct OFNEF_R(crate::FieldReader<bool, bool>);
impl OFNEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFNEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFNEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOCF` reader - End of Conversion Flag"]
pub struct EOCF_R(crate::FieldReader<bool, bool>);
impl EOCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPDF` reader - Header Parsing Done Flag"]
pub struct HPDF_R(crate::FieldReader<bool, bool>);
impl HPDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COF` reader - Codec Operation Flag"]
pub struct COF_R(crate::FieldReader<bool, bool>);
impl COF_R {
    pub(crate) fn new(bits: bool) -> Self {
        COF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Input FIFO Threshold Flag"]
    #[inline(always)]
    pub fn iftf(&self) -> IFTF_R {
        IFTF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input FIFO Not Full Flag"]
    #[inline(always)]
    pub fn ifnff(&self) -> IFNFF_R {
        IFNFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output FIFO Threshold Flag"]
    #[inline(always)]
    pub fn oftf(&self) -> OFTF_R {
        OFTF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Flag"]
    #[inline(always)]
    pub fn ofnef(&self) -> OFNEF_R {
        OFNEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Flag"]
    #[inline(always)]
    pub fn eocf(&self) -> EOCF_R {
        EOCF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Header Parsing Done Flag"]
    #[inline(always)]
    pub fn hpdf(&self) -> HPDF_R {
        HPDF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Codec Operation Flag"]
    #[inline(always)]
    pub fn cof(&self) -> COF_R {
        COF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "JPEG status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_sr](index.html) module"]
pub struct JPEG_SR_SPEC;
impl crate::RegisterSpec for JPEG_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jpeg_sr::R](R) reader structure"]
impl crate::Readable for JPEG_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JPEG_SR to value 0"]
impl crate::Resettable for JPEG_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
