///Register `HRA%s` reader
pub type R = crate::R<HRArs>;
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
        f.debug_struct("HRA").field("h", &self.h()).finish()
    }
}
/**HASH digest register alias %s

You can [`read`](crate::Reg::read) this register and get [`hra::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#HASH:HRA[0])*/
pub struct HRArs;
impl crate::RegisterSpec for HRArs {
    type Ux = u32;
}
///`read()` method returns [`hra::R`](R) reader structure
impl crate::Readable for HRArs {}
///`reset()` method sets HRA%s to value 0
impl crate::Resettable for HRArs {}
