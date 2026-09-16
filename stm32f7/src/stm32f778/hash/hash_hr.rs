///Register `HASH_HR%s` reader
pub type R = crate::R<HASH_HRrs>;
///Field `H` reader - H0
pub type H_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - H0
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_HR").field("h", &self.h()).finish()
    }
}
/**HASH digest register %s

You can [`read`](crate::Reg::read) this register and get [`hash_hr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#HASH:HASH_HR[0])*/
pub struct HASH_HRrs;
impl crate::RegisterSpec for HASH_HRrs {
    type Ux = u32;
}
///`read()` method returns [`hash_hr::R`](R) reader structure
impl crate::Readable for HASH_HRrs {}
///`reset()` method sets HASH_HR%s to value 0
impl crate::Resettable for HASH_HRrs {}
