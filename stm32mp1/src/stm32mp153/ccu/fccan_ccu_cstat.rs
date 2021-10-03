#[doc = "Register `FCCAN_CCU_CSTAT` reader"]
pub struct R(crate::R<FCCAN_CCU_CSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCAN_CCU_CSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCAN_CCU_CSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCAN_CCU_CSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OCPC` reader - OCPC"]
pub struct OCPC_R(crate::FieldReader<u32, u32>);
impl OCPC_R {
    pub(crate) fn new(bits: u32) -> Self {
        OCPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCPC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TQC` reader - TQC"]
pub struct TQC_R(crate::FieldReader<u16, u16>);
impl TQC_R {
    pub(crate) fn new(bits: u16) -> Self {
        TQC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TQC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALS` reader - CALS"]
pub struct CALS_R(crate::FieldReader<u8, u8>);
impl CALS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CALS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - OCPC"]
    #[inline(always)]
    pub fn ocpc(&self) -> OCPC_R {
        OCPC_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:28 - TQC"]
    #[inline(always)]
    pub fn tqc(&self) -> TQC_R {
        TQC_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bits 30:31 - CALS"]
    #[inline(always)]
    pub fn cals(&self) -> CALS_R {
        CALS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
#[doc = "Calibration status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccan_ccu_cstat](index.html) module"]
pub struct FCCAN_CCU_CSTAT_SPEC;
impl crate::RegisterSpec for FCCAN_CCU_CSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fccan_ccu_cstat::R](R) reader structure"]
impl crate::Readable for FCCAN_CCU_CSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FCCAN_CCU_CSTAT to value 0x0203_ffff"]
impl crate::Resettable for FCCAN_CCU_CSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0203_ffff
    }
}
