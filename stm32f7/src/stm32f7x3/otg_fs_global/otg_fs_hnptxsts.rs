#[doc = "Register `OTG_FS_HNPTXSTS` reader"]
pub struct R(crate::R<OTG_FS_HNPTXSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_HNPTXSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_HNPTXSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_HNPTXSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NPTXFSAV` reader - Non-periodic TxFIFO space available"]
pub struct NPTXFSAV_R(crate::FieldReader<u16, u16>);
impl NPTXFSAV_R {
    pub(crate) fn new(bits: u16) -> Self {
        NPTXFSAV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPTXFSAV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPTQXSAV` reader - Non-periodic transmit request queue space available"]
pub struct NPTQXSAV_R(crate::FieldReader<u8, u8>);
impl NPTQXSAV_R {
    pub(crate) fn new(bits: u8) -> Self {
        NPTQXSAV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPTQXSAV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPTXQTOP` reader - Top of the non-periodic transmit request queue"]
pub struct NPTXQTOP_R(crate::FieldReader<u8, u8>);
impl NPTXQTOP_R {
    pub(crate) fn new(bits: u8) -> Self {
        NPTXQTOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPTXQTOP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Non-periodic TxFIFO space available"]
    #[inline(always)]
    pub fn nptxfsav(&self) -> NPTXFSAV_R {
        NPTXFSAV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Non-periodic transmit request queue space available"]
    #[inline(always)]
    pub fn nptqxsav(&self) -> NPTQXSAV_R {
        NPTQXSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the non-periodic transmit request queue"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hnptxsts](index.html) module"]
pub struct OTG_FS_HNPTXSTS_SPEC;
impl crate::RegisterSpec for OTG_FS_HNPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_hnptxsts::R](R) reader structure"]
impl crate::Readable for OTG_FS_HNPTXSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_FS_HNPTXSTS to value 0x0008_0200"]
impl crate::Resettable for OTG_FS_HNPTXSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0200
    }
}
