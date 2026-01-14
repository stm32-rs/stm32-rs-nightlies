///Register `DVBUSPULSE` reader
pub type R = crate::R<DVBUSPULSErs>;
///Register `DVBUSPULSE` writer
pub type W = crate::W<DVBUSPULSErs>;
///Field `DVBUSP` reader - Device VBUS pulsing time
pub type DVBUSP_R = crate::FieldReader<u16>;
///Field `DVBUSP` writer - Device VBUS pulsing time
pub type DVBUSP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Device VBUS pulsing time
    #[inline(always)]
    pub fn dvbusp(&self) -> DVBUSP_R {
        DVBUSP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DVBUSPULSE")
            .field("dvbusp", &self.dvbusp())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Device VBUS pulsing time
    #[inline(always)]
    pub fn dvbusp(&mut self) -> DVBUSP_W<'_, DVBUSPULSErs> {
        DVBUSP_W::new(self, 0)
    }
}
/**OTG_FS device VBUS pulsing time register

You can [`read`](crate::Reg::read) this register and get [`dvbuspulse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbuspulse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#OTG_FS_DEVICE:DVBUSPULSE)*/
pub struct DVBUSPULSErs;
impl crate::RegisterSpec for DVBUSPULSErs {
    type Ux = u32;
}
///`read()` method returns [`dvbuspulse::R`](R) reader structure
impl crate::Readable for DVBUSPULSErs {}
///`write(|w| ..)` method takes [`dvbuspulse::W`](W) writer structure
impl crate::Writable for DVBUSPULSErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DVBUSPULSE to value 0x05b8
impl crate::Resettable for DVBUSPULSErs {
    const RESET_VALUE: u32 = 0x05b8;
}
