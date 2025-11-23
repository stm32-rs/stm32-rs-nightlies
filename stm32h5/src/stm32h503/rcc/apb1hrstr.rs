///Register `APB1HRSTR` reader
pub type R = crate::R<APB1HRSTRrs>;
///Register `APB1HRSTR` writer
pub type W = crate::W<APB1HRSTRrs>;
/**DTS block reset Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTSRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<DTSRST> for bool {
    #[inline(always)]
    fn from(variant: DTSRST) -> Self {
        variant as u8 != 0
    }
}
///Field `DTSRST` reader - DTS block reset Set and reset by software.
pub type DTSRST_R = crate::BitReader<DTSRST>;
impl DTSRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DTSRST> {
        match self.bits {
            true => Some(DTSRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DTSRST::Reset
    }
}
///Field `DTSRST` writer - DTS block reset Set and reset by software.
pub type DTSRST_W<'a, REG> = crate::BitWriter<'a, REG, DTSRST>;
impl<'a, REG> DTSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DTSRST::Reset)
    }
}
///Field `LPTIM2RST` reader - LPTIM2 block reset Set and reset by software.
pub use DTSRST_R as LPTIM2RST_R;
///Field `FDCANRST` reader - FDCAN block reset
pub use DTSRST_R as FDCANRST_R;
///Field `LPTIM2RST` writer - LPTIM2 block reset Set and reset by software.
pub use DTSRST_W as LPTIM2RST_W;
///Field `FDCANRST` writer - FDCAN block reset
pub use DTSRST_W as FDCANRST_W;
impl R {
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
    ///Bit 9 - FDCAN block reset
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HRSTR")
            .field("dtsrst", &self.dtsrst())
            .field("lptim2rst", &self.lptim2rst())
            .field("fdcanrst", &self.fdcanrst())
            .finish()
    }
}
impl W {
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
    ///Bit 9 - FDCAN block reset
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<'_, APB1HRSTRrs> {
        FDCANRST_W::new(self, 9)
    }
}
/**RCC APB1 peripheral high reset register

You can [`read`](crate::Reg::read) this register and get [`apb1hrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RCC:APB1HRSTR)*/
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
