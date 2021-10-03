#[doc = "Register `HSEM_HWCFGR1` reader"]
pub struct R(crate::R<HSEM_HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEM_HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEM_HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEM_HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NBSEM` reader - NBSEM"]
pub struct NBSEM_R(crate::FieldReader<u8, u8>);
impl NBSEM_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBSEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBSEM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBINT` reader - NBINT"]
pub struct NBINT_R(crate::FieldReader<u8, u8>);
impl NBINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - NBSEM"]
    #[inline(always)]
    pub fn nbsem(&self) -> NBSEM_R {
        NBSEM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - NBINT"]
    #[inline(always)]
    pub fn nbint(&self) -> NBINT_R {
        NBINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "HSEM hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_hwcfgr1](index.html) module"]
pub struct HSEM_HWCFGR1_SPEC;
impl crate::RegisterSpec for HSEM_HWCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsem_hwcfgr1::R](R) reader structure"]
impl crate::Readable for HSEM_HWCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSEM_HWCFGR1 to value 0x0220"]
impl crate::Resettable for HSEM_HWCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0220
    }
}
