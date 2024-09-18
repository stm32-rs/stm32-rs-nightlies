///Register `RNG_IPIDR` reader
pub type R = crate::R<RNG_IPIDRrs>;
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG_IPIDR").field("id", &self.id()).finish()
    }
}
/**RNG identification register

You can [`read`](crate::Reg::read) this register and get [`rng_ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RNG1:RNG_IPIDR)*/
pub struct RNG_IPIDRrs;
impl crate::RegisterSpec for RNG_IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`rng_ipidr::R`](R) reader structure
impl crate::Readable for RNG_IPIDRrs {}
///`reset()` method sets RNG_IPIDR to value 0x0017_0041
impl crate::Resettable for RNG_IPIDRrs {
    const RESET_VALUE: u32 = 0x0017_0041;
}
