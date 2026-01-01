///Register `HASH_HR2` reader
pub type R = crate::R<HASH_HR2rs>;
///Field `H2` reader - Hash data x
pub type H2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hash data x
    #[inline(always)]
    pub fn h2(&self) -> H2_R {
        H2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_HR2").field("h2", &self.h2()).finish()
    }
}
/**HASH digest register 2

You can [`read`](crate::Reg::read) this register and get [`hash_hr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#HASH:HASH_HR2)*/
pub struct HASH_HR2rs;
impl crate::RegisterSpec for HASH_HR2rs {
    type Ux = u32;
}
///`read()` method returns [`hash_hr2::R`](R) reader structure
impl crate::Readable for HASH_HR2rs {}
///`reset()` method sets HASH_HR2 to value 0
impl crate::Resettable for HASH_HR2rs {}
