///Register `CALIB` reader
pub type R = crate::R<CALIBrs>;
///Register `CALIB` writer
pub type W = crate::W<CALIBrs>;
///Field `TENMS` reader - Calibration value
pub type TENMS_R = crate::FieldReader<u32>;
///Field `TENMS` writer - Calibration value
pub type TENMS_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `SKEW` reader - SKEW flag: Indicates whether the TENMS value is exact
pub type SKEW_R = crate::BitReader;
///Field `SKEW` writer - SKEW flag: Indicates whether the TENMS value is exact
pub type SKEW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOREF` reader - NOREF flag. Reads as zero
pub type NOREF_R = crate::BitReader;
///Field `NOREF` writer - NOREF flag. Reads as zero
pub type NOREF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:23 - Calibration value
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 30 - SKEW flag: Indicates whether the TENMS value is exact
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - NOREF flag. Reads as zero
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALIB")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Calibration value
    #[inline(always)]
    pub fn tenms(&mut self) -> TENMS_W<'_, CALIBrs> {
        TENMS_W::new(self, 0)
    }
    ///Bit 30 - SKEW flag: Indicates whether the TENMS value is exact
    #[inline(always)]
    pub fn skew(&mut self) -> SKEW_W<'_, CALIBrs> {
        SKEW_W::new(self, 30)
    }
    ///Bit 31 - NOREF flag. Reads as zero
    #[inline(always)]
    pub fn noref(&mut self) -> NOREF_W<'_, CALIBrs> {
        NOREF_W::new(self, 31)
    }
}
/**SysTick calibration value register

You can [`read`](crate::Reg::read) this register and get [`calib::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calib::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x8.html#STK:CALIB)*/
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
