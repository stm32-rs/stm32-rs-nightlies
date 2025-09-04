///Register `HR4` reader
pub type R = crate::R<HR4rs>;
///Field `H4` reader - Hash data x
pub type H4_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hash data x
    #[inline(always)]
    pub fn h4(&self) -> H4_R {
        H4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HR4").field("h4", &self.h4()).finish()
    }
}
/**HASH digest register 4

You can [`read`](crate::Reg::read) this register and get [`hr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#HASH:HR4)*/
pub struct HR4rs;
impl crate::RegisterSpec for HR4rs {
    type Ux = u32;
}
///`read()` method returns [`hr4::R`](R) reader structure
impl crate::Readable for HR4rs {}
///`reset()` method sets HR4 to value 0
impl crate::Resettable for HR4rs {}
