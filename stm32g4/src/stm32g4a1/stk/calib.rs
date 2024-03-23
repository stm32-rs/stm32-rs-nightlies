#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CALIBrs>;
#[doc = "Register `CALIB` writer"]
pub type W = crate::W<CALIBrs>;
#[doc = "Field `TENMS` reader - Calibration value"]
pub type TENMS_R = crate::FieldReader<u32>;
#[doc = "Field `TENMS` writer - Calibration value"]
pub type TENMS_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `SKEW` reader - SKEW flag: Indicates whether the TENMS value is exact"]
pub type SKEW_R = crate::BitReader;
#[doc = "Field `SKEW` writer - SKEW flag: Indicates whether the TENMS value is exact"]
pub type SKEW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOREF` reader - NOREF flag. Reads as zero"]
pub type NOREF_R = crate::BitReader;
#[doc = "Field `NOREF` writer - NOREF flag. Reads as zero"]
pub type NOREF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Calibration value"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - SKEW flag: Indicates whether the TENMS value is exact"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - NOREF flag. Reads as zero"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn tenms(&mut self) -> TENMS_W<CALIBrs> {
        TENMS_W::new(self, 0)
    }
    #[doc = "Bit 30 - SKEW flag: Indicates whether the TENMS value is exact"]
    #[inline(always)]
    #[must_use]
    pub fn skew(&mut self) -> SKEW_W<CALIBrs> {
        SKEW_W::new(self, 30)
    }
    #[doc = "Bit 31 - NOREF flag. Reads as zero"]
    #[inline(always)]
    #[must_use]
    pub fn noref(&mut self) -> NOREF_W<CALIBrs> {
        NOREF_W::new(self, 31)
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
