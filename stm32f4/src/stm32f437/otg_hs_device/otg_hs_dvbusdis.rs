///Register `OTG_HS_DVBUSDIS` reader
pub type R = crate::R<OTG_HS_DVBUSDISrs>;
///Register `OTG_HS_DVBUSDIS` writer
pub type W = crate::W<OTG_HS_DVBUSDISrs>;
///Field `VBUSDT` reader - Device VBUS discharge time
pub type VBUSDT_R = crate::FieldReader<u16>;
///Field `VBUSDT` writer - Device VBUS discharge time
pub type VBUSDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Device VBUS discharge time
    #[inline(always)]
    pub fn vbusdt(&self) -> VBUSDT_R {
        VBUSDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_DVBUSDIS")
            .field("vbusdt", &self.vbusdt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Device VBUS discharge time
    #[inline(always)]
    pub fn vbusdt(&mut self) -> VBUSDT_W<'_, OTG_HS_DVBUSDISrs> {
        VBUSDT_W::new(self, 0)
    }
}
/**OTG_HS device VBUS discharge time register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dvbusdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dvbusdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_DEVICE:OTG_HS_DVBUSDIS)*/
pub struct OTG_HS_DVBUSDISrs;
impl crate::RegisterSpec for OTG_HS_DVBUSDISrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_dvbusdis::R`](R) reader structure
impl crate::Readable for OTG_HS_DVBUSDISrs {}
///`write(|w| ..)` method takes [`otg_hs_dvbusdis::W`](W) writer structure
impl crate::Writable for OTG_HS_DVBUSDISrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_DVBUSDIS to value 0x17d7
impl crate::Resettable for OTG_HS_DVBUSDISrs {
    const RESET_VALUE: u32 = 0x17d7;
}
