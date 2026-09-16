///Register `C2CR1` reader
pub type R = crate::R<C2CR1rs>;
///Register `C2CR1` writer
pub type W = crate::W<C2CR1rs>;
///Field `LPMS` reader - Low-power mode selection for CPU2
pub type LPMS_R = crate::FieldReader;
///Field `LPMS` writer - Low-power mode selection for CPU2
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FPDR` reader - Flash power down mode during LPRun for CPU2
pub type FPDR_R = crate::BitReader;
///Field `FPDR` writer - Flash power down mode during LPRun for CPU2
pub type FPDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPDS` reader - Flash power down mode during LPSleep for CPU2
pub type FPDS_R = crate::BitReader;
///Field `FPDS` writer - Flash power down mode during LPSleep for CPU2
pub type FPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLEEWKUP` reader - BLE external wakeup signal
pub type BLEEWKUP_R = crate::BitReader;
///Field `BLEEWKUP` writer - BLE external wakeup signal
pub type BLEEWKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWKUP802` reader - 802.15.4 external wakeup signal
pub type EWKUP802_R = crate::BitReader;
///Field `EWKUP802` writer - 802.15.4 external wakeup signal
pub type EWKUP802_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Low-power mode selection for CPU2
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - Flash power down mode during LPRun for CPU2
    #[inline(always)]
    pub fn fpdr(&self) -> FPDR_R {
        FPDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Flash power down mode during LPSleep for CPU2
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 14 - BLE external wakeup signal
    #[inline(always)]
    pub fn bleewkup(&self) -> BLEEWKUP_R {
        BLEEWKUP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - 802.15.4 external wakeup signal
    #[inline(always)]
    pub fn ewkup802(&self) -> EWKUP802_R {
        EWKUP802_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2CR1")
            .field("ewkup802", &self.ewkup802())
            .field("bleewkup", &self.bleewkup())
            .field("fpds", &self.fpds())
            .field("fpdr", &self.fpdr())
            .field("lpms", &self.lpms())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Low-power mode selection for CPU2
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W<'_, C2CR1rs> {
        LPMS_W::new(self, 0)
    }
    ///Bit 4 - Flash power down mode during LPRun for CPU2
    #[inline(always)]
    pub fn fpdr(&mut self) -> FPDR_W<'_, C2CR1rs> {
        FPDR_W::new(self, 4)
    }
    ///Bit 5 - Flash power down mode during LPSleep for CPU2
    #[inline(always)]
    pub fn fpds(&mut self) -> FPDS_W<'_, C2CR1rs> {
        FPDS_W::new(self, 5)
    }
    ///Bit 14 - BLE external wakeup signal
    #[inline(always)]
    pub fn bleewkup(&mut self) -> BLEEWKUP_W<'_, C2CR1rs> {
        BLEEWKUP_W::new(self, 14)
    }
    ///Bit 15 - 802.15.4 external wakeup signal
    #[inline(always)]
    pub fn ewkup802(&mut self) -> EWKUP802_W<'_, C2CR1rs> {
        EWKUP802_W::new(self, 15)
    }
}
/**CPU2 Power control register 1

You can [`read`](crate::Reg::read) this register and get [`c2cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PWR:C2CR1)*/
pub struct C2CR1rs;
impl crate::RegisterSpec for C2CR1rs {
    type Ux = u32;
}
///`read()` method returns [`c2cr1::R`](R) reader structure
impl crate::Readable for C2CR1rs {}
///`write(|w| ..)` method takes [`c2cr1::W`](W) writer structure
impl crate::Writable for C2CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2CR1 to value 0
impl crate::Resettable for C2CR1rs {}
