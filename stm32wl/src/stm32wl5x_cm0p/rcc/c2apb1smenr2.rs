///Register `C2APB1SMENR2` reader
pub type R = crate::R<C2APB1SMENR2rs>;
///Register `C2APB1SMENR2` writer
pub type W = crate::W<C2APB1SMENR2rs>;
/**Low power UART 1 clock enable during CPU2 CSleep and CStop mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1SMEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<LPUART1SMEN> for bool {
    #[inline(always)]
    fn from(variant: LPUART1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LPUART1SMEN` reader - Low power UART 1 clock enable during CPU2 CSleep and CStop mode
pub type LPUART1SMEN_R = crate::BitReader<LPUART1SMEN>;
impl LPUART1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1SMEN {
        match self.bits {
            false => LPUART1SMEN::Disabled,
            true => LPUART1SMEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPUART1SMEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPUART1SMEN::Enabled
    }
}
///Field `LPUART1SMEN` writer - Low power UART 1 clock enable during CPU2 CSleep and CStop mode
pub type LPUART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1SMEN>;
impl<'a, REG> LPUART1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SMEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SMEN::Enabled)
    }
}
///Field `LPTIM2SMEN` reader - Low power timer 2 clocks enable during CPU2 CSleep and CStop modes.
pub use LPUART1SMEN_R as LPTIM2SMEN_R;
///Field `LPTIM3SMEN` reader - Low power timer 3 clocks enable during CPU2 CSleep and CStop modes.
pub use LPUART1SMEN_R as LPTIM3SMEN_R;
///Field `LPTIM2SMEN` writer - Low power timer 2 clocks enable during CPU2 CSleep and CStop modes.
pub use LPUART1SMEN_W as LPTIM2SMEN_W;
///Field `LPTIM3SMEN` writer - Low power timer 3 clocks enable during CPU2 CSleep and CStop modes.
pub use LPUART1SMEN_W as LPTIM3SMEN_W;
impl R {
    ///Bit 0 - Low power UART 1 clock enable during CPU2 CSleep and CStop mode
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - Low power timer 2 clocks enable during CPU2 CSleep and CStop modes.
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Low power timer 3 clocks enable during CPU2 CSleep and CStop modes.
    #[inline(always)]
    pub fn lptim3smen(&self) -> LPTIM3SMEN_R {
        LPTIM3SMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2APB1SMENR2")
            .field("lpuart1smen", &self.lpuart1smen())
            .field("lptim3smen", &self.lptim3smen())
            .field("lptim2smen", &self.lptim2smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low power UART 1 clock enable during CPU2 CSleep and CStop mode
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<'_, C2APB1SMENR2rs> {
        LPUART1SMEN_W::new(self, 0)
    }
    ///Bit 5 - Low power timer 2 clocks enable during CPU2 CSleep and CStop modes.
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<'_, C2APB1SMENR2rs> {
        LPTIM2SMEN_W::new(self, 5)
    }
    ///Bit 6 - Low power timer 3 clocks enable during CPU2 CSleep and CStop modes.
    #[inline(always)]
    pub fn lptim3smen(&mut self) -> LPTIM3SMEN_W<'_, C2APB1SMENR2rs> {
        LPTIM3SMEN_W::new(self, 6)
    }
}
/**CPU2 APB1 peripheral clocks enable in Sleep mode register 2 \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2apb1smenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb1smenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2APB1SMENR2)*/
pub struct C2APB1SMENR2rs;
impl crate::RegisterSpec for C2APB1SMENR2rs {
    type Ux = u32;
}
///`read()` method returns [`c2apb1smenr2::R`](R) reader structure
impl crate::Readable for C2APB1SMENR2rs {}
///`write(|w| ..)` method takes [`c2apb1smenr2::W`](W) writer structure
impl crate::Writable for C2APB1SMENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2APB1SMENR2 to value 0x61
impl crate::Resettable for C2APB1SMENR2rs {
    const RESET_VALUE: u32 = 0x61;
}
