///Register `HASH_HR3` reader
pub type R = crate::R<HASH_HR3rs>;
///Field `H3` reader - H3
pub type H3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - H3
    #[inline(always)]
    pub fn h3(&self) -> H3_R {
        H3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_HR3").field("h3", &self.h3()).finish()
    }
}
/**read-only

You can [`read`](crate::Reg::read) this register and get [`hash_hr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#HASH:HASH_HR3)*/
pub struct HASH_HR3rs;
impl crate::RegisterSpec for HASH_HR3rs {
    type Ux = u32;
}
///`read()` method returns [`hash_hr3::R`](R) reader structure
impl crate::Readable for HASH_HR3rs {}
///`reset()` method sets HASH_HR3 to value 0
impl crate::Resettable for HASH_HR3rs {}
