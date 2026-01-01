///Register `C2APB1ENR2` reader
pub type R = crate::R<C2APB1ENR2rs>;
///Register `C2APB1ENR2` writer
pub type W = crate::W<C2APB1ENR2rs>;
/**CPU2 Low power UART 1 clocks enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1EN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<LPUART1EN> for bool {
    #[inline(always)]
    fn from(variant: LPUART1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `LPUART1EN` reader - CPU2 Low power UART 1 clocks enable
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
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPUART1EN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPUART1EN::Enabled
    }
}
///Field `LPUART1EN` writer - CPU2 Low power UART 1 clocks enable
pub type LPUART1EN_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1EN>;
impl<'a, REG> LPUART1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1EN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1EN::Enabled)
    }
}
///Field `LPTIM2EN` reader - CPU2 Low power timer 2 clocks enable
pub use LPUART1EN_R as LPTIM2EN_R;
///Field `LPTIM3EN` reader - CPU2 Low power timer 3 clocks enable
pub use LPUART1EN_R as LPTIM3EN_R;
///Field `LPTIM2EN` writer - CPU2 Low power timer 2 clocks enable
pub use LPUART1EN_W as LPTIM2EN_W;
///Field `LPTIM3EN` writer - CPU2 Low power timer 3 clocks enable
pub use LPUART1EN_W as LPTIM3EN_W;
impl R {
    ///Bit 0 - CPU2 Low power UART 1 clocks enable
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - CPU2 Low power timer 2 clocks enable
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU2 Low power timer 3 clocks enable
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2APB1ENR2")
            .field("lpuart1en", &self.lpuart1en())
            .field("lptim3en", &self.lptim3en())
            .field("lptim2en", &self.lptim2en())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU2 Low power UART 1 clocks enable
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<'_, C2APB1ENR2rs> {
        LPUART1EN_W::new(self, 0)
    }
    ///Bit 5 - CPU2 Low power timer 2 clocks enable
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<'_, C2APB1ENR2rs> {
        LPTIM2EN_W::new(self, 5)
    }
    ///Bit 6 - CPU2 Low power timer 3 clocks enable
    #[inline(always)]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<'_, C2APB1ENR2rs> {
        LPTIM3EN_W::new(self, 6)
    }
}
/**CPU2 APB1 peripheral clock enable register 2 \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2apb1enr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb1enr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#RCC:C2APB1ENR2)*/
pub struct C2APB1ENR2rs;
impl crate::RegisterSpec for C2APB1ENR2rs {
    type Ux = u32;
}
///`read()` method returns [`c2apb1enr2::R`](R) reader structure
impl crate::Readable for C2APB1ENR2rs {}
///`write(|w| ..)` method takes [`c2apb1enr2::W`](W) writer structure
impl crate::Writable for C2APB1ENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2APB1ENR2 to value 0
impl crate::Resettable for C2APB1ENR2rs {}
