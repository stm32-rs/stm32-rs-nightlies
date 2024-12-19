///Register `RDR` reader
pub type R = crate::R<RDRrs>;
///Field `RDR` reader - Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see Figure 243). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit.
pub type RDR_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:8 - Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see Figure 243). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit.
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDR").field("rdr", &self.rdr()).finish()
    }
}
/**USART receive data register

You can [`read`](crate::Reg::read) this register and get [`rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#USART1:RDR)*/
pub struct RDRrs;
impl crate::RegisterSpec for RDRrs {
    type Ux = u32;
}
///`read()` method returns [`rdr::R`](R) reader structure
impl crate::Readable for RDRrs {}
///`reset()` method sets RDR to value 0
impl crate::Resettable for RDRrs {
    const RESET_VALUE: u32 = 0;
}
