#[doc = "Register `DFSDM_HWCFGR` reader"]
pub struct R(crate::R<DFSDM_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NBT` reader - NBT"]
pub struct NBT_R(crate::FieldReader<u8, u8>);
impl NBT_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBF` reader - NBF"]
pub struct NBF_R(crate::FieldReader<u8, u8>);
impl NBF_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - NBT"]
    #[inline(always)]
    pub fn nbt(&self) -> NBT_R {
        NBT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NBF"]
    #[inline(always)]
    pub fn nbf(&self) -> NBF_R {
        NBF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "This register specifies the hardware configuration of DFSDM peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_hwcfgr](index.html) module"]
pub struct DFSDM_HWCFGR_SPEC;
impl crate::RegisterSpec for DFSDM_HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_hwcfgr::R](R) reader structure"]
impl crate::Readable for DFSDM_HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM_HWCFGR to value 0x0608"]
impl crate::Resettable for DFSDM_HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0608
    }
}
