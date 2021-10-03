#[doc = "Register `ETZPC_HWCFGR` reader"]
pub struct R(crate::R<ETZPC_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETZPC_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETZPC_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETZPC_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM_TZMA` reader - NUM_TZMA"]
pub struct NUM_TZMA_R(crate::FieldReader<u8, u8>);
impl NUM_TZMA_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_TZMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_TZMA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_PER_SEC` reader - NUM_PER_SEC"]
pub struct NUM_PER_SEC_R(crate::FieldReader<u8, u8>);
impl NUM_PER_SEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_PER_SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_PER_SEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_AHB_SEC` reader - NUM_AHB_SEC"]
pub struct NUM_AHB_SEC_R(crate::FieldReader<u8, u8>);
impl NUM_AHB_SEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_AHB_SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_AHB_SEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHUNKS1N4` reader - CHUNKS1N4"]
pub struct CHUNKS1N4_R(crate::FieldReader<u8, u8>);
impl CHUNKS1N4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHUNKS1N4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHUNKS1N4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - NUM_TZMA"]
    #[inline(always)]
    pub fn num_tzma(&self) -> NUM_TZMA_R {
        NUM_TZMA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NUM_PER_SEC"]
    #[inline(always)]
    pub fn num_per_sec(&self) -> NUM_PER_SEC_R {
        NUM_PER_SEC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NUM_AHB_SEC"]
    #[inline(always)]
    pub fn num_ahb_sec(&self) -> NUM_AHB_SEC_R {
        NUM_AHB_SEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CHUNKS1N4"]
    #[inline(always)]
    pub fn chunks1n4(&self) -> CHUNKS1N4_R {
        CHUNKS1N4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "ETZPC IP HW configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_hwcfgr](index.html) module"]
pub struct ETZPC_HWCFGR_SPEC;
impl crate::RegisterSpec for ETZPC_HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etzpc_hwcfgr::R](R) reader structure"]
impl crate::Readable for ETZPC_HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETZPC_HWCFGR to value 0x6002"]
impl crate::Resettable for ETZPC_HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6002
    }
}
