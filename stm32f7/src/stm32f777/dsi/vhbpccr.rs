///Register `VHBPCCR` reader
pub type R = crate::R<VHBPCCRrs>;
///Field `HBP` reader - Horizontal Back-Porch duration
pub type HBP_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - Horizontal Back-Porch duration
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VHBPCCR").field("hbp", &self.hbp()).finish()
    }
}
/**DSI Host Video HBP Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vhbpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#DSI:VHBPCCR)*/
pub struct VHBPCCRrs;
impl crate::RegisterSpec for VHBPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vhbpccr::R`](R) reader structure
impl crate::Readable for VHBPCCRrs {}
///`reset()` method sets VHBPCCR to value 0
impl crate::Resettable for VHBPCCRrs {}
