///Register `CALIBR` reader
pub type R = crate::R<CALIBRrs>;
///Register `CALIBR` writer
pub type W = crate::W<CALIBRrs>;
///Field `DC` reader - Digital calibration
pub type DC_R = crate::FieldReader;
///Field `DC` writer - Digital calibration
pub type DC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DCS` reader - Digital calibration sign
pub type DCS_R = crate::BitReader;
///Field `DCS` writer - Digital calibration sign
pub type DCS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Digital calibration
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 7 - Digital calibration sign
    #[inline(always)]
    pub fn dcs(&self) -> DCS_R {
        DCS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALIBR")
            .field("dcs", &self.dcs())
            .field("dc", &self.dc())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Digital calibration
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W<'_, CALIBRrs> {
        DC_W::new(self, 0)
    }
    ///Bit 7 - Digital calibration sign
    #[inline(always)]
    pub fn dcs(&mut self) -> DCS_W<'_, CALIBRrs> {
        DCS_W::new(self, 7)
    }
}
/**calibration register

You can [`read`](crate::Reg::read) this register and get [`calibr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calibr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#RTC:CALIBR)*/
pub struct CALIBRrs;
impl crate::RegisterSpec for CALIBRrs {
    type Ux = u32;
}
///`read()` method returns [`calibr::R`](R) reader structure
impl crate::Readable for CALIBRrs {}
///`write(|w| ..)` method takes [`calibr::W`](W) writer structure
impl crate::Writable for CALIBRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CALIBR to value 0
impl crate::Resettable for CALIBRrs {}
