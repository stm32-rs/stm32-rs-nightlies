///Register `HR8` reader
pub type R = crate::R<HR8rs>;
///Field `H8` reader - Hash data x
pub type H8_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hash data x
    #[inline(always)]
    pub fn h8(&self) -> H8_R {
        H8_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HR8").field("h8", &self.h8()).finish()
    }
}
/**HASH supplementary digest register 8

You can [`read`](crate::Reg::read) this register and get [`hr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HASH:HR8)*/
pub struct HR8rs;
impl crate::RegisterSpec for HR8rs {
    type Ux = u32;
}
///`read()` method returns [`hr8::R`](R) reader structure
impl crate::Readable for HR8rs {}
///`reset()` method sets HR8 to value 0
impl crate::Resettable for HR8rs {}
