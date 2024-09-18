///Register `JDR4` reader
pub type R = crate::R<JDR4rs>;
///Field `JDATA4` reader - ADC group injected sequencer rank 4 conversion data
pub type JDATA4_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ADC group injected sequencer rank 4 conversion data
    #[inline(always)]
    pub fn jdata4(&self) -> JDATA4_R {
        JDATA4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDR4")
            .field("jdata4", &self.jdata4())
            .finish()
    }
}
/**ADC group injected sequencer rank 4 register

You can [`read`](crate::Reg::read) this register and get [`jdr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#ADC3:JDR4)*/
pub struct JDR4rs;
impl crate::RegisterSpec for JDR4rs {
    type Ux = u32;
}
///`read()` method returns [`jdr4::R`](R) reader structure
impl crate::Readable for JDR4rs {}
///`reset()` method sets JDR4 to value 0
impl crate::Resettable for JDR4rs {
    const RESET_VALUE: u32 = 0;
}
