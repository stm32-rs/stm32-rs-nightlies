///Register `HWCFGR` reader
pub type R = crate::R<HWCFGRrs>;
///Register `HWCFGR` writer
pub type W = crate::W<HWCFGRrs>;
///Field `ALARMB` reader - ALARMB
pub type ALARMB_R = crate::FieldReader;
///Field `ALARMB` writer - ALARMB
pub type ALARMB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `WAKEUP` reader - WAKEUP
pub type WAKEUP_R = crate::FieldReader;
///Field `WAKEUP` writer - WAKEUP
pub type WAKEUP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SMOOTH_CALIB` reader - SMOOTH_CALIB
pub type SMOOTH_CALIB_R = crate::FieldReader;
///Field `SMOOTH_CALIB` writer - SMOOTH_CALIB
pub type SMOOTH_CALIB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TIMESTAMP` reader - TIMESTAMP
pub type TIMESTAMP_R = crate::FieldReader;
///Field `TIMESTAMP` writer - TIMESTAMP
pub type TIMESTAMP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `OPTIONREG_OUT` reader - OPTIONREG_OUT
pub type OPTIONREG_OUT_R = crate::FieldReader;
///Field `OPTIONREG_OUT` writer - OPTIONREG_OUT
pub type OPTIONREG_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TRUST_ZONE` reader - TRUST_ZONE
pub type TRUST_ZONE_R = crate::FieldReader;
///Field `TRUST_ZONE` writer - TRUST_ZONE
pub type TRUST_ZONE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - ALARMB
    #[inline(always)]
    pub fn alarmb(&self) -> ALARMB_R {
        ALARMB_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - WAKEUP
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - SMOOTH_CALIB
    #[inline(always)]
    pub fn smooth_calib(&self) -> SMOOTH_CALIB_R {
        SMOOTH_CALIB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - TIMESTAMP
    #[inline(always)]
    pub fn timestamp(&self) -> TIMESTAMP_R {
        TIMESTAMP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:23 - OPTIONREG_OUT
    #[inline(always)]
    pub fn optionreg_out(&self) -> OPTIONREG_OUT_R {
        OPTIONREG_OUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - TRUST_ZONE
    #[inline(always)]
    pub fn trust_zone(&self) -> TRUST_ZONE_R {
        TRUST_ZONE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR")
            .field("alarmb", &self.alarmb())
            .field("wakeup", &self.wakeup())
            .field("smooth_calib", &self.smooth_calib())
            .field("timestamp", &self.timestamp())
            .field("optionreg_out", &self.optionreg_out())
            .field("trust_zone", &self.trust_zone())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - ALARMB
    #[inline(always)]
    pub fn alarmb(&mut self) -> ALARMB_W<'_, HWCFGRrs> {
        ALARMB_W::new(self, 0)
    }
    ///Bits 4:7 - WAKEUP
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W<'_, HWCFGRrs> {
        WAKEUP_W::new(self, 4)
    }
    ///Bits 8:11 - SMOOTH_CALIB
    #[inline(always)]
    pub fn smooth_calib(&mut self) -> SMOOTH_CALIB_W<'_, HWCFGRrs> {
        SMOOTH_CALIB_W::new(self, 8)
    }
    ///Bits 12:15 - TIMESTAMP
    #[inline(always)]
    pub fn timestamp(&mut self) -> TIMESTAMP_W<'_, HWCFGRrs> {
        TIMESTAMP_W::new(self, 12)
    }
    ///Bits 16:23 - OPTIONREG_OUT
    #[inline(always)]
    pub fn optionreg_out(&mut self) -> OPTIONREG_OUT_W<'_, HWCFGRrs> {
        OPTIONREG_OUT_W::new(self, 16)
    }
    ///Bits 24:27 - TRUST_ZONE
    #[inline(always)]
    pub fn trust_zone(&mut self) -> TRUST_ZONE_W<'_, HWCFGRrs> {
        TRUST_ZONE_W::new(self, 24)
    }
}
/**hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#RTC:HWCFGR)*/
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr::R`](R) reader structure
impl crate::Readable for HWCFGRrs {}
///`write(|w| ..)` method takes [`hwcfgr::W`](W) writer structure
impl crate::Writable for HWCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HWCFGR to value 0
impl crate::Resettable for HWCFGRrs {}
