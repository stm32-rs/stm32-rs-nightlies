///Register `AHB4RSTR` reader
pub type R = crate::R<AHB4RSTRrs>;
///Register `AHB4RSTR` writer
pub type W = crate::W<AHB4RSTRrs>;
/**GPIO block reset

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
///Field `GPIOARST` reader - GPIO block reset
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
///Field `GPIOARST` writer - GPIO block reset
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
///Field `GPIOBRST` reader - GPIO block reset
pub use GPIOARST_R as GPIOBRST_R;
///Field `GPIOCRST` reader - GPIO block reset
pub use GPIOARST_R as GPIOCRST_R;
///Field `GPIODRST` reader - GPIO block reset
pub use GPIOARST_R as GPIODRST_R;
///Field `GPIOERST` reader - GPIO block reset
pub use GPIOARST_R as GPIOERST_R;
///Field `GPIOFRST` reader - GPIO block reset
pub use GPIOARST_R as GPIOFRST_R;
///Field `GPIOGRST` reader - GPIO block reset
pub use GPIOARST_R as GPIOGRST_R;
///Field `GPIOHRST` reader - GPIO block reset
pub use GPIOARST_R as GPIOHRST_R;
///Field `GPIOJRST` reader - GPIO block reset
pub use GPIOARST_R as GPIOJRST_R;
///Field `GPIOKRST` reader - GPIO block reset
pub use GPIOARST_R as GPIOKRST_R;
///Field `CRCRST` reader - CRC block reset
pub use GPIOARST_R as CRCRST_R;
///Field `BDMARST` reader - BDMA block reset
pub use GPIOARST_R as BDMARST_R;
///Field `ADC3RST` reader - ADC3 block reset
pub use GPIOARST_R as ADC3RST_R;
///Field `HSEMRST` reader - HSEM block reset
pub use GPIOARST_R as HSEMRST_R;
///Field `GPIOBRST` writer - GPIO block reset
pub use GPIOARST_W as GPIOBRST_W;
///Field `GPIOCRST` writer - GPIO block reset
pub use GPIOARST_W as GPIOCRST_W;
///Field `GPIODRST` writer - GPIO block reset
pub use GPIOARST_W as GPIODRST_W;
///Field `GPIOERST` writer - GPIO block reset
pub use GPIOARST_W as GPIOERST_W;
///Field `GPIOFRST` writer - GPIO block reset
pub use GPIOARST_W as GPIOFRST_W;
///Field `GPIOGRST` writer - GPIO block reset
pub use GPIOARST_W as GPIOGRST_W;
///Field `GPIOHRST` writer - GPIO block reset
pub use GPIOARST_W as GPIOHRST_W;
///Field `GPIOJRST` writer - GPIO block reset
pub use GPIOARST_W as GPIOJRST_W;
///Field `GPIOKRST` writer - GPIO block reset
pub use GPIOARST_W as GPIOKRST_W;
///Field `CRCRST` writer - CRC block reset
pub use GPIOARST_W as CRCRST_W;
///Field `BDMARST` writer - BDMA block reset
pub use GPIOARST_W as BDMARST_W;
///Field `ADC3RST` writer - ADC3 block reset
pub use GPIOARST_W as ADC3RST_W;
///Field `HSEMRST` writer - HSEM block reset
pub use GPIOARST_W as HSEMRST_W;
impl R {
    ///Bit 0 - GPIO block reset
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIO block reset
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIO block reset
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIO block reset
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIO block reset
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIO block reset
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIO block reset
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIO block reset
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - GPIO block reset
    #[inline(always)]
    pub fn gpiojrst(&self) -> GPIOJRST_R {
        GPIOJRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPIO block reset
    #[inline(always)]
    pub fn gpiokrst(&self) -> GPIOKRST_R {
        GPIOKRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 19 - CRC block reset
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - BDMA block reset
    #[inline(always)]
    pub fn bdmarst(&self) -> BDMARST_R {
        BDMARST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - ADC3 block reset
    #[inline(always)]
    pub fn adc3rst(&self) -> ADC3RST_R {
        ADC3RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - HSEM block reset
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB4RSTR")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpioerst", &self.gpioerst())
            .field("gpiofrst", &self.gpiofrst())
            .field("gpiogrst", &self.gpiogrst())
            .field("gpiohrst", &self.gpiohrst())
            .field("gpiojrst", &self.gpiojrst())
            .field("gpiokrst", &self.gpiokrst())
            .field("crcrst", &self.crcrst())
            .field("bdmarst", &self.bdmarst())
            .field("adc3rst", &self.adc3rst())
            .field("hsemrst", &self.hsemrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIO block reset
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<'_, AHB4RSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    ///Bit 1 - GPIO block reset
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<'_, AHB4RSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    ///Bit 2 - GPIO block reset
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<'_, AHB4RSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    ///Bit 3 - GPIO block reset
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<'_, AHB4RSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    ///Bit 4 - GPIO block reset
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W<'_, AHB4RSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    ///Bit 5 - GPIO block reset
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<'_, AHB4RSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
    ///Bit 6 - GPIO block reset
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<'_, AHB4RSTRrs> {
        GPIOGRST_W::new(self, 6)
    }
    ///Bit 7 - GPIO block reset
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<'_, AHB4RSTRrs> {
        GPIOHRST_W::new(self, 7)
    }
    ///Bit 9 - GPIO block reset
    #[inline(always)]
    pub fn gpiojrst(&mut self) -> GPIOJRST_W<'_, AHB4RSTRrs> {
        GPIOJRST_W::new(self, 9)
    }
    ///Bit 10 - GPIO block reset
    #[inline(always)]
    pub fn gpiokrst(&mut self) -> GPIOKRST_W<'_, AHB4RSTRrs> {
        GPIOKRST_W::new(self, 10)
    }
    ///Bit 19 - CRC block reset
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<'_, AHB4RSTRrs> {
        CRCRST_W::new(self, 19)
    }
    ///Bit 21 - BDMA block reset
    #[inline(always)]
    pub fn bdmarst(&mut self) -> BDMARST_W<'_, AHB4RSTRrs> {
        BDMARST_W::new(self, 21)
    }
    ///Bit 24 - ADC3 block reset
    #[inline(always)]
    pub fn adc3rst(&mut self) -> ADC3RST_W<'_, AHB4RSTRrs> {
        ADC3RST_W::new(self, 24)
    }
    ///Bit 25 - HSEM block reset
    #[inline(always)]
    pub fn hsemrst(&mut self) -> HSEMRST_W<'_, AHB4RSTRrs> {
        HSEMRST_W::new(self, 25)
    }
}
/**RCC AHB4 Peripheral Reset Register

You can [`read`](crate::Reg::read) this register and get [`ahb4rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#RCC:AHB4RSTR)*/
pub struct AHB4RSTRrs;
impl crate::RegisterSpec for AHB4RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb4rstr::R`](R) reader structure
impl crate::Readable for AHB4RSTRrs {}
///`write(|w| ..)` method takes [`ahb4rstr::W`](W) writer structure
impl crate::Writable for AHB4RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4RSTR to value 0
impl crate::Resettable for AHB4RSTRrs {}
