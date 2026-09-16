///Register `HASH_HRA3` reader
pub type R = crate::R<HASH_HRA3rs>;
///Field `H3` reader - Hash data x
pub type H3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hash data x
    #[inline(always)]
    pub fn h3(&self) -> H3_R {
        H3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_HRA3").field("h3", &self.h3()).finish()
    }
}
/**HASH aliased digest register 3

You can [`read`](crate::Reg::read) this register and get [`hash_hra3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#HASH:HASH_HRA3)*/
pub struct HASH_HRA3rs;
impl crate::RegisterSpec for HASH_HRA3rs {
    type Ux = u32;
}
///`read()` method returns [`hash_hra3::R`](R) reader structure
impl crate::Readable for HASH_HRA3rs {}
///`reset()` method sets HASH_HRA3 to value 0
impl crate::Resettable for HASH_HRA3rs {}
