#[doc = "Register `FMC_HWCFGR2` reader"]
pub struct R(crate::R<FMC_HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_LN2DPTH` reader - RD_LN2DPTH"]
pub struct RD_LN2DPTH_R(crate::FieldReader<u8, u8>);
impl RD_LN2DPTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        RD_LN2DPTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_LN2DPTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOR_BASE` reader - NOR_BASE"]
pub struct NOR_BASE_R(crate::FieldReader<u8, u8>);
impl NOR_BASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        NOR_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOR_BASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDRAM_RBASE` reader - SDRAM_RBASE"]
pub struct SDRAM_RBASE_R(crate::FieldReader<u8, u8>);
impl SDRAM_RBASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDRAM_RBASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDRAM_RBASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAND_BASE` reader - NAND_BASE"]
pub struct NAND_BASE_R(crate::FieldReader<u8, u8>);
impl NAND_BASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        NAND_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAND_BASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDRAM1_BASE` reader - SDRAM1_BASE"]
pub struct SDRAM1_BASE_R(crate::FieldReader<u8, u8>);
impl SDRAM1_BASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDRAM1_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDRAM1_BASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDRAM2_BASE` reader - SDRAM2_BASE"]
pub struct SDRAM2_BASE_R(crate::FieldReader<u8, u8>);
impl SDRAM2_BASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDRAM2_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDRAM2_BASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - RD_LN2DPTH"]
    #[inline(always)]
    pub fn rd_ln2dpth(&self) -> RD_LN2DPTH_R {
        RD_LN2DPTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - NOR_BASE"]
    #[inline(always)]
    pub fn nor_base(&self) -> NOR_BASE_R {
        NOR_BASE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SDRAM_RBASE"]
    #[inline(always)]
    pub fn sdram_rbase(&self) -> SDRAM_RBASE_R {
        SDRAM_RBASE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - NAND_BASE"]
    #[inline(always)]
    pub fn nand_base(&self) -> NAND_BASE_R {
        NAND_BASE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SDRAM1_BASE"]
    #[inline(always)]
    pub fn sdram1_base(&self) -> SDRAM1_BASE_R {
        SDRAM1_BASE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SDRAM2_BASE"]
    #[inline(always)]
    pub fn sdram2_base(&self) -> SDRAM2_BASE_R {
        SDRAM2_BASE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "FMC Hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_hwcfgr2](index.html) module"]
pub struct FMC_HWCFGR2_SPEC;
impl crate::RegisterSpec for FMC_HWCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_hwcfgr2::R](R) reader structure"]
impl crate::Readable for FMC_HWCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_HWCFGR2 to value 0x00dc_8762"]
impl crate::Resettable for FMC_HWCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00dc_8762
    }
}
