///Register `VVFPCCR` reader
pub type R = crate::R<VVFPCCRrs>;
///Field `VFP` reader - Vertical Front-Porch duration
pub type VFP_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:9 - Vertical Front-Porch duration
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VVFPCCR").field("vfp", &self.vfp()).finish()
    }
}
/**DSI Host Video VFP Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vvfpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#DSI:VVFPCCR)*/
pub struct VVFPCCRrs;
impl crate::RegisterSpec for VVFPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vvfpccr::R`](R) reader structure
impl crate::Readable for VVFPCCRrs {}
///`reset()` method sets VVFPCCR to value 0
impl crate::Resettable for VVFPCCRrs {}
