///Register `HR12` reader
pub type R = crate::R<HR12rs>;
///Field `Hx` reader - Hash data x Refer to introduction.
pub type HX_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hash data x Refer to introduction.
    #[inline(always)]
    pub fn hx(&self) -> HX_R {
        HX_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HR12").field("hx", &self.hx()).finish()
    }
}
/**HASH digest register

You can [`read`](crate::Reg::read) this register and get [`hr12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#HASH:HR12)*/
pub struct HR12rs;
impl crate::RegisterSpec for HR12rs {
    type Ux = u32;
}
///`read()` method returns [`hr12::R`](R) reader structure
impl crate::Readable for HR12rs {}
///`reset()` method sets HR12 to value 0
impl crate::Resettable for HR12rs {}
