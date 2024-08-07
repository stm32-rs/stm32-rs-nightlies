///Register `RNG_DR` reader
pub type R = crate::R<RNG_DRrs>;
///Field `RNDATA` reader - RNDATA
pub type RNDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - RNDATA
    #[inline(always)]
    pub fn rndata(&self) -> RNDATA_R {
        RNDATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG_DR")
            .field("rndata", &self.rndata())
            .finish()
    }
}
/**The RNG_DR register is a read-only register.

You can [`read`](crate::Reg::read) this register and get [`rng_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RNG1:RNG_DR)*/
pub struct RNG_DRrs;
impl crate::RegisterSpec for RNG_DRrs {
    type Ux = u32;
}
///`read()` method returns [`rng_dr::R`](R) reader structure
impl crate::Readable for RNG_DRrs {}
///`reset()` method sets RNG_DR to value 0
impl crate::Resettable for RNG_DRrs {
    const RESET_VALUE: u32 = 0;
}
