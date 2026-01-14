///Register `HASH_HR1` reader
pub type R = crate::R<HASH_HR1rs>;
///Field `H1` reader - H1
pub type H1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - H1
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_HR1").field("h1", &self.h1()).finish()
    }
}
/**read-only

You can [`read`](crate::Reg::read) this register and get [`hash_hr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#HASH:HASH_HR1)*/
pub struct HASH_HR1rs;
impl crate::RegisterSpec for HASH_HR1rs {
    type Ux = u32;
}
///`read()` method returns [`hash_hr1::R`](R) reader structure
impl crate::Readable for HASH_HR1rs {}
///`reset()` method sets HASH_HR1 to value 0
impl crate::Resettable for HASH_HR1rs {}
