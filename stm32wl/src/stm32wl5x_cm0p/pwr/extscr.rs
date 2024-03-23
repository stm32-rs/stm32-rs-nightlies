#[doc = "Register `EXTSCR` reader"]
pub type R = crate::R<EXTSCRrs>;
#[doc = "Register `EXTSCR` writer"]
pub type W = crate::W<EXTSCRrs>;
#[doc = "Clear CPU1 Stop Standby flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1CSSFW {
    #[doc = "1: Setting this bit clears the C1STOPF and C1SBF bits"]
    Clear = 1,
}
impl From<C1CSSFW> for bool {
    #[inline(always)]
    fn from(variant: C1CSSFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1CSSF` writer - Clear CPU1 Stop Standby flags"]
pub type C1CSSF_W<'a, REG> = crate::BitWriter<'a, REG, C1CSSFW>;
impl<'a, REG> C1CSSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit clears the C1STOPF and C1SBF bits"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(C1CSSFW::Clear)
    }
}
#[doc = "Field `C2CSSF` writer - lear CPU2 Stop Standby flags"]
pub type C2CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "System Standby flag for CPU1. (no core states retained)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1SBF {
    #[doc = "0: System has not been in Standby mode"]
    NoStandby = 0,
    #[doc = "1: System has been in Standby mode"]
    Standby = 1,
}
impl From<C1SBF> for bool {
    #[inline(always)]
    fn from(variant: C1SBF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1SBF` reader - System Standby flag for CPU1. (no core states retained)"]
pub type C1SBF_R = crate::BitReader<C1SBF>;
impl C1SBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1SBF {
        match self.bits {
            false => C1SBF::NoStandby,
            true => C1SBF::Standby,
        }
    }
    #[doc = "System has not been in Standby mode"]
    #[inline(always)]
    pub fn is_no_standby(&self) -> bool {
        *self == C1SBF::NoStandby
    }
    #[doc = "System has been in Standby mode"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == C1SBF::Standby
    }
}
#[doc = "System Stop2 flag for CPU1. (partial core states retained)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1STOP2F {
    #[doc = "0: System has not been in Stop 2 mode"]
    NoStop = 0,
    #[doc = "1: System has been in Stop 2 mode"]
    Stop = 1,
}
impl From<C1STOP2F> for bool {
    #[inline(always)]
    fn from(variant: C1STOP2F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1STOP2F` reader - System Stop2 flag for CPU1. (partial core states retained)"]
pub type C1STOP2F_R = crate::BitReader<C1STOP2F>;
impl C1STOP2F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1STOP2F {
        match self.bits {
            false => C1STOP2F::NoStop,
            true => C1STOP2F::Stop,
        }
    }
    #[doc = "System has not been in Stop 2 mode"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == C1STOP2F::NoStop
    }
    #[doc = "System has been in Stop 2 mode"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == C1STOP2F::Stop
    }
}
#[doc = "System Stop0, 1 flag for CPU1. (All core states retained)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1STOPF {
    #[doc = "0: System has not been in Stop 0 or 1 mode"]
    NoStop = 0,
    #[doc = "1: System has been in Stop 0 or 1 mode"]
    Stop = 1,
}
impl From<C1STOPF> for bool {
    #[inline(always)]
    fn from(variant: C1STOPF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1STOPF` reader - System Stop0, 1 flag for CPU1. (All core states retained)"]
pub type C1STOPF_R = crate::BitReader<C1STOPF>;
impl C1STOPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1STOPF {
        match self.bits {
            false => C1STOPF::NoStop,
            true => C1STOPF::Stop,
        }
    }
    #[doc = "System has not been in Stop 0 or 1 mode"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == C1STOPF::NoStop
    }
    #[doc = "System has been in Stop 0 or 1 mode"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == C1STOPF::Stop
    }
}
#[doc = "Field `C2SBF` reader - ystem Standby flag for CPU2. (no core states retained)"]
pub type C2SBF_R = crate::BitReader;
#[doc = "Field `C2STOP2F` reader - ystem Stop2 flag for CPU2. (partial core states retained)"]
pub type C2STOP2F_R = crate::BitReader;
#[doc = "Field `C2STOPF` reader - ystem Stop0, 1 flag for CPU2. (All core states retained)"]
pub type C2STOPF_R = crate::BitReader;
#[doc = "CPU1 deepsleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1DS {
    #[doc = "0: CPU is running or in sleep"]
    RunningOrSleep = 0,
    #[doc = "1: CPU is in Deep-Sleep"]
    DeepSleep = 1,
}
impl From<C1DS> for bool {
    #[inline(always)]
    fn from(variant: C1DS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1DS` reader - CPU1 deepsleep mode"]
pub type C1DS_R = crate::BitReader<C1DS>;
impl C1DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1DS {
        match self.bits {
            false => C1DS::RunningOrSleep,
            true => C1DS::DeepSleep,
        }
    }
    #[doc = "CPU is running or in sleep"]
    #[inline(always)]
    pub fn is_running_or_sleep(&self) -> bool {
        *self == C1DS::RunningOrSleep
    }
    #[doc = "CPU is in Deep-Sleep"]
    #[inline(always)]
    pub fn is_deep_sleep(&self) -> bool {
        *self == C1DS::DeepSleep
    }
}
#[doc = "Field `C2DS` reader - PU2 deepsleep mode"]
pub type C2DS_R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - System Standby flag for CPU1. (no core states retained)"]
    #[inline(always)]
    pub fn c1sbf(&self) -> C1SBF_R {
        C1SBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - System Stop2 flag for CPU1. (partial core states retained)"]
    #[inline(always)]
    pub fn c1stop2f(&self) -> C1STOP2F_R {
        C1STOP2F_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - System Stop0, 1 flag for CPU1. (All core states retained)"]
    #[inline(always)]
    pub fn c1stopf(&self) -> C1STOPF_R {
        C1STOPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ystem Standby flag for CPU2. (no core states retained)"]
    #[inline(always)]
    pub fn c2sbf(&self) -> C2SBF_R {
        C2SBF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ystem Stop2 flag for CPU2. (partial core states retained)"]
    #[inline(always)]
    pub fn c2stop2f(&self) -> C2STOP2F_R {
        C2STOP2F_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ystem Stop0, 1 flag for CPU2. (All core states retained)"]
    #[inline(always)]
    pub fn c2stopf(&self) -> C2STOPF_R {
        C2STOPF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU1 deepsleep mode"]
    #[inline(always)]
    pub fn c1ds(&self) -> C1DS_R {
        C1DS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PU2 deepsleep mode"]
    #[inline(always)]
    pub fn c2ds(&self) -> C2DS_R {
        C2DS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear CPU1 Stop Standby flags"]
    #[inline(always)]
    #[must_use]
    pub fn c1cssf(&mut self) -> C1CSSF_W<EXTSCRrs> {
        C1CSSF_W::new(self, 0)
    }
    #[doc = "Bit 1 - lear CPU2 Stop Standby flags"]
    #[inline(always)]
    #[must_use]
    pub fn c2cssf(&mut self) -> C2CSSF_W<EXTSCRrs> {
        C2CSSF_W::new(self, 1)
    }
}
#[doc = "Power extended status and status clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTSCRrs;
impl crate::RegisterSpec for EXTSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extscr::R`](R) reader structure"]
impl crate::Readable for EXTSCRrs {}
#[doc = "`write(|w| ..)` method takes [`extscr::W`](W) writer structure"]
impl crate::Writable for EXTSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTSCR to value 0"]
impl crate::Resettable for EXTSCRrs {
    const RESET_VALUE: u32 = 0;
}
