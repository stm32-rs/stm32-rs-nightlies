///Register `CALIB` reader
pub type R = crate::R<CALIBrs>;
///Register `CALIB` writer
pub type W = crate::W<CALIBrs>;
///Field `TENMS` reader - Calibration value
pub type TENMS_R = crate::FieldReader<u32>;
///Field `TENMS` writer - Calibration value
pub type TENMS_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - Calibration value
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALIB")
            .field("tenms", &self.tenms())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Calibration value
    #[inline(always)]
    pub fn tenms(&mut self) -> TENMS_W<'_, CALIBrs> {
        TENMS_W::new(self, 0)
    }
}
/**SysTick calibration value register

You can [`read`](crate::Reg::read) this register and get [`calib::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calib::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#STK:CALIB)*/
pub struct CALIBrs;
impl crate::RegisterSpec for CALIBrs {
    type Ux = u32;
}
///`read()` method returns [`calib::R`](R) reader structure
impl crate::Readable for CALIBrs {}
///`write(|w| ..)` method takes [`calib::W`](W) writer structure
impl crate::Writable for CALIBrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CALIB to value 0
impl crate::Resettable for CALIBrs {}
