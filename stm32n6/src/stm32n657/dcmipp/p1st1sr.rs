///Register `P1ST1SR` reader
pub type R = crate::R<P1ST1SRrs>;
///Field `ACCU` reader - Accumulation result, divided by 256.
pub type ACCU_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - Accumulation result, divided by 256.
    #[inline(always)]
    pub fn accu(&self) -> ACCU_R {
        ACCU_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1ST1SR")
            .field("accu", &self.accu())
            .finish()
    }
}
/**DCMIPP Pipe1 statistics 1 status register

You can [`read`](crate::Reg::read) this register and get [`p1st1sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1ST1SR)*/
pub struct P1ST1SRrs;
impl crate::RegisterSpec for P1ST1SRrs {
    type Ux = u32;
}
///`read()` method returns [`p1st1sr::R`](R) reader structure
impl crate::Readable for P1ST1SRrs {}
///`reset()` method sets P1ST1SR to value 0
impl crate::Resettable for P1ST1SRrs {}
