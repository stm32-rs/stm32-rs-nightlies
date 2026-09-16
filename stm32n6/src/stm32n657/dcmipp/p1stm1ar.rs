///Register `P1STM1AR` reader
pub type R = crate::R<P1STM1ARrs>;
///Field `M1A` reader - Memory1 address
pub type M1A_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Memory1 address
    #[inline(always)]
    pub fn m1a(&self) -> M1A_R {
        M1A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1STM1AR")
            .field("m1a", &self.m1a())
            .finish()
    }
}
/**DCMIPP Pipex status Memory1 address register

You can [`read`](crate::Reg::read) this register and get [`p1stm1ar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1STM1AR)*/
pub struct P1STM1ARrs;
impl crate::RegisterSpec for P1STM1ARrs {
    type Ux = u32;
}
///`read()` method returns [`p1stm1ar::R`](R) reader structure
impl crate::Readable for P1STM1ARrs {}
///`reset()` method sets P1STM1AR to value 0
impl crate::Resettable for P1STM1ARrs {}
