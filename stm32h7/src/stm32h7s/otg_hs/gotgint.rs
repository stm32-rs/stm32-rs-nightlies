///Register `GOTGINT` reader
pub type R = crate::R<GOTGINTrs>;
///Register `GOTGINT` writer
pub type W = crate::W<GOTGINTrs>;
///Field `SEDET` reader - Session end detected The core sets this bit to indicate that the level of the voltage on V<sub>BUS</sub> is no longer valid for a B-Peripheral session when V<sub>BUS</sub> < 0.8 V. Note: Accessible in both device and host modes.
pub type SEDET_R = crate::BitReader;
///Field `SEDET` writer - Session end detected The core sets this bit to indicate that the level of the voltage on V<sub>BUS</sub> is no longer valid for a B-Peripheral session when V<sub>BUS</sub> < 0.8 V. Note: Accessible in both device and host modes.
pub type SEDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADTOCHG` reader - A-device timeout change The core sets this bit to indicate that the A-device has timed out while waiting for the B-device to connect. Note: Accessible in both device and host modes.
pub type ADTOCHG_R = crate::BitReader;
///Field `ADTOCHG` writer - A-device timeout change The core sets this bit to indicate that the A-device has timed out while waiting for the B-device to connect. Note: Accessible in both device and host modes.
pub type ADTOCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Session end detected The core sets this bit to indicate that the level of the voltage on V<sub>BUS</sub> is no longer valid for a B-Peripheral session when V<sub>BUS</sub> < 0.8 V. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn sedet(&self) -> SEDET_R {
        SEDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 18 - A-device timeout change The core sets this bit to indicate that the A-device has timed out while waiting for the B-device to connect. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn adtochg(&self) -> ADTOCHG_R {
        ADTOCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGINT")
            .field("sedet", &self.sedet())
            .field("adtochg", &self.adtochg())
            .finish()
    }
}
impl W {
    ///Bit 2 - Session end detected The core sets this bit to indicate that the level of the voltage on V<sub>BUS</sub> is no longer valid for a B-Peripheral session when V<sub>BUS</sub> < 0.8 V. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn sedet(&mut self) -> SEDET_W<'_, GOTGINTrs> {
        SEDET_W::new(self, 2)
    }
    ///Bit 18 - A-device timeout change The core sets this bit to indicate that the A-device has timed out while waiting for the B-device to connect. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn adtochg(&mut self) -> ADTOCHG_W<'_, GOTGINTrs> {
        ADTOCHG_W::new(self, 18)
    }
}
/**OTG interrupt register

You can [`read`](crate::Reg::read) this register and get [`gotgint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:GOTGINT)*/
pub struct GOTGINTrs;
impl crate::RegisterSpec for GOTGINTrs {
    type Ux = u32;
}
///`read()` method returns [`gotgint::R`](R) reader structure
impl crate::Readable for GOTGINTrs {}
///`write(|w| ..)` method takes [`gotgint::W`](W) writer structure
impl crate::Writable for GOTGINTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GOTGINT to value 0
impl crate::Resettable for GOTGINTrs {}
