///Register `P1CPPM1AR2` reader
pub type R = crate::R<P1CPPM1AR2rs>;
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
        f.debug_struct("P1CPPM1AR2")
            .field("m1a", &self.m1a())
            .finish()
    }
}
/**DCMIPP Pipex current pixel packer Memory1 address register 2

You can [`read`](crate::Reg::read) this register and get [`p1cppm1ar2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1CPPM1AR2)*/
pub struct P1CPPM1AR2rs;
impl crate::RegisterSpec for P1CPPM1AR2rs {
    type Ux = u32;
}
///`read()` method returns [`p1cppm1ar2::R`](R) reader structure
impl crate::Readable for P1CPPM1AR2rs {}
///`reset()` method sets P1CPPM1AR2 to value 0
impl crate::Resettable for P1CPPM1AR2rs {}
