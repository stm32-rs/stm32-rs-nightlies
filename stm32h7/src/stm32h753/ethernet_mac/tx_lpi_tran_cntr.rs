#[doc = "Register `TX_LPI_TRAN_CNTR` reader"]
pub struct R(crate::R<TX_LPI_TRAN_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_LPI_TRAN_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_LPI_TRAN_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_LPI_TRAN_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXLPITRC` reader - Tx LPI Transition counter"]
pub struct TXLPITRC_R(crate::FieldReader<u32, u32>);
impl TXLPITRC_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXLPITRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXLPITRC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Tx LPI Transition counter"]
    #[inline(always)]
    pub fn txlpitrc(&self) -> TXLPITRC_R {
        TXLPITRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Tx LPI transition counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_lpi_tran_cntr](index.html) module"]
pub struct TX_LPI_TRAN_CNTR_SPEC;
impl crate::RegisterSpec for TX_LPI_TRAN_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_lpi_tran_cntr::R](R) reader structure"]
impl crate::Readable for TX_LPI_TRAN_CNTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_LPI_TRAN_CNTR to value 0"]
impl crate::Resettable for TX_LPI_TRAN_CNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
