#[doc = "Register `FDCAN_TXFQS` reader"]
pub type R = crate::R<FDCAN_TXFQSrs>;
#[doc = "Field `TFFL` reader - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\\[TFQM\\]
= 1)."]
pub type TFFL_R = crate::FieldReader;
#[doc = "Field `TFGI` reader - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)"]
pub type TFGI_R = crate::FieldReader;
#[doc = "Field `TFQPI` reader - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3"]
pub type TFQPI_R = crate::FieldReader;
#[doc = "Field `TFQF` reader - Tx FIFO/queue full"]
pub type TFQF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\\[TFQM\\]
= 1)."]
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)"]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/queue full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "FDCAN Tx FIFO/queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txfqs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXFQSrs;
impl crate::RegisterSpec for FDCAN_TXFQSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txfqs::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXFQSrs {}
#[doc = "`reset()` method sets FDCAN_TXFQS to value 0x03"]
impl crate::Resettable for FDCAN_TXFQSrs {
    const RESET_VALUE: u32 = 0x03;
}
