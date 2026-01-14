///Register `HR9` reader
pub type R = crate::R<HR9rs>;
///Field `H9` reader - Hash data x
pub type H9_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hash data x
    #[inline(always)]
    pub fn h9(&self) -> H9_R {
        H9_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HR9").field("h9", &self.h9()).finish()
    }
}
/**HASH supplementary digest register 9

You can [`read`](crate::Reg::read) this register and get [`hr9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HASH:HR9)*/
pub struct HR9rs;
impl crate::RegisterSpec for HR9rs {
    type Ux = u32;
}
///`read()` method returns [`hr9::R`](R) reader structure
impl crate::Readable for HR9rs {}
///`reset()` method sets HR9 to value 0
impl crate::Resettable for HR9rs {}
