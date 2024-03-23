#[doc = "Register `RX_ALIGNMENT_ERROR_PACKETS` reader"]
pub type R = crate::R<RX_ALIGNMENT_ERROR_PACKETSrs>;
#[doc = "Field `RXALGNERR` reader - Rx Alignment Error Packets This field indicates the number of packets received with alignment (dribble) error. It is valid only in 10/100 mode."]
pub type RXALGNERR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Rx Alignment Error Packets This field indicates the number of packets received with alignment (dribble) error. It is valid only in 10/100 mode."]
    #[inline(always)]
    pub fn rxalgnerr(&self) -> RXALGNERR_R {
        RXALGNERR_R::new(self.bits)
    }
}
#[doc = "Rx alignment error packets register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_alignment_error_packets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_ALIGNMENT_ERROR_PACKETSrs;
impl crate::RegisterSpec for RX_ALIGNMENT_ERROR_PACKETSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_alignment_error_packets::R`](R) reader structure"]
impl crate::Readable for RX_ALIGNMENT_ERROR_PACKETSrs {}
#[doc = "`reset()` method sets RX_ALIGNMENT_ERROR_PACKETS to value 0"]
impl crate::Resettable for RX_ALIGNMENT_ERROR_PACKETSrs {
    const RESET_VALUE: u32 = 0;
}
