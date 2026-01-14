///Register `DR` reader
pub type R = crate::R<DRrs>;
///Field `RegularDATA` reader - Regular data
pub type REGULAR_DATA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Regular data
    #[inline(always)]
    pub fn regular_data(&self) -> REGULAR_DATA_R {
        REGULAR_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("regular_data", &self.regular_data())
            .finish()
    }
}
/**regular data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#ADC:DR)*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`reset()` method sets DR to value 0
impl crate::Resettable for DRrs {}
