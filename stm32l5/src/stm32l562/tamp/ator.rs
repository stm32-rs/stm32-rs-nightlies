///Register `ATOR` reader
pub type R = crate::R<ATORrs>;
///Field `PRNG` reader - Pseudo-random generator value
pub type PRNG_R = crate::FieldReader;
///Field `SEEDF` reader - Seed running flag
pub type SEEDF_R = crate::BitReader;
///Field `INITS` reader - Active tamper initialization status
pub type INITS_R = crate::BitReader;
impl R {
    ///Bits 0:7 - Pseudo-random generator value
    #[inline(always)]
    pub fn prng(&self) -> PRNG_R {
        PRNG_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 14 - Seed running flag
    #[inline(always)]
    pub fn seedf(&self) -> SEEDF_R {
        SEEDF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Active tamper initialization status
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATOR")
            .field("prng", &self.prng())
            .field("seedf", &self.seedf())
            .field("inits", &self.inits())
            .finish()
    }
}
/**TAMP active tamper output register

You can [`read`](crate::Reg::read) this register and get [`ator::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#TAMP:ATOR)*/
pub struct ATORrs;
impl crate::RegisterSpec for ATORrs {
    type Ux = u32;
}
///`read()` method returns [`ator::R`](R) reader structure
impl crate::Readable for ATORrs {}
///`reset()` method sets ATOR to value 0
impl crate::Resettable for ATORrs {}
