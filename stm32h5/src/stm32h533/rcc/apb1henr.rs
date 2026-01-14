///Register `APB1HENR` reader
pub type R = crate::R<APB1HENRrs>;
///Register `APB1HENR` writer
pub type W = crate::W<APB1HENRrs>;
/**UART9 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART9EN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<UART9EN> for bool {
    #[inline(always)]
    fn from(variant: UART9EN) -> Self {
        variant as u8 != 0
    }
}
///Field `UART9EN` reader - UART9 clock enable
pub type UART9EN_R = crate::BitReader<UART9EN>;
impl UART9EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UART9EN {
        match self.bits {
            false => UART9EN::Disabled,
            true => UART9EN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UART9EN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UART9EN::Enabled
    }
}
///Field `UART9EN` writer - UART9 clock enable
pub type UART9EN_W<'a, REG> = crate::BitWriter<'a, REG, UART9EN>;
impl<'a, REG> UART9EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART9EN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART9EN::Enabled)
    }
}
///Field `UART12EN` reader - UART12 clock enable
pub use UART9EN_R as UART12EN_R;
///Field `DTSEN` reader - DTS clock enable
pub use UART9EN_R as DTSEN_R;
///Field `LPTIM2EN` reader - LPTIM2 clock enable
pub use UART9EN_R as LPTIM2EN_R;
///Field `FDCANEN` reader - FDCAN1 and FDCAN2 peripheral clock enable
pub use UART9EN_R as FDCANEN_R;
///Field `UCPD1EN` reader - UCPD1 clock enable
pub use UART9EN_R as UCPD1EN_R;
///Field `UART12EN` writer - UART12 clock enable
pub use UART9EN_W as UART12EN_W;
///Field `DTSEN` writer - DTS clock enable
pub use UART9EN_W as DTSEN_W;
///Field `LPTIM2EN` writer - LPTIM2 clock enable
pub use UART9EN_W as LPTIM2EN_W;
///Field `FDCANEN` writer - FDCAN1 and FDCAN2 peripheral clock enable
pub use UART9EN_W as FDCANEN_W;
///Field `UCPD1EN` writer - UCPD1 clock enable
pub use UART9EN_W as UCPD1EN_W;
impl R {
    ///Bit 0 - UART9 clock enable
    #[inline(always)]
    pub fn uart9en(&self) -> UART9EN_R {
        UART9EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UART12 clock enable
    #[inline(always)]
    pub fn uart12en(&self) -> UART12EN_R {
        UART12EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - DTS clock enable
    #[inline(always)]
    pub fn dtsen(&self) -> DTSEN_R {
        DTSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - LPTIM2 clock enable
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - FDCAN1 and FDCAN2 peripheral clock enable
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 23 - UCPD1 clock enable
    #[inline(always)]
    pub fn ucpd1en(&self) -> UCPD1EN_R {
        UCPD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HENR")
            .field("uart9en", &self.uart9en())
            .field("uart12en", &self.uart12en())
            .field("dtsen", &self.dtsen())
            .field("lptim2en", &self.lptim2en())
            .field("fdcanen", &self.fdcanen())
            .field("ucpd1en", &self.ucpd1en())
            .finish()
    }
}
impl W {
    ///Bit 0 - UART9 clock enable
    #[inline(always)]
    pub fn uart9en(&mut self) -> UART9EN_W<'_, APB1HENRrs> {
        UART9EN_W::new(self, 0)
    }
    ///Bit 1 - UART12 clock enable
    #[inline(always)]
    pub fn uart12en(&mut self) -> UART12EN_W<'_, APB1HENRrs> {
        UART12EN_W::new(self, 1)
    }
    ///Bit 3 - DTS clock enable
    #[inline(always)]
    pub fn dtsen(&mut self) -> DTSEN_W<'_, APB1HENRrs> {
        DTSEN_W::new(self, 3)
    }
    ///Bit 5 - LPTIM2 clock enable
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<'_, APB1HENRrs> {
        LPTIM2EN_W::new(self, 5)
    }
    ///Bit 9 - FDCAN1 and FDCAN2 peripheral clock enable
    #[inline(always)]
    pub fn fdcanen(&mut self) -> FDCANEN_W<'_, APB1HENRrs> {
        FDCANEN_W::new(self, 9)
    }
    ///Bit 23 - UCPD1 clock enable
    #[inline(always)]
    pub fn ucpd1en(&mut self) -> UCPD1EN_W<'_, APB1HENRrs> {
        UCPD1EN_W::new(self, 23)
    }
}
/**RCC APB1 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`apb1henr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1henr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RCC:APB1HENR)*/
pub struct APB1HENRrs;
impl crate::RegisterSpec for APB1HENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1henr::R`](R) reader structure
impl crate::Readable for APB1HENRrs {}
///`write(|w| ..)` method takes [`apb1henr::W`](W) writer structure
impl crate::Writable for APB1HENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HENR to value 0
impl crate::Resettable for APB1HENRrs {}
