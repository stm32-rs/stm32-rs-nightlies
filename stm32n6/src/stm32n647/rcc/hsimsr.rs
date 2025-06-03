///Register `HSIMSR` reader
pub type R = crate::R<HSIMSRrs>;
///Field `HSIVAL` reader - HSI clock cycle counter measured value.
pub type HSIVAL_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:10 - HSI clock cycle counter measured value.
    #[inline(always)]
    pub fn hsival(&self) -> HSIVAL_R {
        HSIVAL_R::new((self.bits & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSIMSR")
            .field("hsival", &self.hsival())
            .finish()
    }
}
/**RCC HSI monitor status register

You can [`read`](crate::Reg::read) this register and get [`hsimsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:HSIMSR)*/
pub struct HSIMSRrs;
impl crate::RegisterSpec for HSIMSRrs {
    type Ux = u32;
}
///`read()` method returns [`hsimsr::R`](R) reader structure
impl crate::Readable for HSIMSRrs {}
///`reset()` method sets HSIMSR to value 0
impl crate::Resettable for HSIMSRrs {}
