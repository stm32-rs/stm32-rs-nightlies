///Register `SYSCFG_CFGR2` reader
pub type R = crate::R<SYSCFG_CFGR2rs>;
///Register `SYSCFG_CFGR2` writer
pub type W = crate::W<SYSCFG_CFGR2rs>;
/**Cortex<Superscript> <Default Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript> <Default Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKUP_LOCK {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<LOCKUP_LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCKUP_LOCK` reader - Cortex<Superscript> <Default Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript> <Default Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input.
pub type LOCKUP_LOCK_R = crate::BitReader<LOCKUP_LOCK>;
impl LOCKUP_LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCKUP_LOCK {
        match self.bits {
            false => LOCKUP_LOCK::B0x0,
            true => LOCKUP_LOCK::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LOCKUP_LOCK::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LOCKUP_LOCK::B0x1
    }
}
///Field `LOCKUP_LOCK` writer - Cortex<Superscript> <Default Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript> <Default Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input.
pub type LOCKUP_LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCKUP_LOCK>;
impl<'a, REG> LOCKUP_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUP_LOCK::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUP_LOCK::B0x1)
    }
}
impl R {
    ///Bit 0 - Cortex<Superscript> <Default Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript> <Default Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input.
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_CFGR2")
            .field("lockup_lock", &self.lockup_lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Cortex<Superscript> <Default Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript> <Default Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input.
    #[inline(always)]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W<'_, SYSCFG_CFGR2rs> {
        LOCKUP_LOCK_W::new(self, 0)
    }
}
/**SYSCFG configuration register 2

You can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#SYSCFG:SYSCFG_CFGR2)*/
pub struct SYSCFG_CFGR2rs;
impl crate::RegisterSpec for SYSCFG_CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_cfgr2::R`](R) reader structure
impl crate::Readable for SYSCFG_CFGR2rs {}
///`write(|w| ..)` method takes [`syscfg_cfgr2::W`](W) writer structure
impl crate::Writable for SYSCFG_CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYSCFG_CFGR2 to value 0
impl crate::Resettable for SYSCFG_CFGR2rs {}
