#[doc = "Register `FMC_HWCFGR1` reader"]
pub struct R(crate::R<FMC_HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NAND_SEL` reader - NAND_SEL"]
pub struct NAND_SEL_R(crate::FieldReader<bool, bool>);
impl NAND_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAND_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAND_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAND_ECC` reader - NAND_ECC"]
pub struct NAND_ECC_R(crate::FieldReader<bool, bool>);
impl NAND_ECC_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAND_ECC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAND_ECC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDRAM_SEL` reader - SDRAM_SEL"]
pub struct SDRAM_SEL_R(crate::FieldReader<bool, bool>);
impl SDRAM_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDRAM_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDRAM_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID_SIZE` reader - ID_SIZE"]
pub struct ID_SIZE_R(crate::FieldReader<u8, u8>);
impl ID_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ID_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WA_LN2DPTH` reader - WA_LN2DPTH"]
pub struct WA_LN2DPTH_R(crate::FieldReader<u8, u8>);
impl WA_LN2DPTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        WA_LN2DPTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WA_LN2DPTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WD_LN2DPTH` reader - WD_LN2DPTH"]
pub struct WD_LN2DPTH_R(crate::FieldReader<u8, u8>);
impl WD_LN2DPTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        WD_LN2DPTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WD_LN2DPTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_LN2DPTH` reader - WR_LN2DPTH"]
pub struct WR_LN2DPTH_R(crate::FieldReader<u8, u8>);
impl WR_LN2DPTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        WR_LN2DPTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LN2DPTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA_LN2DPTH` reader - RA_LN2DPTH"]
pub struct RA_LN2DPTH_R(crate::FieldReader<u8, u8>);
impl RA_LN2DPTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        RA_LN2DPTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_LN2DPTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - NAND_SEL"]
    #[inline(always)]
    pub fn nand_sel(&self) -> NAND_SEL_R {
        NAND_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAND_ECC"]
    #[inline(always)]
    pub fn nand_ecc(&self) -> NAND_ECC_R {
        NAND_ECC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SDRAM_SEL"]
    #[inline(always)]
    pub fn sdram_sel(&self) -> SDRAM_SEL_R {
        SDRAM_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - ID_SIZE"]
    #[inline(always)]
    pub fn id_size(&self) -> ID_SIZE_R {
        ID_SIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - WA_LN2DPTH"]
    #[inline(always)]
    pub fn wa_ln2dpth(&self) -> WA_LN2DPTH_R {
        WA_LN2DPTH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - WD_LN2DPTH"]
    #[inline(always)]
    pub fn wd_ln2dpth(&self) -> WD_LN2DPTH_R {
        WD_LN2DPTH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - WR_LN2DPTH"]
    #[inline(always)]
    pub fn wr_ln2dpth(&self) -> WR_LN2DPTH_R {
        WR_LN2DPTH_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - RA_LN2DPTH"]
    #[inline(always)]
    pub fn ra_ln2dpth(&self) -> RA_LN2DPTH_R {
        RA_LN2DPTH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "FMC Hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_hwcfgr1](index.html) module"]
pub struct FMC_HWCFGR1_SPEC;
impl crate::RegisterSpec for FMC_HWCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_hwcfgr1::R](R) reader structure"]
impl crate::Readable for FMC_HWCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_HWCFGR1 to value 0x2232_b011"]
impl crate::Resettable for FMC_HWCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2232_b011
    }
}
