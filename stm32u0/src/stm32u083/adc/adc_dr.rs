///Register `ADC_DR` reader
pub type R = crate::R<ADC_DRrs>;
/**Field `DATA` reader - Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in Figure141: Data alignment and resolution (oversampling disabled: OVSE = 0) on page1332. Just after a calibration is complete, DATA\[6:0\]
contains the calibration factor.*/
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    /**Bits 0:15 - Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in Figure141: Data alignment and resolution (oversampling disabled: OVSE = 0) on page1332. Just after a calibration is complete, DATA\[6:0\]
    contains the calibration factor.*/
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_DR")
            .field("data", &self.data())
            .finish()
    }
}
/**ADC data register

You can [`read`](crate::Reg::read) this register and get [`adc_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#ADC:ADC_DR)*/
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
