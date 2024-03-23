#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CALIBrs>;
#[doc = "Register `CALIB` writer"]
pub type W = crate::W<CALIBrs>;
#[doc = "Field `TENMS` reader - Calibration value"]
pub type TENMS_R = crate::FieldReader<u32>;
#[doc = "Field `TENMS` writer - Calibration value"]
pub type TENMS_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Calibration value"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn tenms(&mut self) -> TENMS_W<CALIBrs> {
        TENMS_W::new(self, 0)
    }
}
#[doc = "SysTick calibration value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calib::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calib::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALIBrs;
impl crate::RegisterSpec for CALIBrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calib::R`](R) reader structure"]
impl crate::Readable for CALIBrs {}
#[doc = "`write(|w| ..)` method takes [`calib::W`](W) writer structure"]
impl crate::Writable for CALIBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CALIBrs {
    const RESET_VALUE: u32 = 0;
}
