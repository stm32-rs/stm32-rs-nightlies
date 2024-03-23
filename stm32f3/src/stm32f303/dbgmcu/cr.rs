#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `DBG_SLEEP` reader - Debug Sleep mode"]
pub type DBG_SLEEP_R = crate::BitReader;
#[doc = "Field `DBG_SLEEP` writer - Debug Sleep mode"]
pub type DBG_SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP` reader - Debug Stop Mode"]
pub type DBG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_STOP` writer - Debug Stop Mode"]
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STANDBY` reader - Debug Standby Mode"]
pub type DBG_STANDBY_R = crate::BitReader;
#[doc = "Field `DBG_STANDBY` writer - Debug Standby Mode"]
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_IOEN` reader - Trace pin assignment control"]
pub type TRACE_IOEN_R = crate::BitReader;
#[doc = "Field `TRACE_IOEN` writer - Trace pin assignment control"]
pub type TRACE_IOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_MODE` reader - Trace pin assignment control"]
pub type TRACE_MODE_R = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - Trace pin assignment control"]
pub type TRACE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Debug Sleep mode"]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Stop Mode"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Standby Mode"]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Trace pin assignment control"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Trace pin assignment control"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<CRrs> {
        DBG_SLEEP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Stop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<CRrs> {
        DBG_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Debug Standby Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<CRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
    #[doc = "Bit 5 - Trace pin assignment control"]
    #[inline(always)]
    #[must_use]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<CRrs> {
        TRACE_IOEN_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Trace pin assignment control"]
    #[inline(always)]
    #[must_use]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<CRrs> {
        TRACE_MODE_W::new(self, 6)
    }
}
#[doc = "Debug MCU Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
