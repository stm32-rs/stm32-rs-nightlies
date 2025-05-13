///Register `HR14` reader
pub type R = crate::R<HR14rs>;
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
        f.debug_struct("HR14").field("hx", &self.hx()).finish()
    }
}
/**HASH digest register

You can [`read`](crate::Reg::read) this register and get [`hr14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#HASH:HR14)*/
pub struct HR14rs;
impl crate::RegisterSpec for HR14rs {
    type Ux = u32;
}
///`read()` method returns [`hr14::R`](R) reader structure
impl crate::Readable for HR14rs {}
///`reset()` method sets HR14 to value 0
impl crate::Resettable for HR14rs {}
