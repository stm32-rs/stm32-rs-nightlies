#[doc = "Register `FDCAN_TXESC` reader"]
pub type R = crate::R<FDCAN_TXESCrs>;
#[doc = "Field `TBDS` reader - TBDS"]
pub type TBDS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - TBDS"]
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Configures the number of data bytes belonging to a Tx buffer element. Data field sizes &amp;gt;8 bytes are intended for CAN FD operation only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txesc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXESCrs;
impl crate::RegisterSpec for FDCAN_TXESCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txesc::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXESCrs {}
#[doc = "`reset()` method sets FDCAN_TXESC to value 0"]
impl crate::Resettable for FDCAN_TXESCrs {
    const RESET_VALUE: u32 = 0;
}
