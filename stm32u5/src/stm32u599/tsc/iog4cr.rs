///Register `IOG4CR` reader
pub type R = crate::R<IOG4CRrs>;
///Field `CNT` reader - Counter value
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:13 - Counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOG4CR").field("cnt", &self.cnt()).finish()
    }
}
/**I/O group x counter register

You can [`read`](crate::Reg::read) this register and get [`iog4cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#TSC:IOG4CR)*/
pub struct IOG4CRrs;
impl crate::RegisterSpec for IOG4CRrs {
    type Ux = u32;
}
///`read()` method returns [`iog4cr::R`](R) reader structure
impl crate::Readable for IOG4CRrs {}
///`reset()` method sets IOG4CR to value 0
impl crate::Resettable for IOG4CRrs {
    const RESET_VALUE: u32 = 0;
}
