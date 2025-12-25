///Register `HASH_HR7` reader
pub type R = crate::R<HASH_HR7rs>;
///Field `H7` reader - Hash data x
pub type H7_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hash data x
    #[inline(always)]
    pub fn h7(&self) -> H7_R {
        H7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_HR7").field("h7", &self.h7()).finish()
    }
}
/**HASH supplementary digest register 7

You can [`read`](crate::Reg::read) this register and get [`hash_hr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#HASH:HASH_HR7)*/
pub struct HASH_HR7rs;
impl crate::RegisterSpec for HASH_HR7rs {
    type Ux = u32;
}
///`read()` method returns [`hash_hr7::R`](R) reader structure
impl crate::Readable for HASH_HR7rs {}
///`reset()` method sets HASH_HR7 to value 0
impl crate::Resettable for HASH_HR7rs {}
