///Register `P1STM2AR` reader
pub type R = crate::R<P1STM2ARrs>;
///Field `M2A` reader - Memory2 address
pub type M2A_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Memory2 address
    #[inline(always)]
    pub fn m2a(&self) -> M2A_R {
        M2A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1STM2AR")
            .field("m2a", &self.m2a())
            .finish()
    }
}
/**DCMIPP Pipex status Memory2 address register

You can [`read`](crate::Reg::read) this register and get [`p1stm2ar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1STM2AR)*/
pub struct P1STM2ARrs;
impl crate::RegisterSpec for P1STM2ARrs {
    type Ux = u32;
}
///`read()` method returns [`p1stm2ar::R`](R) reader structure
impl crate::Readable for P1STM2ARrs {}
///`reset()` method sets P1STM2AR to value 0
impl crate::Resettable for P1STM2ARrs {}
