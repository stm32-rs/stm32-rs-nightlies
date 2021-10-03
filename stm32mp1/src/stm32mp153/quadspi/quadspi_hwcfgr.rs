#[doc = "Register `QUADSPI_HWCFGR` reader"]
pub struct R(crate::R<QUADSPI_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOSIZE` reader - FIFOSIZE"]
pub struct FIFOSIZE_R(crate::FieldReader<u8, u8>);
impl FIFOSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOPTR` reader - FIFOPTR"]
pub struct FIFOPTR_R(crate::FieldReader<u8, u8>);
impl FIFOPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESCVAL` reader - PRESCVAL"]
pub struct PRESCVAL_R(crate::FieldReader<u8, u8>);
impl PRESCVAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRESCVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLENGTH` reader - IDLENGTH"]
pub struct IDLENGTH_R(crate::FieldReader<u8, u8>);
impl IDLENGTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDLENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLENGTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - FIFOSIZE"]
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FIFOPTR"]
    #[inline(always)]
    pub fn fifoptr(&self) -> FIFOPTR_R {
        FIFOPTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PRESCVAL"]
    #[inline(always)]
    pub fn prescval(&self) -> PRESCVAL_R {
        PRESCVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - IDLENGTH"]
    #[inline(always)]
    pub fn idlength(&self) -> IDLENGTH_R {
        IDLENGTH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "QUADSPI HW configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_hwcfgr](index.html) module"]
pub struct QUADSPI_HWCFGR_SPEC;
impl crate::RegisterSpec for QUADSPI_HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quadspi_hwcfgr::R](R) reader structure"]
impl crate::Readable for QUADSPI_HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUADSPI_HWCFGR to value 0xb058"]
impl crate::Resettable for QUADSPI_HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb058
    }
}
