///Register `HR11` reader
pub type R = crate::R<HR11rs>;
///Field `H11` reader - Hash data x
pub type H11_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hash data x
    #[inline(always)]
    pub fn h11(&self) -> H11_R {
        H11_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HR11").field("h11", &self.h11()).finish()
    }
}
/**HASH supplementary digest register 11

You can [`read`](crate::Reg::read) this register and get [`hr11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#HASH:HR11)*/
pub struct HR11rs;
impl crate::RegisterSpec for HR11rs {
    type Ux = u32;
}
///`read()` method returns [`hr11::R`](R) reader structure
impl crate::Readable for HR11rs {}
///`reset()` method sets HR11 to value 0
impl crate::Resettable for HR11rs {}
