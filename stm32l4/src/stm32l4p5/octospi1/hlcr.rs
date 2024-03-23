#[doc = "Register `HLCR` reader"]
pub type R = crate::R<HLCRrs>;
#[doc = "Register `HLCR` writer"]
pub type W = crate::W<HLCRrs>;
#[doc = "Latency mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LM {
    #[doc = "0: Variable initial latency"]
    Variable = 0,
    #[doc = "1: Fixed latency"]
    Fixed = 1,
}
impl From<LM> for bool {
    #[inline(always)]
    fn from(variant: LM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LM` reader - Latency mode"]
pub type LM_R = crate::BitReader<LM>;
impl LM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LM {
        match self.bits {
            false => LM::Variable,
            true => LM::Fixed,
        }
    }
    #[doc = "Variable initial latency"]
    #[inline(always)]
    pub fn is_variable(&self) -> bool {
        *self == LM::Variable
    }
    #[doc = "Fixed latency"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == LM::Fixed
    }
}
#[doc = "Field `LM` writer - Latency mode"]
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG, LM>;
impl<'a, REG> LM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Variable initial latency"]
    #[inline(always)]
    pub fn variable(self) -> &'a mut crate::W<REG> {
        self.variant(LM::Variable)
    }
    #[doc = "Fixed latency"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(LM::Fixed)
    }
}
#[doc = "Write zero latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WZL {
    #[doc = "0: Latency on write accesses"]
    Enabled = 0,
    #[doc = "1: No latency on write accesses"]
    Disabled = 1,
}
impl From<WZL> for bool {
    #[inline(always)]
    fn from(variant: WZL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WZL` reader - Write zero latency"]
pub type WZL_R = crate::BitReader<WZL>;
impl WZL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WZL {
        match self.bits {
            false => WZL::Enabled,
            true => WZL::Disabled,
        }
    }
    #[doc = "Latency on write accesses"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WZL::Enabled
    }
    #[doc = "No latency on write accesses"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WZL::Disabled
    }
}
#[doc = "Field `WZL` writer - Write zero latency"]
pub type WZL_W<'a, REG> = crate::BitWriter<'a, REG, WZL>;
impl<'a, REG> WZL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Latency on write accesses"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WZL::Enabled)
    }
    #[doc = "No latency on write accesses"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WZL::Disabled)
    }
}
#[doc = "Field `TACC` reader - Access time"]
pub type TACC_R = crate::FieldReader;
#[doc = "Field `TACC` writer - Access time"]
pub type TACC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `TRWR` reader - Read write recovery time"]
pub type TRWR_R = crate::FieldReader;
#[doc = "Field `TRWR` writer - Read write recovery time"]
pub type TRWR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Latency mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write zero latency"]
    #[inline(always)]
    pub fn wzl(&self) -> WZL_R {
        WZL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Access time"]
    #[inline(always)]
    pub fn tacc(&self) -> TACC_R {
        TACC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Read write recovery time"]
    #[inline(always)]
    pub fn trwr(&self) -> TRWR_R {
        TRWR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Latency mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<HLCRrs> {
        LM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write zero latency"]
    #[inline(always)]
    #[must_use]
    pub fn wzl(&mut self) -> WZL_W<HLCRrs> {
        WZL_W::new(self, 1)
    }
    #[doc = "Bits 8:15 - Access time"]
    #[inline(always)]
    #[must_use]
    pub fn tacc(&mut self) -> TACC_W<HLCRrs> {
        TACC_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Read write recovery time"]
    #[inline(always)]
    #[must_use]
    pub fn trwr(&mut self) -> TRWR_W<HLCRrs> {
        TRWR_W::new(self, 16)
    }
}
#[doc = "HyperBusTM latency configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hlcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hlcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HLCRrs;
impl crate::RegisterSpec for HLCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hlcr::R`](R) reader structure"]
impl crate::Readable for HLCRrs {}
#[doc = "`write(|w| ..)` method takes [`hlcr::W`](W) writer structure"]
impl crate::Writable for HLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HLCR to value 0"]
impl crate::Resettable for HLCRrs {
    const RESET_VALUE: u32 = 0;
}
