///Register `HR15` reader
pub type R = crate::R<HR15rs>;
///Field `H15` reader - Hash data x
pub type H15_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hash data x
    #[inline(always)]
    pub fn h15(&self) -> H15_R {
        H15_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HR15").field("h15", &self.h15()).finish()
    }
}
/**HASH supplementary digest register 15

You can [`read`](crate::Reg::read) this register and get [`hr15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#HASH:HR15)*/
pub struct HR15rs;
impl crate::RegisterSpec for HR15rs {
    type Ux = u32;
}
///`read()` method returns [`hr15::R`](R) reader structure
impl crate::Readable for HR15rs {}
///`reset()` method sets HR15 to value 0
impl crate::Resettable for HR15rs {}
