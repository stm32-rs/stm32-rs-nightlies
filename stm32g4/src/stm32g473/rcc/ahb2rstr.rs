///Register `AHB2RSTR` reader
pub type R = crate::R<AHB2RSTRrs>;
///Register `AHB2RSTR` writer
pub type W = crate::W<AHB2RSTRrs>;
/**IO port A reset

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
///Field `GPIOARST` reader - IO port A reset
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
///Field `GPIOARST` writer - IO port A reset
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
///Field `GPIOBRST` reader - IO port B reset
pub use GPIOARST_R as GPIOBRST_R;
///Field `GPIOCRST` reader - IO port C reset
pub use GPIOARST_R as GPIOCRST_R;
///Field `GPIODRST` reader - IO port D reset
pub use GPIOARST_R as GPIODRST_R;
///Field `GPIOERST` reader - IO port E reset
pub use GPIOARST_R as GPIOERST_R;
///Field `GPIOFRST` reader - IO port F reset
pub use GPIOARST_R as GPIOFRST_R;
///Field `GPIOGRST` reader - IO port G reset
pub use GPIOARST_R as GPIOGRST_R;
///Field `ADC12RST` reader - ADC12 reset
pub use GPIOARST_R as ADC12RST_R;
///Field `ADC345RST` reader - ADC345 reset
pub use GPIOARST_R as ADC345RST_R;
///Field `DAC1RST` reader - DAC1 reset
pub use GPIOARST_R as DAC1RST_R;
///Field `DAC2RST` reader - DAC2 reset
pub use GPIOARST_R as DAC2RST_R;
///Field `DAC3RST` reader - DAC3 reset
pub use GPIOARST_R as DAC3RST_R;
///Field `DAC4RST` reader - DAC4 reset
pub use GPIOARST_R as DAC4RST_R;
///Field `AESRST` reader - AESRST reset
pub use GPIOARST_R as AESRST_R;
///Field `RNGRST` reader - RNG reset
pub use GPIOARST_R as RNGRST_R;
///Field `GPIOBRST` writer - IO port B reset
pub use GPIOARST_W as GPIOBRST_W;
///Field `GPIOCRST` writer - IO port C reset
pub use GPIOARST_W as GPIOCRST_W;
///Field `GPIODRST` writer - IO port D reset
pub use GPIOARST_W as GPIODRST_W;
///Field `GPIOERST` writer - IO port E reset
pub use GPIOARST_W as GPIOERST_W;
///Field `GPIOFRST` writer - IO port F reset
pub use GPIOARST_W as GPIOFRST_W;
///Field `GPIOGRST` writer - IO port G reset
pub use GPIOARST_W as GPIOGRST_W;
///Field `ADC12RST` writer - ADC12 reset
pub use GPIOARST_W as ADC12RST_W;
///Field `ADC345RST` writer - ADC345 reset
pub use GPIOARST_W as ADC345RST_W;
///Field `DAC1RST` writer - DAC1 reset
pub use GPIOARST_W as DAC1RST_W;
///Field `DAC2RST` writer - DAC2 reset
pub use GPIOARST_W as DAC2RST_W;
///Field `DAC3RST` writer - DAC3 reset
pub use GPIOARST_W as DAC3RST_W;
///Field `DAC4RST` writer - DAC4 reset
pub use GPIOARST_W as DAC4RST_W;
///Field `AESRST` writer - AESRST reset
pub use GPIOARST_W as AESRST_W;
///Field `RNGRST` writer - RNG reset
pub use GPIOARST_W as RNGRST_W;
impl R {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - ADC12 reset
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ADC345 reset
    #[inline(always)]
    pub fn adc345rst(&self) -> ADC345RST_R {
        ADC345RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - DAC1 reset
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DAC2 reset
    #[inline(always)]
    pub fn dac2rst(&self) -> DAC2RST_R {
        DAC2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DAC3 reset
    #[inline(always)]
    pub fn dac3rst(&self) -> DAC3RST_R {
        DAC3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DAC4 reset
    #[inline(always)]
    pub fn dac4rst(&self) -> DAC4RST_R {
        DAC4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - AESRST reset
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - RNG reset
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTR")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpioerst", &self.gpioerst())
            .field("gpiofrst", &self.gpiofrst())
            .field("gpiogrst", &self.gpiogrst())
            .field("adc12rst", &self.adc12rst())
            .field("adc345rst", &self.adc345rst())
            .field("dac1rst", &self.dac1rst())
            .field("dac2rst", &self.dac2rst())
            .field("dac3rst", &self.dac3rst())
            .field("dac4rst", &self.dac4rst())
            .field("aesrst", &self.aesrst())
            .field("rngrst", &self.rngrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<'_, AHB2RSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<'_, AHB2RSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<'_, AHB2RSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<'_, AHB2RSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W<'_, AHB2RSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<'_, AHB2RSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<'_, AHB2RSTRrs> {
        GPIOGRST_W::new(self, 6)
    }
    ///Bit 13 - ADC12 reset
    #[inline(always)]
    pub fn adc12rst(&mut self) -> ADC12RST_W<'_, AHB2RSTRrs> {
        ADC12RST_W::new(self, 13)
    }
    ///Bit 14 - ADC345 reset
    #[inline(always)]
    pub fn adc345rst(&mut self) -> ADC345RST_W<'_, AHB2RSTRrs> {
        ADC345RST_W::new(self, 14)
    }
    ///Bit 16 - DAC1 reset
    #[inline(always)]
    pub fn dac1rst(&mut self) -> DAC1RST_W<'_, AHB2RSTRrs> {
        DAC1RST_W::new(self, 16)
    }
    ///Bit 17 - DAC2 reset
    #[inline(always)]
    pub fn dac2rst(&mut self) -> DAC2RST_W<'_, AHB2RSTRrs> {
        DAC2RST_W::new(self, 17)
    }
    ///Bit 18 - DAC3 reset
    #[inline(always)]
    pub fn dac3rst(&mut self) -> DAC3RST_W<'_, AHB2RSTRrs> {
        DAC3RST_W::new(self, 18)
    }
    ///Bit 19 - DAC4 reset
    #[inline(always)]
    pub fn dac4rst(&mut self) -> DAC4RST_W<'_, AHB2RSTRrs> {
        DAC4RST_W::new(self, 19)
    }
    ///Bit 24 - AESRST reset
    #[inline(always)]
    pub fn aesrst(&mut self) -> AESRST_W<'_, AHB2RSTRrs> {
        AESRST_W::new(self, 24)
    }
    ///Bit 26 - RNG reset
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<'_, AHB2RSTRrs> {
        RNGRST_W::new(self, 26)
    }
}
/**AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#RCC:AHB2RSTR)*/
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
