///Register `P1CPPM2AR2` reader
pub type R = crate::R<P1CPPM2AR2rs>;
///Field `M2A` reader - Memory 2 address
pub type M2A_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Memory 2 address
    #[inline(always)]
    pub fn m2a(&self) -> M2A_R {
        M2A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CPPM2AR2")
            .field("m2a", &self.m2a())
            .finish()
    }
}
/**DCMIPP Pipex current pixel packer Memory2 address register 1

You can [`read`](crate::Reg::read) this register and get [`p1cppm2ar2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1CPPM2AR2)*/
pub struct P1CPPM2AR2rs;
impl crate::RegisterSpec for P1CPPM2AR2rs {
    type Ux = u32;
}
///`read()` method returns [`p1cppm2ar2::R`](R) reader structure
impl crate::Readable for P1CPPM2AR2rs {}
///`reset()` method sets P1CPPM2AR2 to value 0
impl crate::Resettable for P1CPPM2AR2rs {}
