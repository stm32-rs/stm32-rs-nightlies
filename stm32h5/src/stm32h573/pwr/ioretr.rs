#[doc = "Register `IORETR` reader"]
pub type R = crate::R<IORETRrs>;
#[doc = "Register `IORETR` writer"]
pub type W = crate::W<IORETRrs>;
#[doc = "IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IORETEN {
    #[doc = "0: IO Retention mode is disabled"]
    Disabled = 0,
    #[doc = "1: IO Retention mode is enabled"]
    Enabled = 1,
}
impl From<IORETEN> for bool {
    #[inline(always)]
    fn from(variant: IORETEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IORETEN` reader - IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
pub type IORETEN_R = crate::BitReader<IORETEN>;
impl IORETEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IORETEN {
        match self.bits {
            false => IORETEN::Disabled,
            true => IORETEN::Enabled,
        }
    }
    #[doc = "IO Retention mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IORETEN::Disabled
    }
    #[doc = "IO Retention mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IORETEN::Enabled
    }
}
#[doc = "Field `IORETEN` writer - IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
pub type IORETEN_W<'a, REG> = crate::BitWriter<'a, REG, IORETEN>;
impl<'a, REG> IORETEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO Retention mode is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IORETEN::Disabled)
    }
    #[doc = "IO Retention mode is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IORETEN::Enabled)
    }
}
#[doc = "IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JTAGIORETEN {
    #[doc = "0: IO Retention mode is disabled"]
    Disabled = 0,
    #[doc = "1: IO Retention mode is enabled"]
    Enabled = 1,
}
impl From<JTAGIORETEN> for bool {
    #[inline(always)]
    fn from(variant: JTAGIORETEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JTAGIORETEN` reader - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode"]
pub type JTAGIORETEN_R = crate::BitReader<JTAGIORETEN>;
impl JTAGIORETEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JTAGIORETEN {
        match self.bits {
            false => JTAGIORETEN::Disabled,
            true => JTAGIORETEN::Enabled,
        }
    }
    #[doc = "IO Retention mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JTAGIORETEN::Disabled
    }
    #[doc = "IO Retention mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JTAGIORETEN::Enabled
    }
}
#[doc = "Field `JTAGIORETEN` writer - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode"]
pub type JTAGIORETEN_W<'a, REG> = crate::BitWriter<'a, REG, JTAGIORETEN>;
impl<'a, REG> JTAGIORETEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO Retention mode is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JTAGIORETEN::Disabled)
    }
    #[doc = "IO Retention mode is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JTAGIORETEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
    #[inline(always)]
    pub fn ioreten(&self) -> IORETEN_R {
        IORETEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode"]
    #[inline(always)]
    pub fn jtagioreten(&self) -> JTAGIORETEN_R {
        JTAGIORETEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
    #[inline(always)]
    #[must_use]
    pub fn ioreten(&mut self) -> IORETEN_W<IORETRrs> {
        IORETEN_W::new(self, 0)
    }
    #[doc = "Bit 16 - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode"]
    #[inline(always)]
    #[must_use]
    pub fn jtagioreten(&mut self) -> JTAGIORETEN_W<IORETRrs> {
        JTAGIORETEN_W::new(self, 16)
    }
}
#[doc = "PWR I/O retention register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioretr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioretr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IORETRrs;
impl crate::RegisterSpec for IORETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioretr::R`](R) reader structure"]
impl crate::Readable for IORETRrs {}
#[doc = "`write(|w| ..)` method takes [`ioretr::W`](W) writer structure"]
impl crate::Writable for IORETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IORETR to value 0"]
impl crate::Resettable for IORETRrs {
    const RESET_VALUE: u32 = 0;
}
