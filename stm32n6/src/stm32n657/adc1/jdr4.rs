///Register `JDR4` reader
pub type R = crate::R<JDR4rs>;
///Field `JDATA` reader - Injected data
pub type JDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Injected data
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDR4")
            .field("jdata", &self.jdata())
            .finish()
    }
}
/**ADC injected channel 4 data register

You can [`read`](crate::Reg::read) this register and get [`jdr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:JDR4)*/
pub struct JDR4rs;
impl crate::RegisterSpec for JDR4rs {
    type Ux = u32;
}
///`read()` method returns [`jdr4::R`](R) reader structure
impl crate::Readable for JDR4rs {}
///`reset()` method sets JDR4 to value 0
impl crate::Resettable for JDR4rs {}
