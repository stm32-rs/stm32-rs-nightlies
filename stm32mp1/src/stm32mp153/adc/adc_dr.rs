///Register `ADC_DR` reader
pub type R = crate::R<ADC_DRrs>;
///Field `RDATA` reader - RDATA
pub type RDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - RDATA
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_DR")
            .field("rdata", &self.rdata())
            .finish()
    }
}
/**ADC regular Data Register

You can [`read`](crate::Reg::read) this register and get [`adc_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC:ADC_DR)*/
pub struct ADC_DRrs;
impl crate::RegisterSpec for ADC_DRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_dr::R`](R) reader structure
impl crate::Readable for ADC_DRrs {}
///`reset()` method sets ADC_DR to value 0
impl crate::Resettable for ADC_DRrs {
    const RESET_VALUE: u32 = 0;
}
