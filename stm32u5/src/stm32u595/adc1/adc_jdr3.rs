///Register `ADC_JDR3` reader
pub type R = crate::R<ADC_JDR3rs>;
///Field `JDATA` reader - JDATA
pub type JDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - JDATA
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_JDR3")
            .field("jdata", &self.jdata())
            .finish()
    }
}
/**ADC injected data register

You can [`read`](crate::Reg::read) this register and get [`adc_jdr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#ADC1:ADC_JDR3)*/
pub struct ADC_JDR3rs;
impl crate::RegisterSpec for ADC_JDR3rs {
    type Ux = u32;
}
///`read()` method returns [`adc_jdr3::R`](R) reader structure
impl crate::Readable for ADC_JDR3rs {}
///`reset()` method sets ADC_JDR3 to value 0
impl crate::Resettable for ADC_JDR3rs {
    const RESET_VALUE: u32 = 0;
}
