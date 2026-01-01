///Register `VAL` reader
pub type R = crate::R<VALrs>;
///Field `RANDOM_VALUE` reader - Random Value
pub type RANDOM_VALUE_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Random Value
    #[inline(always)]
    pub fn random_value(&self) -> RANDOM_VALUE_R {
        RANDOM_VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAL")
            .field("random_value", &self.random_value())
            .finish()
    }
}
/**RNG_VAL register

You can [`read`](crate::Reg::read) this register and get [`val::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:VAL)*/
pub struct VALrs;
impl crate::RegisterSpec for VALrs {
    type Ux = u32;
}
///`read()` method returns [`val::R`](R) reader structure
impl crate::Readable for VALrs {}
///`reset()` method sets VAL to value 0
impl crate::Resettable for VALrs {}
