///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
/**Latency

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY {
    ///0: Zero Wait States (Vcore Boost 1 (<= 34MHz), Vcore Normal 1 (<= 30MHz), Vcore 2 (<= 12MHz)
    Wait0 = 0,
    ///1: One Wait State (Vcore Boost 1 (<= 68MHz), Vcore Normal 1 (<= 60MHz), Vcore 2 (<= 24MHz)
    Wait1 = 1,
    ///2: Two Wait States (Vcore Boost 1 (<= 102MHz), Vcore Normal 1 (<= 90MHz), Vcore 2 (<= 26MHz)
    Wait2 = 2,
    ///3: Three Wait States (Vcore Boost 1 (<= 136MHz), Vcore Normal 1 (<= 120MHz)
    Wait3 = 3,
    ///4: Four Wait States (Vcore Boost 1 (<= 170MHz), Vcore Normal 1 (<= 150MHz)
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
impl crate::IsEnum for LATENCY {}
///Field `LATENCY` reader - Latency
pub type LATENCY_R = crate::FieldReader<LATENCY>;
impl LATENCY_R {
    ///Get enumerated values variant
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
    ///Zero Wait States (Vcore Boost 1 (<= 34MHz), Vcore Normal 1 (<= 30MHz), Vcore 2 (<= 12MHz)
    #[inline(always)]
    pub fn is_wait0(&self) -> bool {
        *self == LATENCY::Wait0
    }
    ///One Wait State (Vcore Boost 1 (<= 68MHz), Vcore Normal 1 (<= 60MHz), Vcore 2 (<= 24MHz)
    #[inline(always)]
    pub fn is_wait1(&self) -> bool {
        *self == LATENCY::Wait1
    }
    ///Two Wait States (Vcore Boost 1 (<= 102MHz), Vcore Normal 1 (<= 90MHz), Vcore 2 (<= 26MHz)
    #[inline(always)]
    pub fn is_wait2(&self) -> bool {
        *self == LATENCY::Wait2
    }
    ///Three Wait States (Vcore Boost 1 (<= 136MHz), Vcore Normal 1 (<= 120MHz)
    #[inline(always)]
    pub fn is_wait3(&self) -> bool {
        *self == LATENCY::Wait3
    }
    ///Four Wait States (Vcore Boost 1 (<= 170MHz), Vcore Normal 1 (<= 150MHz)
    #[inline(always)]
    pub fn is_wait4(&self) -> bool {
        *self == LATENCY::Wait4
    }
}
///Field `LATENCY` writer - Latency
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 4, LATENCY>;
impl<'a, REG> LATENCY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Zero Wait States (Vcore Boost 1 (<= 34MHz), Vcore Normal 1 (<= 30MHz), Vcore 2 (<= 12MHz)
    #[inline(always)]
    pub fn wait0(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Wait0)
    }
    ///One Wait State (Vcore Boost 1 (<= 68MHz), Vcore Normal 1 (<= 60MHz), Vcore 2 (<= 24MHz)
    #[inline(always)]
    pub fn wait1(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Wait1)
    }
    ///Two Wait States (Vcore Boost 1 (<= 102MHz), Vcore Normal 1 (<= 90MHz), Vcore 2 (<= 26MHz)
    #[inline(always)]
    pub fn wait2(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Wait2)
    }
    ///Three Wait States (Vcore Boost 1 (<= 136MHz), Vcore Normal 1 (<= 120MHz)
    #[inline(always)]
    pub fn wait3(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Wait3)
    }
    ///Four Wait States (Vcore Boost 1 (<= 170MHz), Vcore Normal 1 (<= 150MHz)
    #[inline(always)]
    pub fn wait4(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Wait4)
    }
}
///Field `PRFTEN` reader - Prefetch enable
pub type PRFTEN_R = crate::BitReader;
///Field `PRFTEN` writer - Prefetch enable
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICEN` reader - Instruction cache enable
pub type ICEN_R = crate::BitReader;
///Field `ICEN` writer - Instruction cache enable
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCEN` reader - Data cache enable
pub type DCEN_R = crate::BitReader;
///Field `DCEN` writer - Data cache enable
pub type DCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICRST` reader - Instruction cache reset
pub type ICRST_R = crate::BitReader;
///Field `ICRST` writer - Instruction cache reset
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCRST` reader - Data cache reset
pub type DCRST_R = crate::BitReader;
///Field `DCRST` writer - Data cache reset
pub type DCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RUN_PD` reader - Flash Power-down mode during Run or Low-power run mode
pub type RUN_PD_R = crate::BitReader;
///Field `RUN_PD` writer - Flash Power-down mode during Run or Low-power run mode
pub type RUN_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLEEP_PD` reader - Flash Power-down mode during Sleep or Low-power sleep mode
pub type SLEEP_PD_R = crate::BitReader;
///Field `SLEEP_PD` writer - Flash Power-down mode during Sleep or Low-power sleep mode
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_SWEN` reader - Debug software enable
pub type DBG_SWEN_R = crate::BitReader;
///Field `DBG_SWEN` writer - Debug software enable
pub type DBG_SWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Instruction cache enable
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data cache enable
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Instruction cache reset
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Data cache reset
    #[inline(always)]
    pub fn dcrst(&self) -> DCRST_R {
        DCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Flash Power-down mode during Run or Low-power run mode
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Flash Power-down mode during Sleep or Low-power sleep mode
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 18 - Debug software enable
    #[inline(always)]
    pub fn dbg_swen(&self) -> DBG_SWEN_R {
        DBG_SWEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("latency", &self.latency())
            .field("prften", &self.prften())
            .field("icen", &self.icen())
            .field("dcen", &self.dcen())
            .field("icrst", &self.icrst())
            .field("dcrst", &self.dcrst())
            .field("run_pd", &self.run_pd())
            .field("sleep_pd", &self.sleep_pd())
            .field("dbg_swen", &self.dbg_swen())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Latency
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<'_, ACRrs> {
        LATENCY_W::new(self, 0)
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W<'_, ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    ///Bit 9 - Instruction cache enable
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W<'_, ACRrs> {
        ICEN_W::new(self, 9)
    }
    ///Bit 10 - Data cache enable
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W<'_, ACRrs> {
        DCEN_W::new(self, 10)
    }
    ///Bit 11 - Instruction cache reset
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W<'_, ACRrs> {
        ICRST_W::new(self, 11)
    }
    ///Bit 12 - Data cache reset
    #[inline(always)]
    pub fn dcrst(&mut self) -> DCRST_W<'_, ACRrs> {
        DCRST_W::new(self, 12)
    }
    ///Bit 13 - Flash Power-down mode during Run or Low-power run mode
    #[inline(always)]
    pub fn run_pd(&mut self) -> RUN_PD_W<'_, ACRrs> {
        RUN_PD_W::new(self, 13)
    }
    ///Bit 14 - Flash Power-down mode during Sleep or Low-power sleep mode
    #[inline(always)]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<'_, ACRrs> {
        SLEEP_PD_W::new(self, 14)
    }
    ///Bit 18 - Debug software enable
    #[inline(always)]
    pub fn dbg_swen(&mut self) -> DBG_SWEN_W<'_, ACRrs> {
        DBG_SWEN_W::new(self, 18)
    }
}
/**Flash access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G471.html#FLASH:ACR)*/
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
///`read()` method returns [`acr::R`](R) reader structure
impl crate::Readable for ACRrs {}
///`write(|w| ..)` method takes [`acr::W`](W) writer structure
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACR to value 0x0004_0601
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0x0004_0601;
}
