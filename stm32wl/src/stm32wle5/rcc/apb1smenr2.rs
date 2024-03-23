#[doc = "Register `APB1SMENR2` reader"]
pub type R = crate::R<APB1SMENR2rs>;
#[doc = "Register `APB1SMENR2` writer"]
pub type W = crate::W<APB1SMENR2rs>;
#[doc = "Low power UART 1 clock enable during CPU1 Csleep and CStop modes.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1SMEN {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<LPUART1SMEN> for bool {
    #[inline(always)]
    fn from(variant: LPUART1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1SMEN` reader - Low power UART 1 clock enable during CPU1 Csleep and CStop modes."]
pub type LPUART1SMEN_R = crate::BitReader<LPUART1SMEN>;
impl LPUART1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1SMEN {
        match self.bits {
            false => LPUART1SMEN::Disabled,
            true => LPUART1SMEN::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPUART1SMEN::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPUART1SMEN::Enabled
    }
}
#[doc = "Field `LPUART1SMEN` writer - Low power UART 1 clock enable during CPU1 Csleep and CStop modes."]
pub type LPUART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1SMEN>;
impl<'a, REG> LPUART1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SMEN::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SMEN::Enabled)
    }
}
#[doc = "Field `LPTIM2SMEN` reader - Low power timer 2 clock enable during CPU1 Csleep and CStop modes"]
pub use LPUART1SMEN_R as LPTIM2SMEN_R;
#[doc = "Field `LPTIM3SMEN` reader - Low power timer 3 clock enable during CPU1 Csleep and CStop modes"]
pub use LPUART1SMEN_R as LPTIM3SMEN_R;
#[doc = "Field `LPTIM2SMEN` writer - Low power timer 2 clock enable during CPU1 Csleep and CStop modes"]
pub use LPUART1SMEN_W as LPTIM2SMEN_W;
#[doc = "Field `LPTIM3SMEN` writer - Low power timer 3 clock enable during CPU1 Csleep and CStop modes"]
pub use LPUART1SMEN_W as LPTIM3SMEN_W;
impl R {
    #[doc = "Bit 0 - Low power UART 1 clock enable during CPU1 Csleep and CStop modes."]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Low power timer 2 clock enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low power timer 3 clock enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    pub fn lptim3smen(&self) -> LPTIM3SMEN_R {
        LPTIM3SMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clock enable during CPU1 Csleep and CStop modes."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<APB1SMENR2rs> {
        LPUART1SMEN_W::new(self, 0)
    }
    #[doc = "Bit 5 - Low power timer 2 clock enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<APB1SMENR2rs> {
        LPTIM2SMEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Low power timer 3 clock enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3smen(&mut self) -> LPTIM3SMEN_W<APB1SMENR2rs> {
        LPTIM3SMEN_W::new(self, 6)
    }
}
#[doc = "APB1 peripheral clocks enable in Sleep mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1smenr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1smenr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1SMENR2rs;
impl crate::RegisterSpec for APB1SMENR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1smenr2::R`](R) reader structure"]
impl crate::Readable for APB1SMENR2rs {}
#[doc = "`write(|w| ..)` method takes [`apb1smenr2::W`](W) writer structure"]
impl crate::Writable for APB1SMENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1SMENR2 to value 0x61"]
impl crate::Resettable for APB1SMENR2rs {
    const RESET_VALUE: u32 = 0x61;
}
