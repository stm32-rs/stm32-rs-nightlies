///Register `HRA4` reader
pub type R = crate::R<HRA4rs>;
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
        f.debug_struct("HRA4").field("hx", &self.hx()).finish()
    }
}
/**HASH aliased digest register 4

You can [`read`](crate::Reg::read) this register and get [`hra4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#HASH:HRA4)*/
pub struct HRA4rs;
impl crate::RegisterSpec for HRA4rs {
    type Ux = u32;
}
///`read()` method returns [`hra4::R`](R) reader structure
impl crate::Readable for HRA4rs {}
///`reset()` method sets HRA4 to value 0
impl crate::Resettable for HRA4rs {}
