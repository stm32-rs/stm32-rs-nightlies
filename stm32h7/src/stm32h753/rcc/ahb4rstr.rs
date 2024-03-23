#[doc = "Register `AHB4RSTR` reader"]
pub type R = crate::R<AHB4RSTRrs>;
#[doc = "Register `AHB4RSTR` writer"]
pub type W = crate::W<AHB4RSTRrs>;
#[doc = "GPIO block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<GPIOARST> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOARST` reader - GPIO block reset"]
pub type GPIOARST_R = crate::BitReader<GPIOARST>;
impl GPIOARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIOARST> {
        match self.bits {
            true => Some(GPIOARST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPIOARST::Reset
    }
}
#[doc = "Field `GPIOARST` writer - GPIO block reset"]
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOARST>;
impl<'a, REG> GPIOARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST::Reset)
    }
}
#[doc = "Field `GPIOBRST` reader - GPIO block reset"]
pub use GPIOARST_R as GPIOBRST_R;
#[doc = "Field `GPIOCRST` reader - GPIO block reset"]
pub use GPIOARST_R as GPIOCRST_R;
#[doc = "Field `GPIODRST` reader - GPIO block reset"]
pub use GPIOARST_R as GPIODRST_R;
#[doc = "Field `GPIOERST` reader - GPIO block reset"]
pub use GPIOARST_R as GPIOERST_R;
#[doc = "Field `GPIOFRST` reader - GPIO block reset"]
pub use GPIOARST_R as GPIOFRST_R;
#[doc = "Field `GPIOGRST` reader - GPIO block reset"]
pub use GPIOARST_R as GPIOGRST_R;
#[doc = "Field `GPIOHRST` reader - GPIO block reset"]
pub use GPIOARST_R as GPIOHRST_R;
#[doc = "Field `GPIOIRST` reader - GPIO block reset"]
pub use GPIOARST_R as GPIOIRST_R;
#[doc = "Field `GPIOJRST` reader - GPIO block reset"]
pub use GPIOARST_R as GPIOJRST_R;
#[doc = "Field `GPIOKRST` reader - GPIO block reset"]
pub use GPIOARST_R as GPIOKRST_R;
#[doc = "Field `CRCRST` reader - CRC block reset"]
pub use GPIOARST_R as CRCRST_R;
#[doc = "Field `BDMARST` reader - BDMA block reset"]
pub use GPIOARST_R as BDMARST_R;
#[doc = "Field `ADC3RST` reader - ADC3 block reset"]
pub use GPIOARST_R as ADC3RST_R;
#[doc = "Field `HSEMRST` reader - HSEM block reset"]
pub use GPIOARST_R as HSEMRST_R;
#[doc = "Field `GPIOBRST` writer - GPIO block reset"]
pub use GPIOARST_W as GPIOBRST_W;
#[doc = "Field `GPIOCRST` writer - GPIO block reset"]
pub use GPIOARST_W as GPIOCRST_W;
#[doc = "Field `GPIODRST` writer - GPIO block reset"]
pub use GPIOARST_W as GPIODRST_W;
#[doc = "Field `GPIOERST` writer - GPIO block reset"]
pub use GPIOARST_W as GPIOERST_W;
#[doc = "Field `GPIOFRST` writer - GPIO block reset"]
pub use GPIOARST_W as GPIOFRST_W;
#[doc = "Field `GPIOGRST` writer - GPIO block reset"]
pub use GPIOARST_W as GPIOGRST_W;
#[doc = "Field `GPIOHRST` writer - GPIO block reset"]
pub use GPIOARST_W as GPIOHRST_W;
#[doc = "Field `GPIOIRST` writer - GPIO block reset"]
pub use GPIOARST_W as GPIOIRST_W;
#[doc = "Field `GPIOJRST` writer - GPIO block reset"]
pub use GPIOARST_W as GPIOJRST_W;
#[doc = "Field `GPIOKRST` writer - GPIO block reset"]
pub use GPIOARST_W as GPIOKRST_W;
#[doc = "Field `CRCRST` writer - CRC block reset"]
pub use GPIOARST_W as CRCRST_W;
#[doc = "Field `BDMARST` writer - BDMA block reset"]
pub use GPIOARST_W as BDMARST_W;
#[doc = "Field `ADC3RST` writer - ADC3 block reset"]
pub use GPIOARST_W as ADC3RST_W;
#[doc = "Field `HSEMRST` writer - HSEM block reset"]
pub use GPIOARST_W as HSEMRST_W;
impl R {
    #[doc = "Bit 0 - GPIO block reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO block reset"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO block reset"]
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiojrst(&self) -> GPIOJRST_R {
        GPIOJRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiokrst(&self) -> GPIOKRST_R {
        GPIOKRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 19 - CRC block reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - BDMA block reset"]
    #[inline(always)]
    pub fn bdmarst(&self) -> BDMARST_R {
        BDMARST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC3 block reset"]
    #[inline(always)]
    pub fn adc3rst(&self) -> ADC3RST_R {
        ADC3RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HSEM block reset"]
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<AHB4RSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<AHB4RSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<AHB4RSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<AHB4RSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<AHB4RSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<AHB4RSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<AHB4RSTRrs> {
        GPIOGRST_W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<AHB4RSTRrs> {
        GPIOHRST_W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<AHB4RSTRrs> {
        GPIOIRST_W::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiojrst(&mut self) -> GPIOJRST_W<AHB4RSTRrs> {
        GPIOJRST_W::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiokrst(&mut self) -> GPIOKRST_W<AHB4RSTRrs> {
        GPIOKRST_W::new(self, 10)
    }
    #[doc = "Bit 19 - CRC block reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<AHB4RSTRrs> {
        CRCRST_W::new(self, 19)
    }
    #[doc = "Bit 21 - BDMA block reset"]
    #[inline(always)]
    #[must_use]
    pub fn bdmarst(&mut self) -> BDMARST_W<AHB4RSTRrs> {
        BDMARST_W::new(self, 21)
    }
    #[doc = "Bit 24 - ADC3 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc3rst(&mut self) -> ADC3RST_W<AHB4RSTRrs> {
        ADC3RST_W::new(self, 24)
    }
    #[doc = "Bit 25 - HSEM block reset"]
    #[inline(always)]
    #[must_use]
    pub fn hsemrst(&mut self) -> HSEMRST_W<AHB4RSTRrs> {
        HSEMRST_W::new(self, 25)
    }
}
#[doc = "RCC AHB4 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB4RSTRrs;
impl crate::RegisterSpec for AHB4RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb4rstr::R`](R) reader structure"]
impl crate::Readable for AHB4RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb4rstr::W`](W) writer structure"]
impl crate::Writable for AHB4RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB4RSTR to value 0"]
impl crate::Resettable for AHB4RSTRrs {
    const RESET_VALUE: u32 = 0;
}
