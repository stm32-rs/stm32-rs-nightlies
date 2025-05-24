///Register `P2CPPM0AR2` reader
pub type R = crate::R<P2CPPM0AR2rs>;
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
        f.debug_struct("P2CPPM0AR2")
            .field("m0a", &self.m0a())
            .finish()
    }
}
/**DCMIPP Pipe2 current pixel packer Memory0 address register 2

You can [`read`](crate::Reg::read) this register and get [`p2cppm0ar2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CPPM0AR2)*/
pub struct P2CPPM0AR2rs;
impl crate::RegisterSpec for P2CPPM0AR2rs {
    type Ux = u32;
}
///`read()` method returns [`p2cppm0ar2::R`](R) reader structure
impl crate::Readable for P2CPPM0AR2rs {}
///`reset()` method sets P2CPPM0AR2 to value 0
impl crate::Resettable for P2CPPM0AR2rs {}
