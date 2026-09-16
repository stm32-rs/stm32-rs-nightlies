///Register `JDR4` reader
pub type R = crate::R<JDR4rs>;
///Field `JDATA` reader - Injected data These bits are read-only. They contain the conversion result from injected channel y. The data are left -or right-aligned as described in Section 25.4.26: Data management.
pub type JDATA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Injected data These bits are read-only. They contain the conversion result from injected channel y. The data are left -or right-aligned as described in Section 25.4.26: Data management.
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ADC1:JDR4)*/
pub struct JDR4rs;
impl crate::RegisterSpec for JDR4rs {
    type Ux = u32;
}
///`read()` method returns [`jdr4::R`](R) reader structure
impl crate::Readable for JDR4rs {}
///`reset()` method sets JDR4 to value 0
impl crate::Resettable for JDR4rs {}
