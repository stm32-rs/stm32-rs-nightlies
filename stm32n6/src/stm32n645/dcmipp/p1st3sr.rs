///Register `P1ST3SR` reader
pub type R = crate::R<P1ST3SRrs>;
///Field `ACCU` reader - accumulation result, divided by 256.
pub type ACCU_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - accumulation result, divided by 256.
    #[inline(always)]
    pub fn accu(&self) -> ACCU_R {
        ACCU_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1ST3SR")
            .field("accu", &self.accu())
            .finish()
    }
}
/**DCMIPP Pipe1 statistics 3 status register

You can [`read`](crate::Reg::read) this register and get [`p1st3sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1ST3SR)*/
pub struct P1ST3SRrs;
impl crate::RegisterSpec for P1ST3SRrs {
    type Ux = u32;
}
///`read()` method returns [`p1st3sr::R`](R) reader structure
impl crate::Readable for P1ST3SRrs {}
///`reset()` method sets P1ST3SR to value 0
impl crate::Resettable for P1ST3SRrs {}
