///Register `HR1` reader
pub type R = crate::R<HR1rs>;
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
        f.debug_struct("HR1").field("hx", &self.hx()).finish()
    }
}
/**HASH digest register

You can [`read`](crate::Reg::read) this register and get [`hr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#HASH:HR1)*/
pub struct HR1rs;
impl crate::RegisterSpec for HR1rs {
    type Ux = u32;
}
///`read()` method returns [`hr1::R`](R) reader structure
impl crate::Readable for HR1rs {}
///`reset()` method sets HR1 to value 0
impl crate::Resettable for HR1rs {}
