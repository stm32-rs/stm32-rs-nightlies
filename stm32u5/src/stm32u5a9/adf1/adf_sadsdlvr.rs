///Register `ADF_SADSDLVR` reader
pub type R = crate::R<ADF_SADSDLVRrs>;
///Field `SDLVL` reader - SDLVL
pub type SDLVL_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:14 - SDLVL
    #[inline(always)]
    pub fn sdlvl(&self) -> SDLVL_R {
        SDLVL_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADF_SADSDLVR")
            .field("sdlvl", &self.sdlvl())
            .finish()
    }
}
/**ADF SAD sound level register

You can [`read`](crate::Reg::read) this register and get [`adf_sadsdlvr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#ADF1:ADF_SADSDLVR)*/
pub struct ADF_SADSDLVRrs;
impl crate::RegisterSpec for ADF_SADSDLVRrs {
    type Ux = u32;
}
///`read()` method returns [`adf_sadsdlvr::R`](R) reader structure
impl crate::Readable for ADF_SADSDLVRrs {}
///`reset()` method sets ADF_SADSDLVR to value 0
impl crate::Resettable for ADF_SADSDLVRrs {
    const RESET_VALUE: u32 = 0;
}
