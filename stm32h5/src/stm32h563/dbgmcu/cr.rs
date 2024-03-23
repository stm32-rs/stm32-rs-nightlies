#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `DBG_STOP` reader - Allows debug in Stop mode All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state."]
pub type DBG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_STOP` writer - Allows debug in Stop mode All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state."]
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STANDBY` reader - Allows debug in Standby mode All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed."]
pub type DBG_STANDBY_R = crate::BitReader;
#[doc = "Field `DBG_STANDBY` writer - Allows debug in Standby mode All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed."]
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_IOEN` reader - trace pin enable"]
pub type TRACE_IOEN_R = crate::BitReader;
#[doc = "Field `TRACE_IOEN` writer - trace pin enable"]
pub type TRACE_IOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_EN` reader - trace port and clock enable. This bit enables the trace port clock, TRACECK."]
pub type TRACE_EN_R = crate::BitReader;
#[doc = "Field `TRACE_EN` writer - trace port and clock enable. This bit enables the trace port clock, TRACECK."]
pub type TRACE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_MODE` reader - trace pin assignment"]
pub type TRACE_MODE_R = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - trace pin assignment"]
pub type TRACE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCRT` reader - Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials"]
pub type DCRT_R = crate::BitReader;
#[doc = "Field `DCRT` writer - Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials"]
pub type DCRT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Allows debug in Stop mode All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state."]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allows debug in Standby mode All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed."]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - trace pin enable"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - trace port and clock enable. This bit enables the trace port clock, TRACECK."]
    #[inline(always)]
    pub fn trace_en(&self) -> TRACE_EN_R {
        TRACE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - trace pin assignment"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials"]
    #[inline(always)]
    pub fn dcrt(&self) -> DCRT_R {
        DCRT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Allows debug in Stop mode All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<CRrs> {
        DBG_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Allows debug in Standby mode All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<CRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
    #[doc = "Bit 4 - trace pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<CRrs> {
        TRACE_IOEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - trace port and clock enable. This bit enables the trace port clock, TRACECK."]
    #[inline(always)]
    #[must_use]
    pub fn trace_en(&mut self) -> TRACE_EN_W<CRrs> {
        TRACE_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - trace pin assignment"]
    #[inline(always)]
    #[must_use]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<CRrs> {
        TRACE_MODE_W::new(self, 6)
    }
    #[doc = "Bit 16 - Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials"]
    #[inline(always)]
    #[must_use]
    pub fn dcrt(&mut self) -> DCRT_W<CRrs> {
        DCRT_W::new(self, 16)
    }
}
#[doc = "DBGMCU configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
