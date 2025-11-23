///Register `HASH_HR6` reader
pub type R = crate::R<HASH_HR6rs>;
///Field `H6` reader - Hash data x
pub type H6_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hash data x
    #[inline(always)]
    pub fn h6(&self) -> H6_R {
        H6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_HR6").field("h6", &self.h6()).finish()
    }
}
/**HASH supplementary digest register 6

You can [`read`](crate::Reg::read) this register and get [`hash_hr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#HASH:HASH_HR6)*/
pub struct HASH_HR6rs;
impl crate::RegisterSpec for HASH_HR6rs {
    type Ux = u32;
}
///`read()` method returns [`hash_hr6::R`](R) reader structure
impl crate::Readable for HASH_HR6rs {}
///`reset()` method sets HASH_HR6 to value 0
impl crate::Resettable for HASH_HR6rs {}
