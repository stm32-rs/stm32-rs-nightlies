///Register `DSI_VR` reader
pub type R = crate::R<DSI_VRrs>;
///Field `VERSION` reader - Version of the DSI Host This read-only register contains the version of the DSI Host
pub type VERSION_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Version of the DSI Host This read-only register contains the version of the DSI Host
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VR")
            .field("version", &self.version())
            .finish()
    }
}
/**DSI Host version register

You can [`read`](crate::Reg::read) this register and get [`dsi_vr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_VR)*/
pub struct DSI_VRrs;
impl crate::RegisterSpec for DSI_VRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vr::R`](R) reader structure
impl crate::Readable for DSI_VRrs {}
///`reset()` method sets DSI_VR to value 0x3134_312a
impl crate::Resettable for DSI_VRrs {
    const RESET_VALUE: u32 = 0x3134_312a;
}
