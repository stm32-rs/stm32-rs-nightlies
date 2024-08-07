///Register `OTG_DVBUSDIS` reader
pub type R = crate::R<OTG_DVBUSDISrs>;
///Register `OTG_DVBUSDIS` writer
pub type W = crate::W<OTG_DVBUSDISrs>;
///Field `VBUSDT` reader - VBUSDT
pub type VBUSDT_R = crate::FieldReader<u16>;
///Field `VBUSDT` writer - VBUSDT
pub type VBUSDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - VBUSDT
    #[inline(always)]
    pub fn vbusdt(&self) -> VBUSDT_R {
        VBUSDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_DVBUSDIS")
            .field("vbusdt", &self.vbusdt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - VBUSDT
    #[inline(always)]
    #[must_use]
    pub fn vbusdt(&mut self) -> VBUSDT_W<OTG_DVBUSDISrs> {
        VBUSDT_W::new(self, 0)
    }
}
/**This register specifies the VBUS discharge time after VBUS pulsing during SRP.

You can [`read`](crate::Reg::read) this register and get [`otg_dvbusdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_dvbusdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:OTG_DVBUSDIS)*/
pub struct OTG_DVBUSDISrs;
impl crate::RegisterSpec for OTG_DVBUSDISrs {
    type Ux = u32;
}
///`read()` method returns [`otg_dvbusdis::R`](R) reader structure
impl crate::Readable for OTG_DVBUSDISrs {}
///`write(|w| ..)` method takes [`otg_dvbusdis::W`](W) writer structure
impl crate::Writable for OTG_DVBUSDISrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTG_DVBUSDIS to value 0x17d7
impl crate::Resettable for OTG_DVBUSDISrs {
    const RESET_VALUE: u32 = 0x17d7;
}
