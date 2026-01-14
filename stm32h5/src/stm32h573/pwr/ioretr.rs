///Register `IORETR` reader
pub type R = crate::R<IORETRrs>;
///Register `IORETR` writer
pub type W = crate::W<IORETRrs>;
/**IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IORETEN {
    ///0: IO Retention mode is disabled
    Disabled = 0,
    ///1: IO Retention mode is enabled
    Enabled = 1,
}
impl From<IORETEN> for bool {
    #[inline(always)]
    fn from(variant: IORETEN) -> Self {
        variant as u8 != 0
    }
}
///Field `IORETEN` reader - IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register.
pub type IORETEN_R = crate::BitReader<IORETEN>;
impl IORETEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IORETEN {
        match self.bits {
            false => IORETEN::Disabled,
            true => IORETEN::Enabled,
        }
    }
    ///IO Retention mode is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IORETEN::Disabled
    }
    ///IO Retention mode is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IORETEN::Enabled
    }
}
///Field `IORETEN` writer - IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register.
pub type IORETEN_W<'a, REG> = crate::BitWriter<'a, REG, IORETEN>;
impl<'a, REG> IORETEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IO Retention mode is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IORETEN::Disabled)
    }
    ///IO Retention mode is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IORETEN::Enabled)
    }
}
/**IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JTAGIORETEN {
    ///0: IO Retention mode is disabled
    Disabled = 0,
    ///1: IO Retention mode is enabled
    Enabled = 1,
}
impl From<JTAGIORETEN> for bool {
    #[inline(always)]
    fn from(variant: JTAGIORETEN) -> Self {
        variant as u8 != 0
    }
}
///Field `JTAGIORETEN` reader - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode
pub type JTAGIORETEN_R = crate::BitReader<JTAGIORETEN>;
impl JTAGIORETEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JTAGIORETEN {
        match self.bits {
            false => JTAGIORETEN::Disabled,
            true => JTAGIORETEN::Enabled,
        }
    }
    ///IO Retention mode is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JTAGIORETEN::Disabled
    }
    ///IO Retention mode is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JTAGIORETEN::Enabled
    }
}
///Field `JTAGIORETEN` writer - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode
pub type JTAGIORETEN_W<'a, REG> = crate::BitWriter<'a, REG, JTAGIORETEN>;
impl<'a, REG> JTAGIORETEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IO Retention mode is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JTAGIORETEN::Disabled)
    }
    ///IO Retention mode is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JTAGIORETEN::Enabled)
    }
}
impl R {
    ///Bit 0 - IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register.
    #[inline(always)]
    pub fn ioreten(&self) -> IORETEN_R {
        IORETEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode
    #[inline(always)]
    pub fn jtagioreten(&self) -> JTAGIORETEN_R {
        JTAGIORETEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IORETR")
            .field("ioreten", &self.ioreten())
            .field("jtagioreten", &self.jtagioreten())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register.
    #[inline(always)]
    pub fn ioreten(&mut self) -> IORETEN_W<'_, IORETRrs> {
        IORETEN_W::new(self, 0)
    }
    ///Bit 16 - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode
    #[inline(always)]
    pub fn jtagioreten(&mut self) -> JTAGIORETEN_W<'_, IORETRrs> {
        JTAGIORETEN_W::new(self, 16)
    }
}
/**PWR I/O retention register

You can [`read`](crate::Reg::read) this register and get [`ioretr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#PWR:IORETR)*/
pub struct IORETRrs;
impl crate::RegisterSpec for IORETRrs {
    type Ux = u32;
}
///`read()` method returns [`ioretr::R`](R) reader structure
impl crate::Readable for IORETRrs {}
///`write(|w| ..)` method takes [`ioretr::W`](W) writer structure
impl crate::Writable for IORETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IORETR to value 0
impl crate::Resettable for IORETRrs {}
