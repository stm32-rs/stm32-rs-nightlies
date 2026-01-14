///Register `OTG_HS_DVBUSPULSE` reader
pub type R = crate::R<OTG_HS_DVBUSPULSErs>;
///Register `OTG_HS_DVBUSPULSE` writer
pub type W = crate::W<OTG_HS_DVBUSPULSErs>;
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
        f.debug_struct("OTG_HS_DVBUSPULSE")
            .field("dvbusp", &self.dvbusp())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Device VBUS pulsing time
    #[inline(always)]
    pub fn dvbusp(&mut self) -> DVBUSP_W<'_, OTG_HS_DVBUSPULSErs> {
        DVBUSP_W::new(self, 0)
    }
}
/**OTG_HS device VBUS pulsing time register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dvbuspulse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dvbuspulse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_HS_DEVICE:OTG_HS_DVBUSPULSE)*/
pub struct OTG_HS_DVBUSPULSErs;
impl crate::RegisterSpec for OTG_HS_DVBUSPULSErs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_dvbuspulse::R`](R) reader structure
impl crate::Readable for OTG_HS_DVBUSPULSErs {}
///`write(|w| ..)` method takes [`otg_hs_dvbuspulse::W`](W) writer structure
impl crate::Writable for OTG_HS_DVBUSPULSErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_DVBUSPULSE to value 0x05b8
impl crate::Resettable for OTG_HS_DVBUSPULSErs {
    const RESET_VALUE: u32 = 0x05b8;
}
