///Register `HASH_HR5` reader
pub type R = crate::R<HASH_HR5rs>;
///Field `H5` reader - H5
pub type H5_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - H5
    #[inline(always)]
    pub fn h5(&self) -> H5_R {
        H5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_HR5").field("h5", &self.h5()).finish()
    }
}
/**HASH digest register 5

You can [`read`](crate::Reg::read) this register and get [`hash_hr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HASH2:HASH_HR5)*/
pub struct HASH_HR5rs;
impl crate::RegisterSpec for HASH_HR5rs {
    type Ux = u32;
}
///`read()` method returns [`hash_hr5::R`](R) reader structure
impl crate::Readable for HASH_HR5rs {}
///`reset()` method sets HASH_HR5 to value 0
impl crate::Resettable for HASH_HR5rs {
    const RESET_VALUE: u32 = 0;
}
