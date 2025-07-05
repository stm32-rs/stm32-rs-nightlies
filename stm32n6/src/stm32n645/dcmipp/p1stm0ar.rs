///Register `P1STM0AR` reader
pub type R = crate::R<P1STM0ARrs>;
///Field `M0A` reader - Memory0 address
pub type M0A_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Memory0 address
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1STM0AR")
            .field("m0a", &self.m0a())
            .finish()
    }
}
/**DCMIPP Pipex status Memory0 address register

You can [`read`](crate::Reg::read) this register and get [`p1stm0ar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1STM0AR)*/
pub struct P1STM0ARrs;
impl crate::RegisterSpec for P1STM0ARrs {
    type Ux = u32;
}
///`read()` method returns [`p1stm0ar::R`](R) reader structure
impl crate::Readable for P1STM0ARrs {}
///`reset()` method sets P1STM0AR to value 0
impl crate::Resettable for P1STM0ARrs {}
