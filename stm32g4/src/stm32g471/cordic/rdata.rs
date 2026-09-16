///Register `RDATA` reader
pub type R = crate::R<RDATArs>;
///Field `RES` reader - RES
pub type RES_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - RES
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDATA").field("res", &self.res()).finish()
    }
}
/**CORDIC result register

You can [`read`](crate::Reg::read) this register and get [`rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G471.html#CORDIC:RDATA)*/
pub struct RDATArs;
impl crate::RegisterSpec for RDATArs {
    type Ux = u32;
}
///`read()` method returns [`rdata::R`](R) reader structure
impl crate::Readable for RDATArs {}
///`reset()` method sets RDATA to value 0
impl crate::Resettable for RDATArs {}
