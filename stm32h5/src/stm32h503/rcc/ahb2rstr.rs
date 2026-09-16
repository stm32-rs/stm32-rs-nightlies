///Register `AHB2RSTR` reader
pub type R = crate::R<AHB2RSTRrs>;
///Register `AHB2RSTR` writer
pub type W = crate::W<AHB2RSTRrs>;
/**GPIOA block reset Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<GPIOARST> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOARST` reader - GPIOA block reset Set and reset by software.
pub type GPIOARST_R = crate::BitReader<GPIOARST>;
impl GPIOARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIOARST> {
        match self.bits {
            true => Some(GPIOARST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPIOARST::Reset
    }
}
///Field `GPIOARST` writer - GPIOA block reset Set and reset by software.
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOARST>;
impl<'a, REG> GPIOARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST::Reset)
    }
}
///Field `GPIOBRST` reader - GPIOB block reset Set and reset by software.
pub use GPIOARST_R as GPIOBRST_R;
///Field `GPIOCRST` reader - GPIOC block reset Set and reset by software.
pub use GPIOARST_R as GPIOCRST_R;
///Field `GPIODRST` reader - GPIOD block reset Set and reset by software.
pub use GPIOARST_R as GPIODRST_R;
///Field `GPIOHRST` reader - GPIOH block reset Set and reset by software.
pub use GPIOARST_R as GPIOHRST_R;
///Field `ADCRST` reader - ADC block reset
pub use GPIOARST_R as ADCRST_R;
///Field `DAC12RST` reader - DAC block reset Set and reset by software.
pub use GPIOARST_R as DAC12RST_R;
///Field `HASHRST` reader - HASH block reset Set and reset by software.
pub use GPIOARST_R as HASHRST_R;
///Field `RNGRST` reader - RNG block reset Set and reset by software.
pub use GPIOARST_R as RNGRST_R;
///Field `GPIOBRST` writer - GPIOB block reset Set and reset by software.
pub use GPIOARST_W as GPIOBRST_W;
///Field `GPIOCRST` writer - GPIOC block reset Set and reset by software.
pub use GPIOARST_W as GPIOCRST_W;
///Field `GPIODRST` writer - GPIOD block reset Set and reset by software.
pub use GPIOARST_W as GPIODRST_W;
///Field `GPIOHRST` writer - GPIOH block reset Set and reset by software.
pub use GPIOARST_W as GPIOHRST_W;
///Field `ADCRST` writer - ADC block reset
pub use GPIOARST_W as ADCRST_W;
///Field `DAC12RST` writer - DAC block reset Set and reset by software.
pub use GPIOARST_W as DAC12RST_W;
///Field `HASHRST` writer - HASH block reset Set and reset by software.
pub use GPIOARST_W as HASHRST_W;
///Field `RNGRST` writer - RNG block reset Set and reset by software.
pub use GPIOARST_W as RNGRST_W;
impl R {
    ///Bit 0 - GPIOA block reset Set and reset by software.
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOB block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOC block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOD block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - GPIOH block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - ADC block reset
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DAC block reset Set and reset by software.
    #[inline(always)]
    pub fn dac12rst(&self) -> DAC12RST_R {
        DAC12RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 17 - HASH block reset Set and reset by software.
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RNG block reset Set and reset by software.
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTR")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpiohrst", &self.gpiohrst())
            .field("adcrst", &self.adcrst())
            .field("dac12rst", &self.dac12rst())
            .field("hashrst", &self.hashrst())
            .field("rngrst", &self.rngrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOA block reset Set and reset by software.
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<'_, AHB2RSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    ///Bit 1 - GPIOB block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<'_, AHB2RSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    ///Bit 2 - GPIOC block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<'_, AHB2RSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    ///Bit 3 - GPIOD block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<'_, AHB2RSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    ///Bit 7 - GPIOH block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<'_, AHB2RSTRrs> {
        GPIOHRST_W::new(self, 7)
    }
    ///Bit 10 - ADC block reset
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W<'_, AHB2RSTRrs> {
        ADCRST_W::new(self, 10)
    }
    ///Bit 11 - DAC block reset Set and reset by software.
    #[inline(always)]
    pub fn dac12rst(&mut self) -> DAC12RST_W<'_, AHB2RSTRrs> {
        DAC12RST_W::new(self, 11)
    }
    ///Bit 17 - HASH block reset Set and reset by software.
    #[inline(always)]
    pub fn hashrst(&mut self) -> HASHRST_W<'_, AHB2RSTRrs> {
        HASHRST_W::new(self, 17)
    }
    ///Bit 18 - RNG block reset Set and reset by software.
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<'_, AHB2RSTRrs> {
        RNGRST_W::new(self, 18)
    }
}
/**RCC AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RCC:AHB2RSTR)*/
pub struct AHB2RSTRrs;
impl crate::RegisterSpec for AHB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2rstr::R`](R) reader structure
impl crate::Readable for AHB2RSTRrs {}
///`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure
impl crate::Writable for AHB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2RSTR to value 0
impl crate::Resettable for AHB2RSTRrs {}
