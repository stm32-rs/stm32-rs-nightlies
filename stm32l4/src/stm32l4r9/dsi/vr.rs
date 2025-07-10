///Register `VR` reader
pub type R = crate::R<VRrs>;
///Field `VERSION` reader - Version of the DSI Host
pub type VERSION_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Version of the DSI Host
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VR")
            .field("version", &self.version())
            .finish()
    }
}
/**DSI Host Version Register

You can [`read`](crate::Reg::read) this register and get [`vr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DSI:VR)*/
pub struct VRrs;
impl crate::RegisterSpec for VRrs {
    type Ux = u32;
}
///`read()` method returns [`vr::R`](R) reader structure
impl crate::Readable for VRrs {}
///`reset()` method sets VR to value 0x3133_302a
impl crate::Resettable for VRrs {
    const RESET_VALUE: u32 = 0x3133_302a;
}
