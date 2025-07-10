///Register `DR` reader
pub type R = crate::R<DRrs>;
///Field `DATA` reader - Regular data
pub type DATA_R = crate::FieldReader<u16>;
///Field `ADC2DATA` reader - ADC2 data
pub type ADC2DATA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Regular data
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - ADC2 data
    #[inline(always)]
    pub fn adc2data(&self) -> ADC2DATA_R {
        ADC2DATA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("data", &self.data())
            .field("adc2data", &self.adc2data())
            .finish()
    }
}
/**regular data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#ADC1:DR)*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`reset()` method sets DR to value 0
impl crate::Resettable for DRrs {}
