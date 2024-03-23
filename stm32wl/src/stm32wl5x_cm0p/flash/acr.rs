#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACRrs>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACRrs>;
#[doc = "Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY {
    #[doc = "0: 0 wait states"]
    Ws0 = 0,
    #[doc = "1: 1 wait states"]
    Ws1 = 1,
    #[doc = "2: 2 wait states"]
    Ws2 = 2,
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
            0 => Some(LATENCY::Ws0),
            1 => Some(LATENCY::Ws1),
            2 => Some(LATENCY::Ws2),
            _ => None,
        }
    }
    #[doc = "0 wait states"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY::Ws0
    }
    #[doc = "1 wait states"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY::Ws1
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY::Ws2
    }
}
#[doc = "Field `LATENCY` writer - Latency"]
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LATENCY>;
impl<'a, REG> LATENCY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 wait states"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws0)
    }
    #[doc = "1 wait states"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws1)
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws2)
    }
}
#[doc = "Prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN {
    #[doc = "0: Prefetch is disabled"]
    Disabled = 0,
    #[doc = "1: Prefetch is enabled"]
    Enabled = 1,
}
impl From<PRFTEN> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PRFTEN_R = crate::BitReader<PRFTEN>;
impl PRFTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRFTEN {
        match self.bits {
            false => PRFTEN::Disabled,
            true => PRFTEN::Enabled,
        }
    }
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN::Disabled
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN::Enabled
    }
}
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG, PRFTEN>;
impl<'a, REG> PRFTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Disabled)
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Enabled)
    }
}
#[doc = "Instruction cache enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICEN {
    #[doc = "0: Instruction cache is disabled"]
    Disabled = 0,
    #[doc = "1: Instruction cache is enabled"]
    Enabled = 1,
}
impl From<ICEN> for bool {
    #[inline(always)]
    fn from(variant: ICEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICEN` reader - Instruction cache enable"]
pub type ICEN_R = crate::BitReader<ICEN>;
impl ICEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICEN {
        match self.bits {
            false => ICEN::Disabled,
            true => ICEN::Enabled,
        }
    }
    #[doc = "Instruction cache is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICEN::Disabled
    }
    #[doc = "Instruction cache is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICEN::Enabled
    }
}
#[doc = "Field `ICEN` writer - Instruction cache enable"]
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG, ICEN>;
impl<'a, REG> ICEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Instruction cache is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::Disabled)
    }
    #[doc = "Instruction cache is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::Enabled)
    }
}
#[doc = "Data cache enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCEN {
    #[doc = "0: Data cache is disabled"]
    Disabled = 0,
    #[doc = "1: Data cache is enabled"]
    Enabled = 1,
}
impl From<DCEN> for bool {
    #[inline(always)]
    fn from(variant: DCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCEN` reader - Data cache enable"]
pub type DCEN_R = crate::BitReader<DCEN>;
impl DCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCEN {
        match self.bits {
            false => DCEN::Disabled,
            true => DCEN::Enabled,
        }
    }
    #[doc = "Data cache is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCEN::Disabled
    }
    #[doc = "Data cache is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCEN::Enabled
    }
}
#[doc = "Field `DCEN` writer - Data cache enable"]
pub type DCEN_W<'a, REG> = crate::BitWriter<'a, REG, DCEN>;
impl<'a, REG> DCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data cache is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN::Disabled)
    }
    #[doc = "Data cache is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN::Enabled)
    }
}
#[doc = "Instruction cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICRST {
    #[doc = "0: Instruction cache is not reset"]
    NotReset = 0,
    #[doc = "1: Instruction cache is reset"]
    Reset = 1,
}
impl From<ICRST> for bool {
    #[inline(always)]
    fn from(variant: ICRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICRST` reader - Instruction cache reset"]
pub type ICRST_R = crate::BitReader<ICRST>;
impl ICRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICRST {
        match self.bits {
            false => ICRST::NotReset,
            true => ICRST::Reset,
        }
    }
    #[doc = "Instruction cache is not reset"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == ICRST::NotReset
    }
    #[doc = "Instruction cache is reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ICRST::Reset
    }
}
#[doc = "Field `ICRST` writer - Instruction cache reset"]
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG, ICRST>;
impl<'a, REG> ICRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Instruction cache is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::NotReset)
    }
    #[doc = "Instruction cache is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::Reset)
    }
}
#[doc = "Data cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCRST {
    #[doc = "0: Data cache is not reset"]
    NotReset = 0,
    #[doc = "1: Data cache is reset"]
    Reset = 1,
}
impl From<DCRST> for bool {
    #[inline(always)]
    fn from(variant: DCRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCRST` reader - Data cache reset"]
pub type DCRST_R = crate::BitReader<DCRST>;
impl DCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCRST {
        match self.bits {
            false => DCRST::NotReset,
            true => DCRST::Reset,
        }
    }
    #[doc = "Data cache is not reset"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == DCRST::NotReset
    }
    #[doc = "Data cache is reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCRST::Reset
    }
}
#[doc = "Field `DCRST` writer - Data cache reset"]
pub type DCRST_W<'a, REG> = crate::BitWriter<'a, REG, DCRST>;
impl<'a, REG> DCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data cache is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCRST::NotReset)
    }
    #[doc = "Data cache is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCRST::Reset)
    }
}
#[doc = "CPU1 programm erase suspend request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PES {
    #[doc = "0: Flash program and erase operations granted"]
    Granted = 0,
    #[doc = "1: Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_SR is set when PES bit in FLASH_ACR is set"]
    Suspended = 1,
}
impl From<PES> for bool {
    #[inline(always)]
    fn from(variant: PES) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PES` reader - CPU1 programm erase suspend request"]
pub type PES_R = crate::BitReader<PES>;
impl PES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PES {
        match self.bits {
            false => PES::Granted,
            true => PES::Suspended,
        }
    }
    #[doc = "Flash program and erase operations granted"]
    #[inline(always)]
    pub fn is_granted(&self) -> bool {
        *self == PES::Granted
    }
    #[doc = "Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_SR is set when PES bit in FLASH_ACR is set"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == PES::Suspended
    }
}
#[doc = "Field `PES` writer - CPU1 programm erase suspend request"]
pub type PES_W<'a, REG> = crate::BitWriter<'a, REG, PES>;
impl<'a, REG> PES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash program and erase operations granted"]
    #[inline(always)]
    pub fn granted(self) -> &'a mut crate::W<REG> {
        self.variant(PES::Granted)
    }
    #[doc = "Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_SR is set when PES bit in FLASH_ACR is set"]
    #[inline(always)]
    pub fn suspended(self) -> &'a mut crate::W<REG> {
        self.variant(PES::Suspended)
    }
}
#[doc = "Flash User area empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMPTY {
    #[doc = "0: User Flash programmend"]
    Programmed = 0,
    #[doc = "1: User Flash empty"]
    Empty = 1,
}
impl From<EMPTY> for bool {
    #[inline(always)]
    fn from(variant: EMPTY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMPTY` reader - Flash User area empty"]
pub type EMPTY_R = crate::BitReader<EMPTY>;
impl EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EMPTY {
        match self.bits {
            false => EMPTY::Programmed,
            true => EMPTY::Empty,
        }
    }
    #[doc = "User Flash programmend"]
    #[inline(always)]
    pub fn is_programmed(&self) -> bool {
        *self == EMPTY::Programmed
    }
    #[doc = "User Flash empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == EMPTY::Empty
    }
}
#[doc = "Field `EMPTY` writer - Flash User area empty"]
pub type EMPTY_W<'a, REG> = crate::BitWriter<'a, REG, EMPTY>;
impl<'a, REG> EMPTY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "User Flash programmend"]
    #[inline(always)]
    pub fn programmed(self) -> &'a mut crate::W<REG> {
        self.variant(EMPTY::Programmed)
    }
    #[doc = "User Flash empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(EMPTY::Empty)
    }
}
impl R {
    #[doc = "Bits 0:2 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
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
    #[doc = "Bit 15 - CPU1 programm erase suspend request"]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Flash User area empty"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Latency"]
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
    #[doc = "Bit 15 - CPU1 programm erase suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn pes(&mut self) -> PES_W<ACRrs> {
        PES_W::new(self, 15)
    }
    #[doc = "Bit 16 - Flash User area empty"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EMPTY_W<ACRrs> {
        EMPTY_W::new(self, 16)
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
