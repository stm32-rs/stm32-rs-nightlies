#[doc = "Register `FDCAN_TXFQS` reader"]
pub type R = crate::R<FDCAN_TXFQSrs>;
#[doc = "Field `TFFL` reader - TFFL"]
pub type TFFL_R = crate::FieldReader;
#[doc = "Field `TFGI` reader - TFGI"]
pub type TFGI_R = crate::FieldReader;
#[doc = "Field `TFQPI` reader - TFQPI"]
pub type TFQPI_R = crate::FieldReader;
#[doc = "Field `TFQF` reader - TFQF"]
pub type TFQF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - TFFL"]
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - TFGI"]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - TFQPI"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - TFQF"]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "The Tx FIFO/queue status is related to the pending Tx requests listed in register FDCAN_TXBRP. Therefore the effect of add/cancellation requests may be delayed due to a running Tx scan (FDCAN_TXBRP not yet updated).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txfqs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXFQSrs;
impl crate::RegisterSpec for FDCAN_TXFQSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txfqs::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXFQSrs {}
#[doc = "`reset()` method sets FDCAN_TXFQS to value 0"]
impl crate::Resettable for FDCAN_TXFQSrs {
    const RESET_VALUE: u32 = 0;
}
