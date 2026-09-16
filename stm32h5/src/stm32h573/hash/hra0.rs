///Register `HRA0` reader
pub type R = crate::R<HRA0rs>;
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
        f.debug_struct("HRA0").field("hx", &self.hx()).finish()
    }
}
/**HASH aliased digest register 0

You can [`read`](crate::Reg::read) this register and get [`hra0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#HASH:HRA0)*/
pub struct HRA0rs;
impl crate::RegisterSpec for HRA0rs {
    type Ux = u32;
}
///`read()` method returns [`hra0::R`](R) reader structure
impl crate::Readable for HRA0rs {}
///`reset()` method sets HRA0 to value 0
impl crate::Resettable for HRA0rs {}
