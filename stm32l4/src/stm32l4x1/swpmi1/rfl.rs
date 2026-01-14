///Register `RFL` reader
pub type R = crate::R<RFLrs>;
///Field `RFL` reader - Receive frame length
pub type RFL_R = crate::FieldReader;
impl R {
    ///Bits 0:4 - Receive frame length
    #[inline(always)]
    pub fn rfl(&self) -> RFL_R {
        RFL_R::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFL").field("rfl", &self.rfl()).finish()
    }
}
/**SWPMI Receive Frame Length register

You can [`read`](crate::Reg::read) this register and get [`rfl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x1.html#SWPMI1:RFL)*/
pub struct RFLrs;
impl crate::RegisterSpec for RFLrs {
    type Ux = u32;
}
///`read()` method returns [`rfl::R`](R) reader structure
impl crate::Readable for RFLrs {}
///`reset()` method sets RFL to value 0
impl crate::Resettable for RFLrs {}
