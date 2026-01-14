///Register `HR13` reader
pub type R = crate::R<HR13rs>;
///Field `H13` reader - Hash data x
pub type H13_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hash data x
    #[inline(always)]
    pub fn h13(&self) -> H13_R {
        H13_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HR13").field("h13", &self.h13()).finish()
    }
}
/**HASH supplementary digest register 13

You can [`read`](crate::Reg::read) this register and get [`hr13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HASH:HR13)*/
pub struct HR13rs;
impl crate::RegisterSpec for HR13rs {
    type Ux = u32;
}
///`read()` method returns [`hr13::R`](R) reader structure
impl crate::Readable for HR13rs {}
///`reset()` method sets HR13 to value 0
impl crate::Resettable for HR13rs {}
