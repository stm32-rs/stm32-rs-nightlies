///Register `HR10` reader
pub type R = crate::R<HR10rs>;
///Field `H10` reader - Hash data x
pub type H10_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hash data x
    #[inline(always)]
    pub fn h10(&self) -> H10_R {
        H10_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HR10").field("h10", &self.h10()).finish()
    }
}
/**HASH supplementary digest register 10

You can [`read`](crate::Reg::read) this register and get [`hr10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#HASH:HR10)*/
pub struct HR10rs;
impl crate::RegisterSpec for HR10rs {
    type Ux = u32;
}
///`read()` method returns [`hr10::R`](R) reader structure
impl crate::Readable for HR10rs {}
///`reset()` method sets HR10 to value 0
impl crate::Resettable for HR10rs {}
