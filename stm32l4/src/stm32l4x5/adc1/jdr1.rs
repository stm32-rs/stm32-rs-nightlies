///Register `JDR1` reader
pub type R = crate::R<JDR1rs>;
///Field `JDATA1` reader - JDATA1
pub type JDATA1_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - JDATA1
    #[inline(always)]
    pub fn jdata1(&self) -> JDATA1_R {
        JDATA1_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDR1")
            .field("jdata1", &self.jdata1())
            .finish()
    }
}
/**injected data register 1

You can [`read`](crate::Reg::read) this register and get [`jdr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#ADC1:JDR1)*/
pub struct JDR1rs;
impl crate::RegisterSpec for JDR1rs {
    type Ux = u32;
}
///`read()` method returns [`jdr1::R`](R) reader structure
impl crate::Readable for JDR1rs {}
///`reset()` method sets JDR1 to value 0
impl crate::Resettable for JDR1rs {
    const RESET_VALUE: u32 = 0;
}
