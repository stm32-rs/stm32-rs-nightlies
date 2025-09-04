///Register `CALR` reader
pub type R = crate::R<CALRrs>;
///Register `CALR` writer
pub type W = crate::W<CALRrs>;
///Field `CALM` reader - Calibration minus
pub type CALM_R = crate::FieldReader<u16>;
///Field `CALM` writer - Calibration minus
pub type CALM_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `LPCAL` reader - RTC low-power mode
pub type LPCAL_R = crate::BitReader;
///Field `LPCAL` writer - RTC low-power mode
pub type LPCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALW16` reader - Use a 16-second calibration cycle period
pub type CALW16_R = crate::BitReader;
///Field `CALW16` writer - Use a 16-second calibration cycle period
pub type CALW16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALW8` reader - Use an 8-second calibration cycle period
pub type CALW8_R = crate::BitReader;
///Field `CALW8` writer - Use an 8-second calibration cycle period
pub type CALW8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALP` reader - Increase frequency of RTC by 488.5 ppm
pub type CALP_R = crate::BitReader;
///Field `CALP` writer - Increase frequency of RTC by 488.5 ppm
pub type CALP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:8 - Calibration minus
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 12 - RTC low-power mode
    #[inline(always)]
    pub fn lpcal(&self) -> LPCAL_R {
        LPCAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Use a 16-second calibration cycle period
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Use an 8-second calibration cycle period
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Increase frequency of RTC by 488.5 ppm
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALR")
            .field("calm", &self.calm())
            .field("lpcal", &self.lpcal())
            .field("calw16", &self.calw16())
            .field("calw8", &self.calw8())
            .field("calp", &self.calp())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Calibration minus
    #[inline(always)]
    pub fn calm(&mut self) -> CALM_W<CALRrs> {
        CALM_W::new(self, 0)
    }
    ///Bit 12 - RTC low-power mode
    #[inline(always)]
    pub fn lpcal(&mut self) -> LPCAL_W<CALRrs> {
        LPCAL_W::new(self, 12)
    }
    ///Bit 13 - Use a 16-second calibration cycle period
    #[inline(always)]
    pub fn calw16(&mut self) -> CALW16_W<CALRrs> {
        CALW16_W::new(self, 13)
    }
    ///Bit 14 - Use an 8-second calibration cycle period
    #[inline(always)]
    pub fn calw8(&mut self) -> CALW8_W<CALRrs> {
        CALW8_W::new(self, 14)
    }
    ///Bit 15 - Increase frequency of RTC by 488.5 ppm
    #[inline(always)]
    pub fn calp(&mut self) -> CALP_W<CALRrs> {
        CALP_W::new(self, 15)
    }
}
/**RTC calibration register

You can [`read`](crate::Reg::read) this register and get [`calr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RTC:CALR)*/
pub struct CALRrs;
impl crate::RegisterSpec for CALRrs {
    type Ux = u32;
}
///`read()` method returns [`calr::R`](R) reader structure
impl crate::Readable for CALRrs {}
///`write(|w| ..)` method takes [`calr::W`](W) writer structure
impl crate::Writable for CALRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CALR to value 0
impl crate::Resettable for CALRrs {}
