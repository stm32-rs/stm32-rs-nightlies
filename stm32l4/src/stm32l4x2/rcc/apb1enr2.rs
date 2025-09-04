///Register `APB1ENR2` reader
pub type R = crate::R<APB1ENR2rs>;
///Register `APB1ENR2` writer
pub type W = crate::W<APB1ENR2rs>;
/**Low power UART 1 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1EN {
    ///0: LPUART1 clock disabled
    Disabled = 0,
    ///1: LPUART1 clock enabled
    Enabled = 1,
}
impl From<LPUART1EN> for bool {
    #[inline(always)]
    fn from(variant: LPUART1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `LPUART1EN` reader - Low power UART 1 clock enable
pub type LPUART1EN_R = crate::BitReader<LPUART1EN>;
impl LPUART1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1EN {
        match self.bits {
            false => LPUART1EN::Disabled,
            true => LPUART1EN::Enabled,
        }
    }
    ///LPUART1 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPUART1EN::Disabled
    }
    ///LPUART1 clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPUART1EN::Enabled
    }
}
///Field `LPUART1EN` writer - Low power UART 1 clock enable
pub type LPUART1EN_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1EN>;
impl<'a, REG> LPUART1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPUART1 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1EN::Disabled)
    }
    ///LPUART1 clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1EN::Enabled)
    }
}
///Field `I2C4EN` reader - I2C4 clock enable
pub type I2C4EN_R = crate::BitReader;
///Field `I2C4EN` writer - I2C4 clock enable
pub type I2C4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWPMI1EN` reader - Single wire protocol clock enable
pub type SWPMI1EN_R = crate::BitReader;
///Field `SWPMI1EN` writer - Single wire protocol clock enable
pub type SWPMI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**LPTIM2EN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM2EN {
    ///0: LPTIM2 clock disabled
    Disabled = 0,
    ///1: LPTIM2 clock enabled
    Enabled = 1,
}
impl From<LPTIM2EN> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `LPTIM2EN` reader - LPTIM2EN
pub type LPTIM2EN_R = crate::BitReader<LPTIM2EN>;
impl LPTIM2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM2EN {
        match self.bits {
            false => LPTIM2EN::Disabled,
            true => LPTIM2EN::Enabled,
        }
    }
    ///LPTIM2 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPTIM2EN::Disabled
    }
    ///LPTIM2 clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPTIM2EN::Enabled
    }
}
///Field `LPTIM2EN` writer - LPTIM2EN
pub type LPTIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM2EN>;
impl<'a, REG> LPTIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIM2 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2EN::Disabled)
    }
    ///LPTIM2 clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2EN::Enabled)
    }
}
///Field `DFSDMEN` reader - DFSDMEN enable
pub type DFSDMEN_R = crate::BitReader;
///Field `DFSDMEN` writer - DFSDMEN enable
pub type DFSDMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Low power UART 1 clock enable
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C4 clock enable
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Single wire protocol clock enable
    #[inline(always)]
    pub fn swpmi1en(&self) -> SWPMI1EN_R {
        SWPMI1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - LPTIM2EN
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 24 - DFSDMEN enable
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR2")
            .field("lptim2en", &self.lptim2en())
            .field("swpmi1en", &self.swpmi1en())
            .field("lpuart1en", &self.lpuart1en())
            .field("dfsdmen", &self.dfsdmen())
            .field("i2c4en", &self.i2c4en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low power UART 1 clock enable
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<APB1ENR2rs> {
        LPUART1EN_W::new(self, 0)
    }
    ///Bit 1 - I2C4 clock enable
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W<APB1ENR2rs> {
        I2C4EN_W::new(self, 1)
    }
    ///Bit 2 - Single wire protocol clock enable
    #[inline(always)]
    pub fn swpmi1en(&mut self) -> SWPMI1EN_W<APB1ENR2rs> {
        SWPMI1EN_W::new(self, 2)
    }
    ///Bit 5 - LPTIM2EN
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<APB1ENR2rs> {
        LPTIM2EN_W::new(self, 5)
    }
    ///Bit 24 - DFSDMEN enable
    #[inline(always)]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W<APB1ENR2rs> {
        DFSDMEN_W::new(self, 24)
    }
}
/**APB1 peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1enr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#RCC:APB1ENR2)*/
pub struct APB1ENR2rs;
impl crate::RegisterSpec for APB1ENR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1enr2::R`](R) reader structure
impl crate::Readable for APB1ENR2rs {}
///`write(|w| ..)` method takes [`apb1enr2::W`](W) writer structure
impl crate::Writable for APB1ENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1ENR2 to value 0
impl crate::Resettable for APB1ENR2rs {}
