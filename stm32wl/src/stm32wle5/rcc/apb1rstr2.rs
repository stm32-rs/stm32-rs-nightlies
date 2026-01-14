///Register `APB1RSTR2` reader
pub type R = crate::R<APB1RSTR2rs>;
///Register `APB1RSTR2` writer
pub type W = crate::W<APB1RSTR2rs>;
/**Low-power UART 1 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1RST {
    ///0: No effect
    NoReset = 0,
    ///1: Reset peripheral
    Reset = 1,
}
impl From<LPUART1RST> for bool {
    #[inline(always)]
    fn from(variant: LPUART1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `LPUART1RST` reader - Low-power UART 1 reset
pub type LPUART1RST_R = crate::BitReader<LPUART1RST>;
impl LPUART1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1RST {
        match self.bits {
            false => LPUART1RST::NoReset,
            true => LPUART1RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == LPUART1RST::NoReset
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPUART1RST::Reset
    }
}
///Field `LPUART1RST` writer - Low-power UART 1 reset
pub type LPUART1RST_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1RST>;
impl<'a, REG> LPUART1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1RST::NoReset)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1RST::Reset)
    }
}
///Field `LPTIM2RST` reader - Low-power timer 2 reset
pub use LPUART1RST_R as LPTIM2RST_R;
///Field `LPTIM3RST` reader - Low-power timer 3 reset
pub use LPUART1RST_R as LPTIM3RST_R;
///Field `LPTIM2RST` writer - Low-power timer 2 reset
pub use LPUART1RST_W as LPTIM2RST_W;
///Field `LPTIM3RST` writer - Low-power timer 3 reset
pub use LPUART1RST_W as LPTIM3RST_W;
impl R {
    ///Bit 0 - Low-power UART 1 reset
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - Low-power timer 2 reset
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Low-power timer 3 reset
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RSTR2")
            .field("lpuart1rst", &self.lpuart1rst())
            .field("lptim3rst", &self.lptim3rst())
            .field("lptim2rst", &self.lptim2rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low-power UART 1 reset
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<'_, APB1RSTR2rs> {
        LPUART1RST_W::new(self, 0)
    }
    ///Bit 5 - Low-power timer 2 reset
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<'_, APB1RSTR2rs> {
        LPTIM2RST_W::new(self, 5)
    }
    ///Bit 6 - Low-power timer 3 reset
    #[inline(always)]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<'_, APB1RSTR2rs> {
        LPTIM3RST_W::new(self, 6)
    }
}
/**APB1 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apb1rstr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#RCC:APB1RSTR2)*/
pub struct APB1RSTR2rs;
impl crate::RegisterSpec for APB1RSTR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1rstr2::R`](R) reader structure
impl crate::Readable for APB1RSTR2rs {}
///`write(|w| ..)` method takes [`apb1rstr2::W`](W) writer structure
impl crate::Writable for APB1RSTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1RSTR2 to value 0
impl crate::Resettable for APB1RSTR2rs {}
