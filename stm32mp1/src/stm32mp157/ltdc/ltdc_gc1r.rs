#[doc = "Register `LTDC_GC1R` reader"]
pub struct R(crate::R<LTDC_GC1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_GC1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_GC1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_GC1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WBCH` reader - WBCH"]
pub struct WBCH_R(crate::FieldReader<u8, u8>);
impl WBCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        WBCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WBCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WGCH` reader - WGCH"]
pub struct WGCH_R(crate::FieldReader<u8, u8>);
impl WGCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        WGCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WGCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRCH` reader - WRCH"]
pub struct WRCH_R(crate::FieldReader<u8, u8>);
impl WRCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRBEN` reader - PRBEN"]
pub struct PRBEN_R(crate::FieldReader<bool, bool>);
impl PRBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT` reader - DT"]
pub struct DT_R(crate::FieldReader<u8, u8>);
impl DT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCT` reader - GCT"]
pub struct GCT_R(crate::FieldReader<u8, u8>);
impl GCT_R {
    pub(crate) fn new(bits: u8) -> Self {
        GCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHREN` reader - SHREN"]
pub struct SHREN_R(crate::FieldReader<bool, bool>);
impl SHREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCP` reader - BCP"]
pub struct BCP_R(crate::FieldReader<bool, bool>);
impl BCP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BBEN` reader - BBEN"]
pub struct BBEN_R(crate::FieldReader<bool, bool>);
impl BBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNIP` reader - LNIP"]
pub struct LNIP_R(crate::FieldReader<bool, bool>);
impl LNIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LNIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP` reader - TP"]
pub struct TP_R(crate::FieldReader<bool, bool>);
impl TP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPP` reader - IPP"]
pub struct IPP_R(crate::FieldReader<bool, bool>);
impl IPP_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPP` reader - SPP"]
pub struct SPP_R(crate::FieldReader<bool, bool>);
impl SPP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DWP` reader - DWP"]
pub struct DWP_R(crate::FieldReader<bool, bool>);
impl DWP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DWP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DWP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STREN` reader - STREN"]
pub struct STREN_R(crate::FieldReader<bool, bool>);
impl STREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMEN` reader - BMEN"]
pub struct BMEN_R(crate::FieldReader<bool, bool>);
impl BMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - WBCH"]
    #[inline(always)]
    pub fn wbch(&self) -> WBCH_R {
        WBCH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - WGCH"]
    #[inline(always)]
    pub fn wgch(&self) -> WGCH_R {
        WGCH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - WRCH"]
    #[inline(always)]
    pub fn wrch(&self) -> WRCH_R {
        WRCH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - PRBEN"]
    #[inline(always)]
    pub fn prben(&self) -> PRBEN_R {
        PRBEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - DT"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 17:19 - GCT"]
    #[inline(always)]
    pub fn gct(&self) -> GCT_R {
        GCT_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 21 - SHREN"]
    #[inline(always)]
    pub fn shren(&self) -> SHREN_R {
        SHREN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - BCP"]
    #[inline(always)]
    pub fn bcp(&self) -> BCP_R {
        BCP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - BBEN"]
    #[inline(always)]
    pub fn bben(&self) -> BBEN_R {
        BBEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LNIP"]
    #[inline(always)]
    pub fn lnip(&self) -> LNIP_R {
        LNIP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TP"]
    #[inline(always)]
    pub fn tp(&self) -> TP_R {
        TP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - IPP"]
    #[inline(always)]
    pub fn ipp(&self) -> IPP_R {
        IPP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SPP"]
    #[inline(always)]
    pub fn spp(&self) -> SPP_R {
        SPP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DWP"]
    #[inline(always)]
    pub fn dwp(&self) -> DWP_R {
        DWP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - STREN"]
    #[inline(always)]
    pub fn stren(&self) -> STREN_R {
        STREN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - BMEN"]
    #[inline(always)]
    pub fn bmen(&self) -> BMEN_R {
        BMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "LTDC global configuration 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_gc1r](index.html) module"]
pub struct LTDC_GC1R_SPEC;
impl crate::RegisterSpec for LTDC_GC1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_gc1r::R](R) reader structure"]
impl crate::Readable for LTDC_GC1R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTDC_GC1R to value 0x6be2_d888"]
impl crate::Resettable for LTDC_GC1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6be2_d888
    }
}
