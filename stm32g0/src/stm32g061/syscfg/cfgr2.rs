///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**Cortex-M0+ LOCKUP bit enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKUP_LOCK {
    ///0: error not connected to timers
    Disabled = 0,
    ///1: error triggers TIM1/15/16/17 break input
    Enabled = 1,
}
impl From<LOCKUP_LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCKUP_LOCK` reader - Cortex-M0+ LOCKUP bit enable bit
pub type LOCKUP_LOCK_R = crate::BitReader<LOCKUP_LOCK>;
impl LOCKUP_LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCKUP_LOCK {
        match self.bits {
            false => LOCKUP_LOCK::Disabled,
            true => LOCKUP_LOCK::Enabled,
        }
    }
    ///error not connected to timers
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOCKUP_LOCK::Disabled
    }
    ///error triggers TIM1/15/16/17 break input
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOCKUP_LOCK::Enabled
    }
}
///Field `LOCKUP_LOCK` writer - Cortex-M0+ LOCKUP bit enable bit
pub type LOCKUP_LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCKUP_LOCK>;
impl<'a, REG> LOCKUP_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///error not connected to timers
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUP_LOCK::Disabled)
    }
    ///error triggers TIM1/15/16/17 break input
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUP_LOCK::Enabled)
    }
}
///Field `SRAM_PARITY_LOCK` reader - SRAM parity lock bit
pub use LOCKUP_LOCK_R as SRAM_PARITY_LOCK_R;
///Field `ECC_LOCK` reader - ECC error lock bit
pub use LOCKUP_LOCK_R as ECC_LOCK_R;
///Field `SRAM_PARITY_LOCK` writer - SRAM parity lock bit
pub use LOCKUP_LOCK_W as SRAM_PARITY_LOCK_W;
///Field `ECC_LOCK` writer - ECC error lock bit
pub use LOCKUP_LOCK_W as ECC_LOCK_W;
/**SRAM parity error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_PEF {
    ///0: No SRAM parity error detected
    Normal = 0,
    ///1: SRAM parity error detected
    Error = 1,
}
impl From<SRAM_PEF> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PEF) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM_PEF` reader - SRAM parity error flag
pub type SRAM_PEF_R = crate::BitReader<SRAM_PEF>;
impl SRAM_PEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM_PEF {
        match self.bits {
            false => SRAM_PEF::Normal,
            true => SRAM_PEF::Error,
        }
    }
    ///No SRAM parity error detected
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SRAM_PEF::Normal
    }
    ///SRAM parity error detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SRAM_PEF::Error
    }
}
///Field `SRAM_PEF` writer - SRAM parity error flag
pub type SRAM_PEF_W<'a, REG> = crate::BitWriter<'a, REG, SRAM_PEF>;
impl<'a, REG> SRAM_PEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No SRAM parity error detected
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM_PEF::Normal)
    }
    ///SRAM parity error detected
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM_PEF::Error)
    }
}
impl R {
    ///Bit 0 - Cortex-M0+ LOCKUP bit enable bit
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM parity lock bit
    #[inline(always)]
    pub fn sram_parity_lock(&self) -> SRAM_PARITY_LOCK_R {
        SRAM_PARITY_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - ECC error lock bit
    #[inline(always)]
    pub fn ecc_lock(&self) -> ECC_LOCK_R {
        ECC_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - SRAM parity error flag
    #[inline(always)]
    pub fn sram_pef(&self) -> SRAM_PEF_R {
        SRAM_PEF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("lockup_lock", &self.lockup_lock())
            .field("sram_parity_lock", &self.sram_parity_lock())
            .field("ecc_lock", &self.ecc_lock())
            .field("sram_pef", &self.sram_pef())
            .finish()
    }
}
impl W {
    ///Bit 0 - Cortex-M0+ LOCKUP bit enable bit
    #[inline(always)]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W<'_, CFGR2rs> {
        LOCKUP_LOCK_W::new(self, 0)
    }
    ///Bit 1 - SRAM parity lock bit
    #[inline(always)]
    pub fn sram_parity_lock(&mut self) -> SRAM_PARITY_LOCK_W<'_, CFGR2rs> {
        SRAM_PARITY_LOCK_W::new(self, 1)
    }
    ///Bit 3 - ECC error lock bit
    #[inline(always)]
    pub fn ecc_lock(&mut self) -> ECC_LOCK_W<'_, CFGR2rs> {
        ECC_LOCK_W::new(self, 3)
    }
    ///Bit 8 - SRAM parity error flag
    #[inline(always)]
    pub fn sram_pef(&mut self) -> SRAM_PEF_W<'_, CFGR2rs> {
        SRAM_PEF_W::new(self, 8)
    }
}
/**SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#SYSCFG:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
