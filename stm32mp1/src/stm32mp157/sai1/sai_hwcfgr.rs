#[doc = "Register `SAI_HWCFGR` reader"]
pub struct R(crate::R<SAI_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFO_SIZE` reader - FIFO_SIZE"]
pub struct FIFO_SIZE_R(crate::FieldReader<u8, u8>);
impl FIFO_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFO_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDIF_PDM` reader - SPDIF_PDM"]
pub struct SPDIF_PDM_R(crate::FieldReader<u8, u8>);
impl SPDIF_PDM_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPDIF_PDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPDIF_PDM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTION_REGOUT` reader - OPTION_REGOUT"]
pub struct OPTION_REGOUT_R(crate::FieldReader<u8, u8>);
impl OPTION_REGOUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPTION_REGOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTION_REGOUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - FIFO_SIZE"]
    #[inline(always)]
    pub fn fifo_size(&self) -> FIFO_SIZE_R {
        FIFO_SIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - SPDIF_PDM"]
    #[inline(always)]
    pub fn spdif_pdm(&self) -> SPDIF_PDM_R {
        SPDIF_PDM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:19 - OPTION_REGOUT"]
    #[inline(always)]
    pub fn option_regout(&self) -> OPTION_REGOUT_R {
        OPTION_REGOUT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
#[doc = "SAI hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_hwcfgr](index.html) module"]
pub struct SAI_HWCFGR_SPEC;
impl crate::RegisterSpec for SAI_HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_hwcfgr::R](R) reader structure"]
impl crate::Readable for SAI_HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAI_HWCFGR to value 0x0108"]
impl crate::Resettable for SAI_HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0108
    }
}
