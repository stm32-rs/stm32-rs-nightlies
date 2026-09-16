///Register `DOR` reader
pub type R = crate::R<DORrs>;
///Field `DATAOUT` reader - Data Output FIFO
pub type DATAOUT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Data Output FIFO
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOR")
            .field("dataout", &self.dataout())
            .finish()
    }
}
/**JPEG data output register

You can [`read`](crate::Reg::read) this register and get [`dor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#JPEG:DOR)*/
pub struct DORrs;
impl crate::RegisterSpec for DORrs {
    type Ux = u32;
}
///`read()` method returns [`dor::R`](R) reader structure
impl crate::Readable for DORrs {}
///`reset()` method sets DOR to value 0
impl crate::Resettable for DORrs {}
