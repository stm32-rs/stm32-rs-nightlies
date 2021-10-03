#[doc = "Register `HWCFGR` reader"]
pub struct R(crate::R<HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AXI` reader - AXI interface"]
pub struct AXI_R(crate::FieldReader<u8, u8>);
impl AXI_R {
    pub(crate) fn new(bits: u8) -> Self {
        AXI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO` reader - FIFO depth"]
pub struct FIFO_R(crate::FieldReader<u8, u8>);
impl FIFO_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRES` reader - Prescaler"]
pub struct PRES_R(crate::FieldReader<u8, u8>);
impl PRES_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDL` reader - ID Length"]
pub struct IDL_R(crate::FieldReader<u8, u8>);
impl IDL_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMW` reader - Memory map write"]
pub struct MMW_R(crate::FieldReader<u8, u8>);
impl MMW_R {
    pub(crate) fn new(bits: u8) -> Self {
        MMW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST` reader - Master"]
pub struct MST_R(crate::FieldReader<u8, u8>);
impl MST_R {
    pub(crate) fn new(bits: u8) -> Self {
        MST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - AXI interface"]
    #[inline(always)]
    pub fn axi(&self) -> AXI_R {
        AXI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - FIFO depth"]
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:19 - Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - ID Length"]
    #[inline(always)]
    pub fn idl(&self) -> IDL_R {
        IDL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Memory map write"]
    #[inline(always)]
    pub fn mmw(&self) -> MMW_R {
        MMW_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Master"]
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "HW configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr](index.html) module"]
pub struct HWCFGR_SPEC;
impl crate::RegisterSpec for HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr::R](R) reader structure"]
impl crate::Readable for HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR to value 0x1130_0080"]
impl crate::Resettable for HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1130_0080
    }
}
