///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
/**Latency

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY {
    ///0: 0 wait states
    Ws0 = 0,
    ///1: 1 wait states
    Ws1 = 1,
    ///2: 2 wait states
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
impl crate::IsEnum for LATENCY {}
///Field `LATENCY` reader - Latency
pub type LATENCY_R = crate::FieldReader<LATENCY>;
impl LATENCY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LATENCY> {
        match self.bits {
            0 => Some(LATENCY::Ws0),
            1 => Some(LATENCY::Ws1),
            2 => Some(LATENCY::Ws2),
            _ => None,
        }
    }
    ///0 wait states
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY::Ws0
    }
    ///1 wait states
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY::Ws1
    }
    ///2 wait states
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY::Ws2
    }
}
///Field `LATENCY` writer - Latency
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LATENCY>;
impl<'a, REG> LATENCY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///0 wait states
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws0)
    }
    ///1 wait states
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws1)
    }
    ///2 wait states
    #[inline(always)]
    pub fn ws2(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws2)
    }
}
/**Prefetch enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN {
    ///0: Prefetch is disabled
    Disabled = 0,
    ///1: Prefetch is enabled
    Enabled = 1,
}
impl From<PRFTEN> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PRFTEN` reader - Prefetch enable
pub type PRFTEN_R = crate::BitReader<PRFTEN>;
impl PRFTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRFTEN {
        match self.bits {
            false => PRFTEN::Disabled,
            true => PRFTEN::Enabled,
        }
    }
    ///Prefetch is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN::Disabled
    }
    ///Prefetch is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN::Enabled
    }
}
///Field `PRFTEN` writer - Prefetch enable
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG, PRFTEN>;
impl<'a, REG> PRFTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prefetch is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Disabled)
    }
    ///Prefetch is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Enabled)
    }
}
/**Instruction cache enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICEN {
    ///0: Instruction cache is disabled
    Disabled = 0,
    ///1: Instruction cache is enabled
    Enabled = 1,
}
impl From<ICEN> for bool {
    #[inline(always)]
    fn from(variant: ICEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ICEN` reader - Instruction cache enable
pub type ICEN_R = crate::BitReader<ICEN>;
impl ICEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICEN {
        match self.bits {
            false => ICEN::Disabled,
            true => ICEN::Enabled,
        }
    }
    ///Instruction cache is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICEN::Disabled
    }
    ///Instruction cache is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICEN::Enabled
    }
}
///Field `ICEN` writer - Instruction cache enable
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG, ICEN>;
impl<'a, REG> ICEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Instruction cache is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::Disabled)
    }
    ///Instruction cache is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::Enabled)
    }
}
/**Data cache enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCEN {
    ///0: Data cache is disabled
    Disabled = 0,
    ///1: Data cache is enabled
    Enabled = 1,
}
impl From<DCEN> for bool {
    #[inline(always)]
    fn from(variant: DCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DCEN` reader - Data cache enable
pub type DCEN_R = crate::BitReader<DCEN>;
impl DCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCEN {
        match self.bits {
            false => DCEN::Disabled,
            true => DCEN::Enabled,
        }
    }
    ///Data cache is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCEN::Disabled
    }
    ///Data cache is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCEN::Enabled
    }
}
///Field `DCEN` writer - Data cache enable
pub type DCEN_W<'a, REG> = crate::BitWriter<'a, REG, DCEN>;
impl<'a, REG> DCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data cache is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN::Disabled)
    }
    ///Data cache is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN::Enabled)
    }
}
/**Instruction cache reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICRST {
    ///0: Instruction cache is not reset
    NotReset = 0,
    ///1: Instruction cache is reset
    Reset = 1,
}
impl From<ICRST> for bool {
    #[inline(always)]
    fn from(variant: ICRST) -> Self {
        variant as u8 != 0
    }
}
///Field `ICRST` reader - Instruction cache reset
pub type ICRST_R = crate::BitReader<ICRST>;
impl ICRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICRST {
        match self.bits {
            false => ICRST::NotReset,
            true => ICRST::Reset,
        }
    }
    ///Instruction cache is not reset
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == ICRST::NotReset
    }
    ///Instruction cache is reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ICRST::Reset
    }
}
///Field `ICRST` writer - Instruction cache reset
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG, ICRST>;
impl<'a, REG> ICRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Instruction cache is not reset
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::NotReset)
    }
    ///Instruction cache is reset
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::Reset)
    }
}
/**Data cache reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCRST {
    ///0: Data cache is not reset
    NotReset = 0,
    ///1: Data cache is reset
    Reset = 1,
}
impl From<DCRST> for bool {
    #[inline(always)]
    fn from(variant: DCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `DCRST` reader - Data cache reset
pub type DCRST_R = crate::BitReader<DCRST>;
impl DCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCRST {
        match self.bits {
            false => DCRST::NotReset,
            true => DCRST::Reset,
        }
    }
    ///Data cache is not reset
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == DCRST::NotReset
    }
    ///Data cache is reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCRST::Reset
    }
}
///Field `DCRST` writer - Data cache reset
pub type DCRST_W<'a, REG> = crate::BitWriter<'a, REG, DCRST>;
impl<'a, REG> DCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data cache is not reset
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCRST::NotReset)
    }
    ///Data cache is reset
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCRST::Reset)
    }
}
/**CPU1 programm erase suspend request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PES {
    ///0: Flash program and erase operations granted
    Granted = 0,
    ///1: Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_SR is set when PES bit in FLASH_ACR is set
    Suspended = 1,
}
impl From<PES> for bool {
    #[inline(always)]
    fn from(variant: PES) -> Self {
        variant as u8 != 0
    }
}
///Field `PES` reader - CPU1 programm erase suspend request
pub type PES_R = crate::BitReader<PES>;
impl PES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PES {
        match self.bits {
            false => PES::Granted,
            true => PES::Suspended,
        }
    }
    ///Flash program and erase operations granted
    #[inline(always)]
    pub fn is_granted(&self) -> bool {
        *self == PES::Granted
    }
    ///Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_SR is set when PES bit in FLASH_ACR is set
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == PES::Suspended
    }
}
///Field `PES` writer - CPU1 programm erase suspend request
pub type PES_W<'a, REG> = crate::BitWriter<'a, REG, PES>;
impl<'a, REG> PES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash program and erase operations granted
    #[inline(always)]
    pub fn granted(self) -> &'a mut crate::W<REG> {
        self.variant(PES::Granted)
    }
    ///Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_SR is set when PES bit in FLASH_ACR is set
    #[inline(always)]
    pub fn suspended(self) -> &'a mut crate::W<REG> {
        self.variant(PES::Suspended)
    }
}
/**Flash User area empty

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMPTY {
    ///0: User Flash programmend
    Programmed = 0,
    ///1: User Flash empty
    Empty = 1,
}
impl From<EMPTY> for bool {
    #[inline(always)]
    fn from(variant: EMPTY) -> Self {
        variant as u8 != 0
    }
}
///Field `EMPTY` reader - Flash User area empty
pub type EMPTY_R = crate::BitReader<EMPTY>;
impl EMPTY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EMPTY {
        match self.bits {
            false => EMPTY::Programmed,
            true => EMPTY::Empty,
        }
    }
    ///User Flash programmend
    #[inline(always)]
    pub fn is_programmed(&self) -> bool {
        *self == EMPTY::Programmed
    }
    ///User Flash empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == EMPTY::Empty
    }
}
///Field `EMPTY` writer - Flash User area empty
pub type EMPTY_W<'a, REG> = crate::BitWriter<'a, REG, EMPTY>;
impl<'a, REG> EMPTY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///User Flash programmend
    #[inline(always)]
    pub fn programmed(self) -> &'a mut crate::W<REG> {
        self.variant(EMPTY::Programmed)
    }
    ///User Flash empty
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(EMPTY::Empty)
    }
}
impl R {
    ///Bits 0:2 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
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
    ///Bit 15 - CPU1 programm erase suspend request
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Flash User area empty
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 16) & 1) != 0)
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
            .field("pes", &self.pes())
            .field("empty", &self.empty())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Latency
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
    ///Bit 15 - CPU1 programm erase suspend request
    #[inline(always)]
    pub fn pes(&mut self) -> PES_W<'_, ACRrs> {
        PES_W::new(self, 15)
    }
    ///Bit 16 - Flash User area empty
    #[inline(always)]
    pub fn empty(&mut self) -> EMPTY_W<'_, ACRrs> {
        EMPTY_W::new(self, 16)
    }
}
/**Access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:ACR)*/
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
///`reset()` method sets ACR to value 0x0600
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0x0600;
}
