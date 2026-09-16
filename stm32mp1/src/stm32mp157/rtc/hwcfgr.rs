///Register `HWCFGR` reader
pub type R = crate::R<HWCFGRrs>;
///Field `ALARMB` reader - ALARMB
pub type ALARMB_R = crate::FieldReader;
///Field `WAKEUP` reader - WAKEUP
pub type WAKEUP_R = crate::FieldReader;
///Field `SMOOTH_CALIB` reader - SMOOTH_CALIB
pub type SMOOTH_CALIB_R = crate::FieldReader;
///Field `TIMESTAMP` reader - TIMESTAMP
pub type TIMESTAMP_R = crate::FieldReader;
///Field `OPTIONREG_OUT` reader - OPTIONREG_OUT
pub type OPTIONREG_OUT_R = crate::FieldReader;
///Field `TRUST_ZONE` reader - TRUST_ZONE
pub type TRUST_ZONE_R = crate::FieldReader;
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
/**RTC hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RTC:HWCFGR)*/
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr::R`](R) reader structure
impl crate::Readable for HWCFGRrs {}
///`reset()` method sets HWCFGR to value 0x0103_1111
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x0103_1111;
}
