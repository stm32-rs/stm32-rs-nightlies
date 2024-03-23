#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACRrs>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACRrs>;
#[doc = "Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY {
    #[doc = "0: Zero Wait States (Vcore Boost 1 (&lt;= 34MHz), Vcore Normal 1 (&lt;= 30MHz), Vcore 2 (&lt;= 12MHz)"]
    Wait0 = 0,
    #[doc = "1: One Wait State (Vcore Boost 1 (&lt;= 68MHz), Vcore Normal 1 (&lt;= 60MHz), Vcore 2 (&lt;= 24MHz)"]
    Wait1 = 1,
    #[doc = "2: Two Wait States (Vcore Boost 1 (&lt;= 102MHz), Vcore Normal 1 (&lt;= 90MHz), Vcore 2 (&lt;= 26MHz)"]
    Wait2 = 2,
    #[doc = "3: Three Wait States (Vcore Boost 1 (&lt;= 136MHz), Vcore Normal 1 (&lt;= 120MHz)"]
    Wait3 = 3,
    #[doc = "4: Four Wait States (Vcore Boost 1 (&lt;= 170MHz), Vcore Normal 1 (&lt;= 150MHz)"]
    Wait4 = 4,
}
impl From<LATENCY> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LATENCY {
    type Ux = u8;
}
#[doc = "Field `LATENCY` reader - Latency"]
pub type LATENCY_R = crate::FieldReader<LATENCY>;
impl LATENCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LATENCY> {
        match self.bits {
            0 => Some(LATENCY::Wait0),
            1 => Some(LATENCY::Wait1),
            2 => Some(LATENCY::Wait2),
            3 => Some(LATENCY::Wait3),
            4 => Some(LATENCY::Wait4),
            _ => None,
        }
    }
    #[doc = "Zero Wait States (Vcore Boost 1 (&lt;= 34MHz), Vcore Normal 1 (&lt;= 30MHz), Vcore 2 (&lt;= 12MHz)"]
    #[inline(always)]
    pub fn is_wait0(&self) -> bool {
        *self == LATENCY::Wait0
    }
    #[doc = "One Wait State (Vcore Boost 1 (&lt;= 68MHz), Vcore Normal 1 (&lt;= 60MHz), Vcore 2 (&lt;= 24MHz)"]
    #[inline(always)]
    pub fn is_wait1(&self) -> bool {
        *self == LATENCY::Wait1
    }
    #[doc = "Two Wait States (Vcore Boost 1 (&lt;= 102MHz), Vcore Normal 1 (&lt;= 90MHz), Vcore 2 (&lt;= 26MHz)"]
    #[inline(always)]
    pub fn is_wait2(&self) -> bool {
        *self == LATENCY::Wait2
    }
    #[doc = "Three Wait States (Vcore Boost 1 (&lt;= 136MHz), Vcore Normal 1 (&lt;= 120MHz)"]
    #[inline(always)]
    pub fn is_wait3(&self) -> bool {
        *self == LATENCY::Wait3
    }
    #[doc = "Four Wait States (Vcore Boost 1 (&lt;= 170MHz), Vcore Normal 1 (&lt;= 150MHz)"]
    #[inline(always)]
    pub fn is_wait4(&self) -> bool {
        *self == LATENCY::Wait4
    }
}
#[doc = "Field `LATENCY` writer - Latency"]
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 4, LATENCY>;
impl<'a, REG> LATENCY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero Wait States (Vcore Boost 1 (&lt;= 34MHz), Vcore Normal 1 (&lt;= 30MHz), Vcore 2 (&lt;= 12MHz)"]
    #[inline(always)]
    pub fn wait0(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Wait0)
    }
    #[doc = "One Wait State (Vcore Boost 1 (&lt;= 68MHz), Vcore Normal 1 (&lt;= 60MHz), Vcore 2 (&lt;= 24MHz)"]
    #[inline(always)]
    pub fn wait1(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Wait1)
    }
    #[doc = "Two Wait States (Vcore Boost 1 (&lt;= 102MHz), Vcore Normal 1 (&lt;= 90MHz), Vcore 2 (&lt;= 26MHz)"]
    #[inline(always)]
    pub fn wait2(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Wait2)
    }
    #[doc = "Three Wait States (Vcore Boost 1 (&lt;= 136MHz), Vcore Normal 1 (&lt;= 120MHz)"]
    #[inline(always)]
    pub fn wait3(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Wait3)
    }
    #[doc = "Four Wait States (Vcore Boost 1 (&lt;= 170MHz), Vcore Normal 1 (&lt;= 150MHz)"]
    #[inline(always)]
    pub fn wait4(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Wait4)
    }
}
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PRFTEN_R = crate::BitReader;
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEN` reader - Instruction cache enable"]
pub type ICEN_R = crate::BitReader;
#[doc = "Field `ICEN` writer - Instruction cache enable"]
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEN` reader - Data cache enable"]
pub type DCEN_R = crate::BitReader;
#[doc = "Field `DCEN` writer - Data cache enable"]
pub type DCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICRST` reader - Instruction cache reset"]
pub type ICRST_R = crate::BitReader;
#[doc = "Field `ICRST` writer - Instruction cache reset"]
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRST` reader - Data cache reset"]
pub type DCRST_R = crate::BitReader;
#[doc = "Field `DCRST` writer - Data cache reset"]
pub type DCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN_PD` reader - Flash Power-down mode during Low-power run mode"]
pub type RUN_PD_R = crate::BitReader;
#[doc = "Field `RUN_PD` writer - Flash Power-down mode during Low-power run mode"]
pub type RUN_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_PD` reader - Flash Power-down mode during Low-power sleep mode"]
pub type SLEEP_PD_R = crate::BitReader;
#[doc = "Field `SLEEP_PD` writer - Flash Power-down mode during Low-power sleep mode"]
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_SWEN` reader - Debug software enable"]
pub type DBG_SWEN_R = crate::BitReader;
#[doc = "Field `DBG_SWEN` writer - Debug software enable"]
pub type DBG_SWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    pub fn dcrst(&self) -> DCRST_R {
        DCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flash Power-down mode during Low-power run mode"]
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flash Power-down mode during Low-power sleep mode"]
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Debug software enable"]
    #[inline(always)]
    pub fn dbg_swen(&self) -> DBG_SWEN_R {
        DBG_SWEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<ACRrs> {
        LATENCY_W::new(self, 0)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> ICEN_W<ACRrs> {
        ICEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcen(&mut self) -> DCEN_W<ACRrs> {
        DCEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn icrst(&mut self) -> ICRST_W<ACRrs> {
        ICRST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn dcrst(&mut self) -> DCRST_W<ACRrs> {
        DCRST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Flash Power-down mode during Low-power run mode"]
    #[inline(always)]
    #[must_use]
    pub fn run_pd(&mut self) -> RUN_PD_W<ACRrs> {
        RUN_PD_W::new(self, 13)
    }
    #[doc = "Bit 14 - Flash Power-down mode during Low-power sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<ACRrs> {
        SLEEP_PD_W::new(self, 14)
    }
    #[doc = "Bit 18 - Debug software enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_swen(&mut self) -> DBG_SWEN_W<ACRrs> {
        DBG_SWEN_W::new(self, 18)
    }
}
#[doc = "Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for ACRrs {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0x0600"]
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0x0600;
}
