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
///Field `GPIOERST` reader - GPIOE block reset Set and reset by software.
pub use GPIOARST_R as GPIOERST_R;
///Field `GPIOFRST` reader - GPIOF block reset Set and reset by software.
pub use GPIOARST_R as GPIOFRST_R;
///Field `GPIOGRST` reader - GPIOG block reset Set and reset by software.
pub use GPIOARST_R as GPIOGRST_R;
///Field `GPIOHRST` reader - GPIOH block reset Set and reset by software.
pub use GPIOARST_R as GPIOHRST_R;
///Field `GPIOIRST` reader - GPIOI block reset Set and reset by software.
pub use GPIOARST_R as GPIOIRST_R;
///Field `ADCRST` reader - ADC1 and 2 blocks reset
pub use GPIOARST_R as ADCRST_R;
///Field `DAC1RST` reader - DAC block reset
pub use GPIOARST_R as DAC1RST_R;
///Field `DCMI_PSSIRST` reader - digital camera interface block reset (DCMI or PSSI depending which interface is active) Set and reset by software.
pub use GPIOARST_R as DCMI_PSSIRST_R;
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
///Field `GPIOERST` writer - GPIOE block reset Set and reset by software.
pub use GPIOARST_W as GPIOERST_W;
///Field `GPIOFRST` writer - GPIOF block reset Set and reset by software.
pub use GPIOARST_W as GPIOFRST_W;
///Field `GPIOGRST` writer - GPIOG block reset Set and reset by software.
pub use GPIOARST_W as GPIOGRST_W;
///Field `GPIOHRST` writer - GPIOH block reset Set and reset by software.
pub use GPIOARST_W as GPIOHRST_W;
///Field `GPIOIRST` writer - GPIOI block reset Set and reset by software.
pub use GPIOARST_W as GPIOIRST_W;
///Field `ADCRST` writer - ADC1 and 2 blocks reset
pub use GPIOARST_W as ADCRST_W;
///Field `DAC1RST` writer - DAC block reset
pub use GPIOARST_W as DAC1RST_W;
///Field `DCMI_PSSIRST` writer - digital camera interface block reset (DCMI or PSSI depending which interface is active) Set and reset by software.
pub use GPIOARST_W as DCMI_PSSIRST_W;
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
    ///Bit 4 - GPIOE block reset Set and reset by software.
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOF block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOG block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOH block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIOI block reset Set and reset by software.
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - ADC1 and 2 blocks reset
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DAC block reset
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - digital camera interface block reset (DCMI or PSSI depending which interface is active) Set and reset by software.
    #[inline(always)]
    pub fn dcmi_pssirst(&self) -> DCMI_PSSIRST_R {
        DCMI_PSSIRST_R::new(((self.bits >> 12) & 1) != 0)
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
            .field("gpioerst", &self.gpioerst())
            .field("gpiofrst", &self.gpiofrst())
            .field("gpiogrst", &self.gpiogrst())
            .field("gpiohrst", &self.gpiohrst())
            .field("gpioirst", &self.gpioirst())
            .field("adcrst", &self.adcrst())
            .field("dac1rst", &self.dac1rst())
            .field("dcmi_pssirst", &self.dcmi_pssirst())
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
    ///Bit 4 - GPIOE block reset Set and reset by software.
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W<'_, AHB2RSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    ///Bit 5 - GPIOF block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<'_, AHB2RSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
    ///Bit 6 - GPIOG block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<'_, AHB2RSTRrs> {
        GPIOGRST_W::new(self, 6)
    }
    ///Bit 7 - GPIOH block reset Set and reset by software.
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<'_, AHB2RSTRrs> {
        GPIOHRST_W::new(self, 7)
    }
    ///Bit 8 - GPIOI block reset Set and reset by software.
    #[inline(always)]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<'_, AHB2RSTRrs> {
        GPIOIRST_W::new(self, 8)
    }
    ///Bit 10 - ADC1 and 2 blocks reset
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W<'_, AHB2RSTRrs> {
        ADCRST_W::new(self, 10)
    }
    ///Bit 11 - DAC block reset
    #[inline(always)]
    pub fn dac1rst(&mut self) -> DAC1RST_W<'_, AHB2RSTRrs> {
        DAC1RST_W::new(self, 11)
    }
    ///Bit 12 - digital camera interface block reset (DCMI or PSSI depending which interface is active) Set and reset by software.
    #[inline(always)]
    pub fn dcmi_pssirst(&mut self) -> DCMI_PSSIRST_W<'_, AHB2RSTRrs> {
        DCMI_PSSIRST_W::new(self, 12)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#RCC:AHB2RSTR)*/
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
