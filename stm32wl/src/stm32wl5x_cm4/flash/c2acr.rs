///Register `C2ACR` reader
pub type R = crate::R<C2ACRrs>;
///Register `C2ACR` writer
pub type W = crate::W<C2ACRrs>;
/**CPU2 Prefetch enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN {
    ///0: CPU2 prefetch is disabled
    Disabled = 0,
    ///1: CPU2 prefetch is enabled
    Enabled = 1,
}
impl From<PRFTEN> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PRFTEN` reader - CPU2 Prefetch enable
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
    ///CPU2 prefetch is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN::Disabled
    }
    ///CPU2 prefetch is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN::Enabled
    }
}
///Field `PRFTEN` writer - CPU2 Prefetch enable
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG, PRFTEN>;
impl<'a, REG> PRFTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CPU2 prefetch is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Disabled)
    }
    ///CPU2 prefetch is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Enabled)
    }
}
/**CPU2 Instruction cache enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICEN {
    ///0: CPU2 instruction cache is disabled
    Disabled = 0,
    ///1: CPU2 instruction cache is enabled
    Enabled = 1,
}
impl From<ICEN> for bool {
    #[inline(always)]
    fn from(variant: ICEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ICEN` reader - CPU2 Instruction cache enable
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
    ///CPU2 instruction cache is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICEN::Disabled
    }
    ///CPU2 instruction cache is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICEN::Enabled
    }
}
///Field `ICEN` writer - CPU2 Instruction cache enable
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG, ICEN>;
impl<'a, REG> ICEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CPU2 instruction cache is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::Disabled)
    }
    ///CPU2 instruction cache is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::Enabled)
    }
}
/**CPU2 Instruction cache reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICRST {
    ///0: CPU2 instruction cache is not reset
    NotReset = 0,
    ///1: CPU2 instruction cache is reset
    Reset = 1,
}
impl From<ICRST> for bool {
    #[inline(always)]
    fn from(variant: ICRST) -> Self {
        variant as u8 != 0
    }
}
///Field `ICRST` reader - CPU2 Instruction cache reset
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
    ///CPU2 instruction cache is not reset
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == ICRST::NotReset
    }
    ///CPU2 instruction cache is reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ICRST::Reset
    }
}
///Field `ICRST` writer - CPU2 Instruction cache reset
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG, ICRST>;
impl<'a, REG> ICRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CPU2 instruction cache is not reset
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::NotReset)
    }
    ///CPU2 instruction cache is reset
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::Reset)
    }
}
/**CPU2 program / erase suspend request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PES {
    ///0: Flash program and erase operations granted
    Granted = 0,
    ///1: Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_C2SR is set when PES bit in FLASH_C2ACR is set
    Suspended = 1,
}
impl From<PES> for bool {
    #[inline(always)]
    fn from(variant: PES) -> Self {
        variant as u8 != 0
    }
}
///Field `PES` reader - CPU2 program / erase suspend request
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
    ///Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_C2SR is set when PES bit in FLASH_C2ACR is set
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == PES::Suspended
    }
}
///Field `PES` writer - CPU2 program / erase suspend request
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
    ///Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_C2SR is set when PES bit in FLASH_C2ACR is set
    #[inline(always)]
    pub fn suspended(self) -> &'a mut crate::W<REG> {
        self.variant(PES::Suspended)
    }
}
impl R {
    ///Bit 8 - CPU2 Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU2 Instruction cache enable
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - CPU2 Instruction cache reset
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - CPU2 program / erase suspend request
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2ACR")
            .field("prften", &self.prften())
            .field("icen", &self.icen())
            .field("icrst", &self.icrst())
            .field("pes", &self.pes())
            .finish()
    }
}
impl W {
    ///Bit 8 - CPU2 Prefetch enable
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W<'_, C2ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    ///Bit 9 - CPU2 Instruction cache enable
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W<'_, C2ACRrs> {
        ICEN_W::new(self, 9)
    }
    ///Bit 11 - CPU2 Instruction cache reset
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W<'_, C2ACRrs> {
        ICRST_W::new(self, 11)
    }
    ///Bit 15 - CPU2 program / erase suspend request
    #[inline(always)]
    pub fn pes(&mut self) -> PES_W<'_, C2ACRrs> {
        PES_W::new(self, 15)
    }
}
/**Flash CPU2 access control register

You can [`read`](crate::Reg::read) this register and get [`c2acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:C2ACR)*/
pub struct C2ACRrs;
impl crate::RegisterSpec for C2ACRrs {
    type Ux = u32;
}
///`read()` method returns [`c2acr::R`](R) reader structure
impl crate::Readable for C2ACRrs {}
///`write(|w| ..)` method takes [`c2acr::W`](W) writer structure
impl crate::Writable for C2ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2ACR to value 0x0600
impl crate::Resettable for C2ACRrs {
    const RESET_VALUE: u32 = 0x0600;
}
