///Register `APB4RSTR` reader
pub type R = crate::R<APB4RSTRrs>;
///Register `APB4RSTR` writer
pub type W = crate::W<APB4RSTRrs>;
/**SBS block reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<SBSRST> for bool {
    #[inline(always)]
    fn from(variant: SBSRST) -> Self {
        variant as u8 != 0
    }
}
///Field `SBSRST` reader - SBS block reset
pub type SBSRST_R = crate::BitReader<SBSRST>;
impl SBSRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SBSRST> {
        match self.bits {
            true => Some(SBSRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SBSRST::Reset
    }
}
///Field `SBSRST` writer - SBS block reset
pub type SBSRST_W<'a, REG> = crate::BitWriter<'a, REG, SBSRST>;
impl<'a, REG> SBSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SBSRST::Reset)
    }
}
///Field `LPUART1RST` reader - LPUART1 block reset
pub use SBSRST_R as LPUART1RST_R;
///Field `SPI6RST` reader - SPI/I2S6 block reset
pub use SBSRST_R as SPI6RST_R;
///Field `LPTIM2RST` reader - LPTIM2 block reset
pub use SBSRST_R as LPTIM2RST_R;
///Field `LPTIM3RST` reader - LPTIM3 block reset
pub use SBSRST_R as LPTIM3RST_R;
///Field `LPTIM4RST` reader - LPTIM4 block reset
pub use SBSRST_R as LPTIM4RST_R;
///Field `LPTIM5RST` reader - LPTIM5 block reset
pub use SBSRST_R as LPTIM5RST_R;
///Field `VREFRST` reader - VREF block reset
pub use SBSRST_R as VREFRST_R;
///Field `DTSRST` reader - DTS block reset
pub use SBSRST_R as DTSRST_R;
///Field `LPUART1RST` writer - LPUART1 block reset
pub use SBSRST_W as LPUART1RST_W;
///Field `SPI6RST` writer - SPI/I2S6 block reset
pub use SBSRST_W as SPI6RST_W;
///Field `LPTIM2RST` writer - LPTIM2 block reset
pub use SBSRST_W as LPTIM2RST_W;
///Field `LPTIM3RST` writer - LPTIM3 block reset
pub use SBSRST_W as LPTIM3RST_W;
///Field `LPTIM4RST` writer - LPTIM4 block reset
pub use SBSRST_W as LPTIM4RST_W;
///Field `LPTIM5RST` writer - LPTIM5 block reset
pub use SBSRST_W as LPTIM5RST_W;
///Field `VREFRST` writer - VREF block reset
pub use SBSRST_W as VREFRST_W;
///Field `DTSRST` writer - DTS block reset
pub use SBSRST_W as DTSRST_W;
impl R {
    ///Bit 1 - SBS block reset
    #[inline(always)]
    pub fn sbsrst(&self) -> SBSRST_R {
        SBSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - LPUART1 block reset
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - SPI/I2S6 block reset
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 block reset
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM3 block reset
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LPTIM4 block reset
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM5 block reset
    #[inline(always)]
    pub fn lptim5rst(&self) -> LPTIM5RST_R {
        LPTIM5RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - VREF block reset
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 26 - DTS block reset
    #[inline(always)]
    pub fn dtsrst(&self) -> DTSRST_R {
        DTSRST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4RSTR")
            .field("sbsrst", &self.sbsrst())
            .field("lpuart1rst", &self.lpuart1rst())
            .field("spi6rst", &self.spi6rst())
            .field("lptim2rst", &self.lptim2rst())
            .field("lptim3rst", &self.lptim3rst())
            .field("lptim4rst", &self.lptim4rst())
            .field("lptim5rst", &self.lptim5rst())
            .field("vrefrst", &self.vrefrst())
            .field("dtsrst", &self.dtsrst())
            .finish()
    }
}
impl W {
    ///Bit 1 - SBS block reset
    #[inline(always)]
    pub fn sbsrst(&mut self) -> SBSRST_W<'_, APB4RSTRrs> {
        SBSRST_W::new(self, 1)
    }
    ///Bit 3 - LPUART1 block reset
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<'_, APB4RSTRrs> {
        LPUART1RST_W::new(self, 3)
    }
    ///Bit 5 - SPI/I2S6 block reset
    #[inline(always)]
    pub fn spi6rst(&mut self) -> SPI6RST_W<'_, APB4RSTRrs> {
        SPI6RST_W::new(self, 5)
    }
    ///Bit 9 - LPTIM2 block reset
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<'_, APB4RSTRrs> {
        LPTIM2RST_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 block reset
    #[inline(always)]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<'_, APB4RSTRrs> {
        LPTIM3RST_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 block reset
    #[inline(always)]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<'_, APB4RSTRrs> {
        LPTIM4RST_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 block reset
    #[inline(always)]
    pub fn lptim5rst(&mut self) -> LPTIM5RST_W<'_, APB4RSTRrs> {
        LPTIM5RST_W::new(self, 12)
    }
    ///Bit 15 - VREF block reset
    #[inline(always)]
    pub fn vrefrst(&mut self) -> VREFRST_W<'_, APB4RSTRrs> {
        VREFRST_W::new(self, 15)
    }
    ///Bit 26 - DTS block reset
    #[inline(always)]
    pub fn dtsrst(&mut self) -> DTSRST_W<'_, APB4RSTRrs> {
        DTSRST_W::new(self, 26)
    }
}
/**RCC APB4 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb4rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB4RSTR)*/
pub struct APB4RSTRrs;
impl crate::RegisterSpec for APB4RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb4rstr::R`](R) reader structure
impl crate::Readable for APB4RSTRrs {}
///`write(|w| ..)` method takes [`apb4rstr::W`](W) writer structure
impl crate::Writable for APB4RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4RSTR to value 0
impl crate::Resettable for APB4RSTRrs {}
