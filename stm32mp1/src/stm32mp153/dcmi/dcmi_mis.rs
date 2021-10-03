#[doc = "Register `DCMI_MIS` reader"]
pub struct R(crate::R<DCMI_MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAME_MIS` reader - FRAME_MIS"]
pub struct FRAME_MIS_R(crate::FieldReader<bool, bool>);
impl FRAME_MIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_MIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_MIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_MIS` reader - OVR_MIS"]
pub struct OVR_MIS_R(crate::FieldReader<bool, bool>);
impl OVR_MIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR_MIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_MIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_MIS` reader - ERR_MIS"]
pub struct ERR_MIS_R(crate::FieldReader<bool, bool>);
impl ERR_MIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_MIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_MIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSYNC_MIS` reader - VSYNC_MIS"]
pub struct VSYNC_MIS_R(crate::FieldReader<bool, bool>);
impl VSYNC_MIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSYNC_MIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSYNC_MIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_MIS` reader - LINE_MIS"]
pub struct LINE_MIS_R(crate::FieldReader<bool, bool>);
impl LINE_MIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINE_MIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_MIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - FRAME_MIS"]
    #[inline(always)]
    pub fn frame_mis(&self) -> FRAME_MIS_R {
        FRAME_MIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OVR_MIS"]
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ERR_MIS"]
    #[inline(always)]
    pub fn err_mis(&self) -> ERR_MIS_R {
        ERR_MIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VSYNC_MIS"]
    #[inline(always)]
    pub fn vsync_mis(&self) -> VSYNC_MIS_R {
        VSYNC_MIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LINE_MIS"]
    #[inline(always)]
    pub fn line_mis(&self) -> LINE_MIS_R {
        LINE_MIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
#[doc = "This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_mis](index.html) module"]
pub struct DCMI_MIS_SPEC;
impl crate::RegisterSpec for DCMI_MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_mis::R](R) reader structure"]
impl crate::Readable for DCMI_MIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCMI_MIS to value 0"]
impl crate::Resettable for DCMI_MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
