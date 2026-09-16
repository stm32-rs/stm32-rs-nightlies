///Register `RDR` reader
pub type R = crate::R<RDRrs>;
///Field `RD` reader - received data
pub type RD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - received data
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDR").field("rd", &self.rd()).finish()
    }
}
/**SWPMI Receive data register

You can [`read`](crate::Reg::read) this register and get [`rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#SWPMI:RDR)*/
pub struct RDRrs;
impl crate::RegisterSpec for RDRrs {
    type Ux = u32;
}
///`read()` method returns [`rdr::R`](R) reader structure
impl crate::Readable for RDRrs {}
///`reset()` method sets RDR to value 0
impl crate::Resettable for RDRrs {}
