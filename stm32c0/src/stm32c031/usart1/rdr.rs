#[doc = "Register `RDR` reader"]
pub type R = crate::R<RDRrs>;
#[doc = "Field `RDR` reader - Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see ). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit."]
pub type RDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see ). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit."]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "USART receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDRrs;
impl crate::RegisterSpec for RDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RDRrs {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RDRrs {
    const RESET_VALUE: u32 = 0;
}
