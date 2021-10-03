#[doc = "Register `MACTxTSSNR` reader"]
pub struct R(crate::R<MACTXTSSNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTXTSSNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTXTSSNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTXTSSNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXTSSLO` reader - Transmit Timestamp Status Low"]
pub struct TXTSSLO_R(crate::FieldReader<u32, u32>);
impl TXTSSLO_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXTSSLO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTSSLO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTSSMIS` reader - Transmit Timestamp Status Missed"]
pub struct TXTSSMIS_R(crate::FieldReader<bool, bool>);
impl TXTSSMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXTSSMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTSSMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:30 - Transmit Timestamp Status Low"]
    #[inline(always)]
    pub fn txtsslo(&self) -> TXTSSLO_R {
        TXTSSLO_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Transmit Timestamp Status Missed"]
    #[inline(always)]
    pub fn txtssmis(&self) -> TXTSSMIS_R {
        TXTSSMIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Tx timestamp status nanoseconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactx_tssnr](index.html) module"]
pub struct MACTXTSSNR_SPEC;
impl crate::RegisterSpec for MACTXTSSNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mactx_tssnr::R](R) reader structure"]
impl crate::Readable for MACTXTSSNR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACTxTSSNR to value 0"]
impl crate::Resettable for MACTXTSSNR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
