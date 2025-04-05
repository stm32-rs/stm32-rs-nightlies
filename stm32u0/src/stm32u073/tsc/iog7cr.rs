///Register `IOG7CR` reader
pub type R = crate::R<IOG7CRrs>;
///Field `CNT` reader - Counter value These bits represent the number of charge transfer cycles generated on the analog I/O group x to complete its acquisition (voltage across C<sub>S</sub> has reached the threshold).
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:13 - Counter value These bits represent the number of charge transfer cycles generated on the analog I/O group x to complete its acquisition (voltage across C<sub>S</sub> has reached the threshold).
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOG7CR").field("cnt", &self.cnt()).finish()
    }
}
/**TSC I/O group 7 counter register

You can [`read`](crate::Reg::read) this register and get [`iog7cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TSC:IOG7CR)*/
pub struct IOG7CRrs;
impl crate::RegisterSpec for IOG7CRrs {
    type Ux = u32;
}
///`read()` method returns [`iog7cr::R`](R) reader structure
impl crate::Readable for IOG7CRrs {}
///`reset()` method sets IOG7CR to value 0
impl crate::Resettable for IOG7CRrs {}
