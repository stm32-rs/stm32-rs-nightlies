#[doc = "Register `APB1ENR2` reader"]
pub type R = crate::R<APB1ENR2rs>;
#[doc = "Register `APB1ENR2` writer"]
pub type W = crate::W<APB1ENR2rs>;
#[doc = "Low power UART 1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1EN {
    #[doc = "0: LPUART1 clock disabled"]
    Disabled = 0,
    #[doc = "1: LPUART1 clock enabled"]
    Enabled = 1,
}
impl From<LPUART1EN> for bool {
    #[inline(always)]
    fn from(variant: LPUART1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1EN` reader - Low power UART 1 clock enable"]
pub type LPUART1EN_R = crate::BitReader<LPUART1EN>;
impl LPUART1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1EN {
        match self.bits {
            false => LPUART1EN::Disabled,
            true => LPUART1EN::Enabled,
        }
    }
    #[doc = "LPUART1 clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPUART1EN::Disabled
    }
    #[doc = "LPUART1 clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPUART1EN::Enabled
    }
}
#[doc = "Field `LPUART1EN` writer - Low power UART 1 clock enable"]
pub type LPUART1EN_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1EN>;
impl<'a, REG> LPUART1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPUART1 clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1EN::Disabled)
    }
    #[doc = "LPUART1 clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1EN::Enabled)
    }
}
#[doc = "Field `I2C4EN` reader - I2C4 clock enable"]
pub type I2C4EN_R = crate::BitReader;
#[doc = "Field `I2C4EN` writer - I2C4 clock enable"]
pub type I2C4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPMI1EN` reader - Single wire protocol clock enable"]
pub type SWPMI1EN_R = crate::BitReader;
#[doc = "Field `SWPMI1EN` writer - Single wire protocol clock enable"]
pub type SWPMI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LPTIM2EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM2EN {
    #[doc = "0: LPTIM2 clock disabled"]
    Disabled = 0,
    #[doc = "1: LPTIM2 clock enabled"]
    Enabled = 1,
}
impl From<LPTIM2EN> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM2EN` reader - LPTIM2EN"]
pub type LPTIM2EN_R = crate::BitReader<LPTIM2EN>;
impl LPTIM2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM2EN {
        match self.bits {
            false => LPTIM2EN::Disabled,
            true => LPTIM2EN::Enabled,
        }
    }
    #[doc = "LPTIM2 clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPTIM2EN::Disabled
    }
    #[doc = "LPTIM2 clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPTIM2EN::Enabled
    }
}
#[doc = "Field `LPTIM2EN` writer - LPTIM2EN"]
pub type LPTIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM2EN>;
impl<'a, REG> LPTIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM2 clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2EN::Disabled)
    }
    #[doc = "LPTIM2 clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2EN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Low power UART 1 clock enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 clock enable"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single wire protocol clock enable"]
    #[inline(always)]
    pub fn swpmi1en(&self) -> SWPMI1EN_R {
        SWPMI1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<APB1ENR2rs> {
        LPUART1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4en(&mut self) -> I2C4EN_W<APB1ENR2rs> {
        I2C4EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Single wire protocol clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn swpmi1en(&mut self) -> SWPMI1EN_W<APB1ENR2rs> {
        SWPMI1EN_W::new(self, 2)
    }
    #[doc = "Bit 5 - LPTIM2EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<APB1ENR2rs> {
        LPTIM2EN_W::new(self, 5)
    }
}
#[doc = "APB1 peripheral clock enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1enr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1enr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1ENR2rs;
impl crate::RegisterSpec for APB1ENR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1enr2::R`](R) reader structure"]
impl crate::Readable for APB1ENR2rs {}
#[doc = "`write(|w| ..)` method takes [`apb1enr2::W`](W) writer structure"]
impl crate::Writable for APB1ENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1ENR2 to value 0"]
impl crate::Resettable for APB1ENR2rs {
    const RESET_VALUE: u32 = 0;
}
