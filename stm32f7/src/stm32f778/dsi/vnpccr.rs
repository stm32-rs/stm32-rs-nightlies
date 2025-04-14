///Register `VNPCCR` reader
pub type R = crate::R<VNPCCRrs>;
///Field `NPSIZE` reader - Null Packet Size
pub type NPSIZE_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:12 - Null Packet Size
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VNPCCR")
            .field("npsize", &self.npsize())
            .finish()
    }
}
/**DSI Host Video Null Packet Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vnpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#DSI:VNPCCR)*/
pub struct VNPCCRrs;
impl crate::RegisterSpec for VNPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vnpccr::R`](R) reader structure
impl crate::Readable for VNPCCRrs {}
///`reset()` method sets VNPCCR to value 0
impl crate::Resettable for VNPCCRrs {}
