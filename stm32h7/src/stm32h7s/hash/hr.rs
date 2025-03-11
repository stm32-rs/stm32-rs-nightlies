///Register `HR%s` reader
pub type R = crate::R<HRrs>;
///Field `H` reader - Hash data x Refer to Section 34.7.4: HASH digest registers introduction.
pub type H_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hash data x Refer to Section 34.7.4: HASH digest registers introduction.
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HR").field("h", &self.h()).finish()
    }
}
/**HASH digest register %s

You can [`read`](crate::Reg::read) this register and get [`hr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#HASH:HR[0])*/
pub struct HRrs;
impl crate::RegisterSpec for HRrs {
    type Ux = u32;
}
///`read()` method returns [`hr::R`](R) reader structure
impl crate::Readable for HRrs {}
///`reset()` method sets HR%s to value 0
impl crate::Resettable for HRrs {}
