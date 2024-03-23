#[doc = "Register `C2APB1ENR2` reader"]
pub type R = crate::R<C2APB1ENR2rs>;
#[doc = "Register `C2APB1ENR2` writer"]
pub type W = crate::W<C2APB1ENR2rs>;
#[doc = "CPU2 Low power UART 1 clocks enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1EN {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<LPUART1EN> for bool {
    #[inline(always)]
    fn from(variant: LPUART1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1EN` reader - CPU2 Low power UART 1 clocks enable"]
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
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPUART1EN::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPUART1EN::Enabled
    }
}
#[doc = "Field `LPUART1EN` writer - CPU2 Low power UART 1 clocks enable"]
pub type LPUART1EN_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1EN>;
impl<'a, REG> LPUART1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1EN::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1EN::Enabled)
    }
}
#[doc = "Field `LPTIM2EN` reader - CPU2 Low power timer 2 clocks enable"]
pub use LPUART1EN_R as LPTIM2EN_R;
#[doc = "Field `LPTIM3EN` reader - CPU2 Low power timer 3 clocks enable"]
pub use LPUART1EN_R as LPTIM3EN_R;
#[doc = "Field `LPTIM2EN` writer - CPU2 Low power timer 2 clocks enable"]
pub use LPUART1EN_W as LPTIM2EN_W;
#[doc = "Field `LPTIM3EN` writer - CPU2 Low power timer 3 clocks enable"]
pub use LPUART1EN_W as LPTIM3EN_W;
impl R {
    #[doc = "Bit 0 - CPU2 Low power UART 1 clocks enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - CPU2 Low power timer 2 clocks enable"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU2 Low power timer 3 clocks enable"]
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 Low power UART 1 clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<C2APB1ENR2rs> {
        LPUART1EN_W::new(self, 0)
    }
    #[doc = "Bit 5 - CPU2 Low power timer 2 clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<C2APB1ENR2rs> {
        LPTIM2EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU2 Low power timer 3 clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<C2APB1ENR2rs> {
        LPTIM3EN_W::new(self, 6)
    }
}
#[doc = "CPU2 APB1 peripheral clock enable register 2 \\[dual core device only\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2apb1enr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2apb1enr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2APB1ENR2rs;
impl crate::RegisterSpec for C2APB1ENR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2apb1enr2::R`](R) reader structure"]
impl crate::Readable for C2APB1ENR2rs {}
#[doc = "`write(|w| ..)` method takes [`c2apb1enr2::W`](W) writer structure"]
impl crate::Writable for C2APB1ENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2APB1ENR2 to value 0"]
impl crate::Resettable for C2APB1ENR2rs {
    const RESET_VALUE: u32 = 0;
}
