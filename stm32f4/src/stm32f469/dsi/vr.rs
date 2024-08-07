///Register `VR` reader
pub type R = crate::R<VRrs>;
///Register `VR` writer
pub type W = crate::W<VRrs>;
///Field `VERSION` reader - Version of the DSI Host
pub type VERSION_R = crate::FieldReader<u32>;
///Field `VERSION` writer - Version of the DSI Host
pub type VERSION_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
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
impl W {
    ///Bits 0:31 - Version of the DSI Host
    #[inline(always)]
    #[must_use]
    pub fn version(&mut self) -> VERSION_W<VRrs> {
        VERSION_W::new(self, 0)
    }
}
/**DSI Host Version Register

You can [`read`](crate::Reg::read) this register and get [`vr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:VR)*/
pub struct VRrs;
impl crate::RegisterSpec for VRrs {
    type Ux = u32;
}
///`read()` method returns [`vr::R`](R) reader structure
impl crate::Readable for VRrs {}
///`write(|w| ..)` method takes [`vr::W`](W) writer structure
impl crate::Writable for VRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VR to value 0x3133_302a
impl crate::Resettable for VRrs {
    const RESET_VALUE: u32 = 0x3133_302a;
}
