///Register `APB1ENR2` reader
pub type R = crate::R<APB1ENR2rs>;
///Register `APB1ENR2` writer
pub type W = crate::W<APB1ENR2rs>;
/**Low power UART 1 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1EN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
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
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPUART1EN::Disabled
    }
    ///The selected clock is enabled
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
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1EN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1EN::Enabled)
    }
}
///Field `I2C4EN` reader - I2C4 clock enable
pub use LPUART1EN_R as I2C4EN_R;
///Field `LPTIM2EN` reader - LPTIM2EN
pub use LPUART1EN_R as LPTIM2EN_R;
///Field `LPTIM3EN` reader - LPTIM3EN
pub use LPUART1EN_R as LPTIM3EN_R;
///Field `FDCAN1EN` reader - FDCAN1EN
pub use LPUART1EN_R as FDCAN1EN_R;
///Field `USBFSEN` reader - USBFSEN
pub use LPUART1EN_R as USBFSEN_R;
///Field `UCPD1EN` reader - UCPD1EN
pub use LPUART1EN_R as UCPD1EN_R;
///Field `I2C4EN` writer - I2C4 clock enable
pub use LPUART1EN_W as I2C4EN_W;
///Field `LPTIM2EN` writer - LPTIM2EN
pub use LPUART1EN_W as LPTIM2EN_W;
///Field `LPTIM3EN` writer - LPTIM3EN
pub use LPUART1EN_W as LPTIM3EN_W;
///Field `FDCAN1EN` writer - FDCAN1EN
pub use LPUART1EN_W as FDCAN1EN_W;
///Field `USBFSEN` writer - USBFSEN
pub use LPUART1EN_W as USBFSEN_W;
///Field `UCPD1EN` writer - UCPD1EN
pub use LPUART1EN_W as UCPD1EN_W;
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
    ///Bit 5 - LPTIM2EN
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPTIM3EN
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - FDCAN1EN
    #[inline(always)]
    pub fn fdcan1en(&self) -> FDCAN1EN_R {
        FDCAN1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 21 - USBFSEN
    #[inline(always)]
    pub fn usbfsen(&self) -> USBFSEN_R {
        USBFSEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - UCPD1EN
    #[inline(always)]
    pub fn ucpd1en(&self) -> UCPD1EN_R {
        UCPD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR2")
            .field("lpuart1en", &self.lpuart1en())
            .field("i2c4en", &self.i2c4en())
            .field("lptim2en", &self.lptim2en())
            .field("lptim3en", &self.lptim3en())
            .field("fdcan1en", &self.fdcan1en())
            .field("usbfsen", &self.usbfsen())
            .field("ucpd1en", &self.ucpd1en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low power UART 1 clock enable
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<'_, APB1ENR2rs> {
        LPUART1EN_W::new(self, 0)
    }
    ///Bit 1 - I2C4 clock enable
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W<'_, APB1ENR2rs> {
        I2C4EN_W::new(self, 1)
    }
    ///Bit 5 - LPTIM2EN
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<'_, APB1ENR2rs> {
        LPTIM2EN_W::new(self, 5)
    }
    ///Bit 6 - LPTIM3EN
    #[inline(always)]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<'_, APB1ENR2rs> {
        LPTIM3EN_W::new(self, 6)
    }
    ///Bit 9 - FDCAN1EN
    #[inline(always)]
    pub fn fdcan1en(&mut self) -> FDCAN1EN_W<'_, APB1ENR2rs> {
        FDCAN1EN_W::new(self, 9)
    }
    ///Bit 21 - USBFSEN
    #[inline(always)]
    pub fn usbfsen(&mut self) -> USBFSEN_W<'_, APB1ENR2rs> {
        USBFSEN_W::new(self, 21)
    }
    ///Bit 23 - UCPD1EN
    #[inline(always)]
    pub fn ucpd1en(&mut self) -> UCPD1EN_W<'_, APB1ENR2rs> {
        UCPD1EN_W::new(self, 23)
    }
}
/**APB1 peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1enr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB1ENR2)*/
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
