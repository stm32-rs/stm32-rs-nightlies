///Register `VAL` reader
pub type R = crate::R<VALrs>;
///Field `RND_VAL` reader - Random value
pub type RND_VAL_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Random value
    #[inline(always)]
    pub fn rnd_val(&self) -> RND_VAL_R {
        RND_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAL")
            .field("rnd_val", &self.rnd_val())
            .finish()
    }
}
/**RNG_VAL register

You can [`read`](crate::Reg::read) this register and get [`val::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RNG:VAL)*/
pub struct VALrs;
impl crate::RegisterSpec for VALrs {
    type Ux = u32;
}
///`read()` method returns [`val::R`](R) reader structure
impl crate::Readable for VALrs {}
///`reset()` method sets VAL to value 0
impl crate::Resettable for VALrs {}
