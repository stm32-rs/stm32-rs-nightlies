///Register `APB1HRSTR` reader
pub type R = crate::R<APB1HRSTRrs>;
///Register `APB1HRSTR` writer
pub type W = crate::W<APB1HRSTRrs>;
/**UART9 block reset Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART9RST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<UART9RST> for bool {
    #[inline(always)]
    fn from(variant: UART9RST) -> Self {
        variant as u8 != 0
    }
}
///Field `UART9RST` reader - UART9 block reset Set and reset by software.
pub type UART9RST_R = crate::BitReader<UART9RST>;
impl UART9RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<UART9RST> {
        match self.bits {
            true => Some(UART9RST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == UART9RST::Reset
    }
}
///Field `UART9RST` writer - UART9 block reset Set and reset by software.
pub type UART9RST_W<'a, REG> = crate::BitWriter<'a, REG, UART9RST>;
impl<'a, REG> UART9RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(UART9RST::Reset)
    }
}
///Field `UART12RST` reader - UART12 block reset Set and reset by software.
pub use UART9RST_R as UART12RST_R;
///Field `DTSRST` reader - DTS block reset Set and reset by software.
pub use UART9RST_R as DTSRST_R;
///Field `LPTIM2RST` reader - LPTIM2 block reset Set and reset by software.
pub use UART9RST_R as LPTIM2RST_R;
///Field `FDCANRST` reader - FDCAN1 and FDCAN2 blocks reset
pub use UART9RST_R as FDCANRST_R;
///Field `UCPD1RST` reader - UCPD1 block reset
pub use UART9RST_R as UCPD1RST_R;
///Field `UART12RST` writer - UART12 block reset Set and reset by software.
pub use UART9RST_W as UART12RST_W;
///Field `DTSRST` writer - DTS block reset Set and reset by software.
pub use UART9RST_W as DTSRST_W;
///Field `LPTIM2RST` writer - LPTIM2 block reset Set and reset by software.
pub use UART9RST_W as LPTIM2RST_W;
///Field `FDCANRST` writer - FDCAN1 and FDCAN2 blocks reset
pub use UART9RST_W as FDCANRST_W;
///Field `UCPD1RST` writer - UCPD1 block reset
pub use UART9RST_W as UCPD1RST_W;
impl R {
    ///Bit 0 - UART9 block reset Set and reset by software.
    #[inline(always)]
    pub fn uart9rst(&self) -> UART9RST_R {
        UART9RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UART12 block reset Set and reset by software.
    #[inline(always)]
    pub fn uart12rst(&self) -> UART12RST_R {
        UART12RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - DTS block reset Set and reset by software.
    #[inline(always)]
    pub fn dtsrst(&self) -> DTSRST_R {
        DTSRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - LPTIM2 block reset Set and reset by software.
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - FDCAN1 and FDCAN2 blocks reset
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 23 - UCPD1 block reset
    #[inline(always)]
    pub fn ucpd1rst(&self) -> UCPD1RST_R {
        UCPD1RST_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HRSTR")
            .field("uart9rst", &self.uart9rst())
            .field("uart12rst", &self.uart12rst())
            .field("dtsrst", &self.dtsrst())
            .field("lptim2rst", &self.lptim2rst())
            .field("fdcanrst", &self.fdcanrst())
            .field("ucpd1rst", &self.ucpd1rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - UART9 block reset Set and reset by software.
    #[inline(always)]
    pub fn uart9rst(&mut self) -> UART9RST_W<'_, APB1HRSTRrs> {
        UART9RST_W::new(self, 0)
    }
    ///Bit 1 - UART12 block reset Set and reset by software.
    #[inline(always)]
    pub fn uart12rst(&mut self) -> UART12RST_W<'_, APB1HRSTRrs> {
        UART12RST_W::new(self, 1)
    }
    ///Bit 3 - DTS block reset Set and reset by software.
    #[inline(always)]
    pub fn dtsrst(&mut self) -> DTSRST_W<'_, APB1HRSTRrs> {
        DTSRST_W::new(self, 3)
    }
    ///Bit 5 - LPTIM2 block reset Set and reset by software.
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<'_, APB1HRSTRrs> {
        LPTIM2RST_W::new(self, 5)
    }
    ///Bit 9 - FDCAN1 and FDCAN2 blocks reset
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<'_, APB1HRSTRrs> {
        FDCANRST_W::new(self, 9)
    }
    ///Bit 23 - UCPD1 block reset
    #[inline(always)]
    pub fn ucpd1rst(&mut self) -> UCPD1RST_W<'_, APB1HRSTRrs> {
        UCPD1RST_W::new(self, 23)
    }
}
/**RCC APB1 peripheral high reset register

You can [`read`](crate::Reg::read) this register and get [`apb1hrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#RCC:APB1HRSTR)*/
pub struct APB1HRSTRrs;
impl crate::RegisterSpec for APB1HRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1hrstr::R`](R) reader structure
impl crate::Readable for APB1HRSTRrs {}
///`write(|w| ..)` method takes [`apb1hrstr::W`](W) writer structure
impl crate::Writable for APB1HRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HRSTR to value 0
impl crate::Resettable for APB1HRSTRrs {}
