///Register `JDR1` reader
pub type R = crate::R<JDR1rs>;
///Field `JDATA` reader - Injected data
pub type JDATA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Injected data
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDR1")
            .field("jdata", &self.jdata())
            .finish()
    }
}
/**injected data register x

You can [`read`](crate::Reg::read) this register and get [`jdr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#ADC1:JDR1)*/
pub struct JDR1rs;
impl crate::RegisterSpec for JDR1rs {
    type Ux = u32;
}
///`read()` method returns [`jdr1::R`](R) reader structure
impl crate::Readable for JDR1rs {}
///`reset()` method sets JDR1 to value 0
impl crate::Resettable for JDR1rs {}
