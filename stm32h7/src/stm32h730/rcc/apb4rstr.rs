///Register `APB4RSTR` reader
pub type R = crate::R<APB4RSTRrs>;
///Register `APB4RSTR` writer
pub type W = crate::W<APB4RSTRrs>;
/**SYSCFG block reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<SYSCFGRST> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGRST` reader - SYSCFG block reset
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRST>;
impl SYSCFGRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCFGRST> {
        match self.bits {
            true => Some(SYSCFGRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRST::Reset
    }
}
///Field `SYSCFGRST` writer - SYSCFG block reset
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGRST>;
impl<'a, REG> SYSCFGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGRST::Reset)
    }
}
///Field `LPUART1RST` reader - LPUART1 block reset
pub use SYSCFGRST_R as LPUART1RST_R;
///Field `SPI6RST` reader - SPI6 block reset
pub use SYSCFGRST_R as SPI6RST_R;
///Field `I2C4RST` reader - I2C4 block reset
pub use SYSCFGRST_R as I2C4RST_R;
///Field `LPTIM2RST` reader - LPTIM2 block reset
pub use SYSCFGRST_R as LPTIM2RST_R;
///Field `LPTIM3RST` reader - LPTIM3 block reset
pub use SYSCFGRST_R as LPTIM3RST_R;
///Field `LPTIM4RST` reader - LPTIM4 block reset
pub use SYSCFGRST_R as LPTIM4RST_R;
///Field `LPTIM5RST` reader - LPTIM5 block reset
pub use SYSCFGRST_R as LPTIM5RST_R;
///Field `COMP12RST` reader - COMP12 Blocks Reset
pub use SYSCFGRST_R as COMP12RST_R;
///Field `VREFRST` reader - VREF block reset
pub use SYSCFGRST_R as VREFRST_R;
///Field `SAI4RST` reader - SAI4 block reset
pub use SYSCFGRST_R as SAI4RST_R;
///Field `DTSRST` reader - Digital temperature sensor block reset Set and reset by software.
pub use SYSCFGRST_R as DTSRST_R;
///Field `LPUART1RST` writer - LPUART1 block reset
pub use SYSCFGRST_W as LPUART1RST_W;
///Field `SPI6RST` writer - SPI6 block reset
pub use SYSCFGRST_W as SPI6RST_W;
///Field `I2C4RST` writer - I2C4 block reset
pub use SYSCFGRST_W as I2C4RST_W;
///Field `LPTIM2RST` writer - LPTIM2 block reset
pub use SYSCFGRST_W as LPTIM2RST_W;
///Field `LPTIM3RST` writer - LPTIM3 block reset
pub use SYSCFGRST_W as LPTIM3RST_W;
///Field `LPTIM4RST` writer - LPTIM4 block reset
pub use SYSCFGRST_W as LPTIM4RST_W;
///Field `LPTIM5RST` writer - LPTIM5 block reset
pub use SYSCFGRST_W as LPTIM5RST_W;
///Field `COMP12RST` writer - COMP12 Blocks Reset
pub use SYSCFGRST_W as COMP12RST_W;
///Field `VREFRST` writer - VREF block reset
pub use SYSCFGRST_W as VREFRST_W;
///Field `SAI4RST` writer - SAI4 block reset
pub use SYSCFGRST_W as SAI4RST_W;
///Field `DTSRST` writer - Digital temperature sensor block reset Set and reset by software.
pub use SYSCFGRST_W as DTSRST_W;
impl R {
    ///Bit 1 - SYSCFG block reset
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - LPUART1 block reset
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - SPI6 block reset
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - I2C4 block reset
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 7) & 1) != 0)
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
    ///Bit 14 - COMP12 Blocks Reset
    #[inline(always)]
    pub fn comp12rst(&self) -> COMP12RST_R {
        COMP12RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - VREF block reset
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 21 - SAI4 block reset
    #[inline(always)]
    pub fn sai4rst(&self) -> SAI4RST_R {
        SAI4RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 26 - Digital temperature sensor block reset Set and reset by software.
    #[inline(always)]
    pub fn dtsrst(&self) -> DTSRST_R {
        DTSRST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4RSTR")
            .field("syscfgrst", &self.syscfgrst())
            .field("lpuart1rst", &self.lpuart1rst())
            .field("spi6rst", &self.spi6rst())
            .field("i2c4rst", &self.i2c4rst())
            .field("lptim2rst", &self.lptim2rst())
            .field("lptim3rst", &self.lptim3rst())
            .field("lptim4rst", &self.lptim4rst())
            .field("lptim5rst", &self.lptim5rst())
            .field("comp12rst", &self.comp12rst())
            .field("vrefrst", &self.vrefrst())
            .field("sai4rst", &self.sai4rst())
            .field("dtsrst", &self.dtsrst())
            .finish()
    }
}
impl W {
    ///Bit 1 - SYSCFG block reset
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, APB4RSTRrs> {
        SYSCFGRST_W::new(self, 1)
    }
    ///Bit 3 - LPUART1 block reset
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<'_, APB4RSTRrs> {
        LPUART1RST_W::new(self, 3)
    }
    ///Bit 5 - SPI6 block reset
    #[inline(always)]
    pub fn spi6rst(&mut self) -> SPI6RST_W<'_, APB4RSTRrs> {
        SPI6RST_W::new(self, 5)
    }
    ///Bit 7 - I2C4 block reset
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<'_, APB4RSTRrs> {
        I2C4RST_W::new(self, 7)
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
    ///Bit 14 - COMP12 Blocks Reset
    #[inline(always)]
    pub fn comp12rst(&mut self) -> COMP12RST_W<'_, APB4RSTRrs> {
        COMP12RST_W::new(self, 14)
    }
    ///Bit 15 - VREF block reset
    #[inline(always)]
    pub fn vrefrst(&mut self) -> VREFRST_W<'_, APB4RSTRrs> {
        VREFRST_W::new(self, 15)
    }
    ///Bit 21 - SAI4 block reset
    #[inline(always)]
    pub fn sai4rst(&mut self) -> SAI4RST_W<'_, APB4RSTRrs> {
        SAI4RST_W::new(self, 21)
    }
    ///Bit 26 - Digital temperature sensor block reset Set and reset by software.
    #[inline(always)]
    pub fn dtsrst(&mut self) -> DTSRST_W<'_, APB4RSTRrs> {
        DTSRST_W::new(self, 26)
    }
}
/**RCC APB4 Peripheral Reset Register

You can [`read`](crate::Reg::read) this register and get [`apb4rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#RCC:APB4RSTR)*/
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
