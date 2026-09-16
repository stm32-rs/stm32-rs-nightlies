///Register `VAL` reader
pub type R = crate::R<VALrs>;
///Field `RND_VAL` reader - RND_VAL is a 32-bit Random Value. This is the output of the internal four-word FIFO. Note that application must ensure that a random is available in FIFO by ready VAL_READY flag before starting a read otherwise a null value will be returned.
pub type RND_VAL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - RND_VAL is a 32-bit Random Value. This is the output of the internal four-word FIFO. Note that application must ensure that a random is available in FIFO by ready VAL_READY flag before starting a read otherwise a null value will be returned.
    #[inline(always)]
    pub fn rnd_val(&self) -> RND_VAL_R {
        RND_VAL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAL")
            .field("rnd_val", &self.rnd_val())
            .finish()
    }
}
/**TRNG_VAL register

You can [`read`](crate::Reg::read) this register and get [`val::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:VAL)*/
pub struct VALrs;
impl crate::RegisterSpec for VALrs {
    type Ux = u32;
}
///`read()` method returns [`val::R`](R) reader structure
impl crate::Readable for VALrs {}
///`reset()` method sets VAL to value 0
impl crate::Resettable for VALrs {}
