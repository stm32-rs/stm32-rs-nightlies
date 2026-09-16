///Register `PTSR` reader
pub type R = crate::R<PTSRrs>;
///Field `TDO` reader - CSI PHY test interface data output bus for read-back and internal probing functionalities
pub type TDO_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - CSI PHY test interface data output bus for read-back and internal probing functionalities
    #[inline(always)]
    pub fn tdo(&self) -> TDO_R {
        TDO_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTSR").field("tdo", &self.tdo()).finish()
    }
}
/**CSI PHY test status register

You can [`read`](crate::Reg::read) this register and get [`ptsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#CSI:PTSR)*/
pub struct PTSRrs;
impl crate::RegisterSpec for PTSRrs {
    type Ux = u32;
}
///`read()` method returns [`ptsr::R`](R) reader structure
impl crate::Readable for PTSRrs {}
///`reset()` method sets PTSR to value 0
impl crate::Resettable for PTSRrs {}
