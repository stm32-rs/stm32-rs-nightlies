///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
/**Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY {
    ///0: Zero wait states
    B0x0 = 0,
    ///1: One wait state
    B0x1 = 1,
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
///Field `LATENCY` reader - Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read.
pub type LATENCY_R = crate::FieldReader<LATENCY>;
impl LATENCY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LATENCY> {
        match self.bits {
            0 => Some(LATENCY::B0x0),
            1 => Some(LATENCY::B0x1),
            _ => None,
        }
    }
    ///Zero wait states
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LATENCY::B0x0
    }
    ///One wait state
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LATENCY::B0x1
    }
}
///Field `LATENCY` writer - Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read.
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LATENCY>;
impl<'a, REG> LATENCY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Zero wait states
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::B0x0)
    }
    ///One wait state
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::B0x1)
    }
}
/**CPU Prefetch enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN {
    ///0: CPU Prefetch disabled
    B0x0 = 0,
    ///1: CPU Prefetch enabled
    B0x1 = 1,
}
impl From<PRFTEN> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PRFTEN` reader - CPU Prefetch enable
pub type PRFTEN_R = crate::BitReader<PRFTEN>;
impl PRFTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRFTEN {
        match self.bits {
            false => PRFTEN::B0x0,
            true => PRFTEN::B0x1,
        }
    }
    ///CPU Prefetch disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PRFTEN::B0x0
    }
    ///CPU Prefetch enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PRFTEN::B0x1
    }
}
///Field `PRFTEN` writer - CPU Prefetch enable
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG, PRFTEN>;
impl<'a, REG> PRFTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CPU Prefetch disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::B0x0)
    }
    ///CPU Prefetch enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::B0x1)
    }
}
/**CPU Instruction cache enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICEN {
    ///0: CPU Instruction cache is disabled
    B0x0 = 0,
    ///1: CPU Instruction cache is enabled
    B0x1 = 1,
}
impl From<ICEN> for bool {
    #[inline(always)]
    fn from(variant: ICEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ICEN` reader - CPU Instruction cache enable
pub type ICEN_R = crate::BitReader<ICEN>;
impl ICEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICEN {
        match self.bits {
            false => ICEN::B0x0,
            true => ICEN::B0x1,
        }
    }
    ///CPU Instruction cache is disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ICEN::B0x0
    }
    ///CPU Instruction cache is enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ICEN::B0x1
    }
}
///Field `ICEN` writer - CPU Instruction cache enable
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG, ICEN>;
impl<'a, REG> ICEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CPU Instruction cache is disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::B0x0)
    }
    ///CPU Instruction cache is enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::B0x1)
    }
}
/**CPU Instruction cache reset This bit can be written only when the instruction cache is disabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICRST {
    ///0: CPU Instruction cache is not reset
    B0x0 = 0,
    ///1: CPU Instruction cache is reset
    B0x1 = 1,
}
impl From<ICRST> for bool {
    #[inline(always)]
    fn from(variant: ICRST) -> Self {
        variant as u8 != 0
    }
}
///Field `ICRST` reader - CPU Instruction cache reset This bit can be written only when the instruction cache is disabled.
pub type ICRST_R = crate::BitReader<ICRST>;
impl ICRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICRST {
        match self.bits {
            false => ICRST::B0x0,
            true => ICRST::B0x1,
        }
    }
    ///CPU Instruction cache is not reset
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ICRST::B0x0
    }
    ///CPU Instruction cache is reset
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ICRST::B0x1
    }
}
///Field `ICRST` writer - CPU Instruction cache reset This bit can be written only when the instruction cache is disabled.
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG, ICRST>;
impl<'a, REG> ICRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CPU Instruction cache is not reset
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::B0x0)
    }
    ///CPU Instruction cache is reset
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::B0x1)
    }
}
/**Main flash memory area empty This bit indicates whether the first location of the Main flash memory area was read as erased or as programmed during OBL. It is not affected by the system reset. Software may need to change this bit value after a flash memory program or erase operation. The bit can be set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMPTY {
    ///0: Main flash memory area programmed
    B0x0 = 0,
    ///1: Main flash memory area empty
    B0x1 = 1,
}
impl From<EMPTY> for bool {
    #[inline(always)]
    fn from(variant: EMPTY) -> Self {
        variant as u8 != 0
    }
}
///Field `EMPTY` reader - Main flash memory area empty This bit indicates whether the first location of the Main flash memory area was read as erased or as programmed during OBL. It is not affected by the system reset. Software may need to change this bit value after a flash memory program or erase operation. The bit can be set and reset by software.
pub type EMPTY_R = crate::BitReader<EMPTY>;
impl EMPTY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EMPTY {
        match self.bits {
            false => EMPTY::B0x0,
            true => EMPTY::B0x1,
        }
    }
    ///Main flash memory area programmed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EMPTY::B0x0
    }
    ///Main flash memory area empty
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EMPTY::B0x1
    }
}
///Field `EMPTY` writer - Main flash memory area empty This bit indicates whether the first location of the Main flash memory area was read as erased or as programmed during OBL. It is not affected by the system reset. Software may need to change this bit value after a flash memory program or erase operation. The bit can be set and reset by software.
pub type EMPTY_W<'a, REG> = crate::BitWriter<'a, REG, EMPTY>;
impl<'a, REG> EMPTY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Main flash memory area programmed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EMPTY::B0x0)
    }
    ///Main flash memory area empty
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EMPTY::B0x1)
    }
}
/**Debug access software enable Software may use this bit to enable/disable the debugger read access.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_SWEN {
    ///0: Debugger disabled
    B0x0 = 0,
    ///1: Debugger enabled
    B0x1 = 1,
}
impl From<DBG_SWEN> for bool {
    #[inline(always)]
    fn from(variant: DBG_SWEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_SWEN` reader - Debug access software enable Software may use this bit to enable/disable the debugger read access.
pub type DBG_SWEN_R = crate::BitReader<DBG_SWEN>;
impl DBG_SWEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_SWEN {
        match self.bits {
            false => DBG_SWEN::B0x0,
            true => DBG_SWEN::B0x1,
        }
    }
    ///Debugger disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_SWEN::B0x0
    }
    ///Debugger enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_SWEN::B0x1
    }
}
///Field `DBG_SWEN` writer - Debug access software enable Software may use this bit to enable/disable the debugger read access.
pub type DBG_SWEN_W<'a, REG> = crate::BitWriter<'a, REG, DBG_SWEN>;
impl<'a, REG> DBG_SWEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Debugger disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_SWEN::B0x0)
    }
    ///Debugger enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_SWEN::B0x1)
    }
}
impl R {
    ///Bits 0:2 - Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read.
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - CPU Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU Instruction cache enable
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - CPU Instruction cache reset This bit can be written only when the instruction cache is disabled.
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - Main flash memory area empty This bit indicates whether the first location of the Main flash memory area was read as erased or as programmed during OBL. It is not affected by the system reset. Software may need to change this bit value after a flash memory program or erase operation. The bit can be set and reset by software.
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Debug access software enable Software may use this bit to enable/disable the debugger read access.
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
            .field("icrst", &self.icrst())
            .field("empty", &self.empty())
            .field("dbg_swen", &self.dbg_swen())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read.
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<'_, ACRrs> {
        LATENCY_W::new(self, 0)
    }
    ///Bit 8 - CPU Prefetch enable
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W<'_, ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    ///Bit 9 - CPU Instruction cache enable
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W<'_, ACRrs> {
        ICEN_W::new(self, 9)
    }
    ///Bit 11 - CPU Instruction cache reset This bit can be written only when the instruction cache is disabled.
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W<'_, ACRrs> {
        ICRST_W::new(self, 11)
    }
    ///Bit 16 - Main flash memory area empty This bit indicates whether the first location of the Main flash memory area was read as erased or as programmed during OBL. It is not affected by the system reset. Software may need to change this bit value after a flash memory program or erase operation. The bit can be set and reset by software.
    #[inline(always)]
    pub fn empty(&mut self) -> EMPTY_W<'_, ACRrs> {
        EMPTY_W::new(self, 16)
    }
    ///Bit 18 - Debug access software enable Software may use this bit to enable/disable the debugger read access.
    #[inline(always)]
    pub fn dbg_swen(&mut self) -> DBG_SWEN_W<'_, ACRrs> {
        DBG_SWEN_W::new(self, 18)
    }
}
/**FLASH access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#FLASH:ACR)*/
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
///`reset()` method sets ACR to value 0x0004_0600
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0x0004_0600;
}
