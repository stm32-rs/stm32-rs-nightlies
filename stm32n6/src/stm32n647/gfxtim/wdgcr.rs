///Register `WDGCR` reader
pub type R = crate::R<WDGCRrs>;
///Field `VALUE` reader - value
pub type VALUE_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - value
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDGCR")
            .field("value", &self.value())
            .finish()
    }
}
/**GFXTIM watchdog counter register

You can [`read`](crate::Reg::read) this register and get [`wdgcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GFXTIM:WDGCR)*/
pub struct WDGCRrs;
impl crate::RegisterSpec for WDGCRrs {
    type Ux = u32;
}
///`read()` method returns [`wdgcr::R`](R) reader structure
impl crate::Readable for WDGCRrs {}
///`reset()` method sets WDGCR to value 0
impl crate::Resettable for WDGCRrs {}
