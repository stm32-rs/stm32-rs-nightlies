#[doc = "Register `LPTIM_CFGR` reader"]
pub type R = crate::R<LPTIM_CFGRrs>;
#[doc = "Register `LPTIM_CFGR` writer"]
pub type W = crate::W<LPTIM_CFGRrs>;
#[doc = "Field `CKSEL` reader - CKSEL"]
pub type CKSEL_R = crate::BitReader;
#[doc = "Field `CKSEL` writer - CKSEL"]
pub type CKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPOL` reader - CKPOL"]
pub type CKPOL_R = crate::FieldReader;
#[doc = "Field `CKPOL` writer - CKPOL"]
pub type CKPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKFLT` reader - CKFLT"]
pub type CKFLT_R = crate::FieldReader;
#[doc = "Field `CKFLT` writer - CKFLT"]
pub type CKFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRGFLT` reader - TRGFLT"]
pub type TRGFLT_R = crate::FieldReader;
#[doc = "Field `TRGFLT` writer - TRGFLT"]
pub type TRGFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRESC` reader - PRESC"]
pub type PRESC_R = crate::FieldReader;
#[doc = "Field `PRESC` writer - PRESC"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRIGSEL` reader - TRIGSEL"]
pub type TRIGSEL_R = crate::FieldReader;
#[doc = "Field `TRIGSEL` writer - TRIGSEL"]
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRIGEN` reader - TRIGEN"]
pub type TRIGEN_R = crate::FieldReader;
#[doc = "Field `TRIGEN` writer - TRIGEN"]
pub type TRIGEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMOUT` reader - TIMOUT"]
pub type TIMOUT_R = crate::BitReader;
#[doc = "Field `TIMOUT` writer - TIMOUT"]
pub type TIMOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVE` reader - WAVE"]
pub type WAVE_R = crate::BitReader;
#[doc = "Field `WAVE` writer - WAVE"]
pub type WAVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVPOL` reader - WAVPOL"]
pub type WAVPOL_R = crate::BitReader;
#[doc = "Field `WAVPOL` writer - WAVPOL"]
pub type WAVPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRELOAD` reader - PRELOAD"]
pub type PRELOAD_R = crate::BitReader;
#[doc = "Field `PRELOAD` writer - PRELOAD"]
pub type PRELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTMODE` reader - COUNTMODE"]
pub type COUNTMODE_R = crate::BitReader;
#[doc = "Field `COUNTMODE` writer - COUNTMODE"]
pub type COUNTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENC` reader - ENC"]
pub type ENC_R = crate::BitReader;
#[doc = "Field `ENC` writer - ENC"]
pub type ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CKSEL"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - CKPOL"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - CKFLT"]
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TRGFLT"]
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 9:11 - PRESC"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 13:15 - TRIGSEL"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 17:18 - TRIGEN"]
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - TIMOUT"]
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WAVE"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WAVPOL"]
    #[inline(always)]
    pub fn wavpol(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PRELOAD"]
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - COUNTMODE"]
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ENC"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<LPTIM_CFGRrs> {
        CKSEL_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - CKPOL"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<LPTIM_CFGRrs> {
        CKPOL_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - CKFLT"]
    #[inline(always)]
    #[must_use]
    pub fn ckflt(&mut self) -> CKFLT_W<LPTIM_CFGRrs> {
        CKFLT_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - TRGFLT"]
    #[inline(always)]
    #[must_use]
    pub fn trgflt(&mut self) -> TRGFLT_W<LPTIM_CFGRrs> {
        TRGFLT_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - PRESC"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<LPTIM_CFGRrs> {
        PRESC_W::new(self, 9)
    }
    #[doc = "Bits 13:15 - TRIGSEL"]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<LPTIM_CFGRrs> {
        TRIGSEL_W::new(self, 13)
    }
    #[doc = "Bits 17:18 - TRIGEN"]
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TRIGEN_W<LPTIM_CFGRrs> {
        TRIGEN_W::new(self, 17)
    }
    #[doc = "Bit 19 - TIMOUT"]
    #[inline(always)]
    #[must_use]
    pub fn timout(&mut self) -> TIMOUT_W<LPTIM_CFGRrs> {
        TIMOUT_W::new(self, 19)
    }
    #[doc = "Bit 20 - WAVE"]
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WAVE_W<LPTIM_CFGRrs> {
        WAVE_W::new(self, 20)
    }
    #[doc = "Bit 21 - WAVPOL"]
    #[inline(always)]
    #[must_use]
    pub fn wavpol(&mut self) -> WAVPOL_W<LPTIM_CFGRrs> {
        WAVPOL_W::new(self, 21)
    }
    #[doc = "Bit 22 - PRELOAD"]
    #[inline(always)]
    #[must_use]
    pub fn preload(&mut self) -> PRELOAD_W<LPTIM_CFGRrs> {
        PRELOAD_W::new(self, 22)
    }
    #[doc = "Bit 23 - COUNTMODE"]
    #[inline(always)]
    #[must_use]
    pub fn countmode(&mut self) -> COUNTMODE_W<LPTIM_CFGRrs> {
        COUNTMODE_W::new(self, 23)
    }
    #[doc = "Bit 24 - ENC"]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> ENC_W<LPTIM_CFGRrs> {
        ENC_W::new(self, 24)
    }
}
#[doc = "LPTIM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPTIM_CFGRrs;
impl crate::RegisterSpec for LPTIM_CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim_cfgr::R`](R) reader structure"]
impl crate::Readable for LPTIM_CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`lptim_cfgr::W`](W) writer structure"]
impl crate::Writable for LPTIM_CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM_CFGR to value 0"]
impl crate::Resettable for LPTIM_CFGRrs {
    const RESET_VALUE: u32 = 0;
}
