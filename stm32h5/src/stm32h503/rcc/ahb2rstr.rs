#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<AHB2RSTRrs>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<AHB2RSTRrs>;
#[doc = "GPIOA block reset Set and reset by software.\n\nValue on reset: 0"]
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
#[doc = "Field `GPIOARST` reader - GPIOA block reset Set and reset by software."]
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
#[doc = "Field `GPIOARST` writer - GPIOA block reset Set and reset by software."]
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
#[doc = "Field `GPIOBRST` reader - GPIOB block reset Set and reset by software."]
pub use GPIOARST_R as GPIOBRST_R;
#[doc = "Field `GPIOCRST` reader - GPIOC block reset Set and reset by software."]
pub use GPIOARST_R as GPIOCRST_R;
#[doc = "Field `GPIODRST` reader - GPIOD block reset Set and reset by software."]
pub use GPIOARST_R as GPIODRST_R;
#[doc = "Field `GPIOHRST` reader - GPIOH block reset Set and reset by software."]
pub use GPIOARST_R as GPIOHRST_R;
#[doc = "Field `ADCRST` reader - ADC block reset"]
pub use GPIOARST_R as ADCRST_R;
#[doc = "Field `DAC12RST` reader - DAC block reset Set and reset by software."]
pub use GPIOARST_R as DAC12RST_R;
#[doc = "Field `HASHRST` reader - HASH block reset Set and reset by software."]
pub use GPIOARST_R as HASHRST_R;
#[doc = "Field `RNGRST` reader - RNG block reset Set and reset by software."]
pub use GPIOARST_R as RNGRST_R;
#[doc = "Field `GPIOBRST` writer - GPIOB block reset Set and reset by software."]
pub use GPIOARST_W as GPIOBRST_W;
#[doc = "Field `GPIOCRST` writer - GPIOC block reset Set and reset by software."]
pub use GPIOARST_W as GPIOCRST_W;
#[doc = "Field `GPIODRST` writer - GPIOD block reset Set and reset by software."]
pub use GPIOARST_W as GPIODRST_W;
#[doc = "Field `GPIOHRST` writer - GPIOH block reset Set and reset by software."]
pub use GPIOARST_W as GPIOHRST_W;
#[doc = "Field `ADCRST` writer - ADC block reset"]
pub use GPIOARST_W as ADCRST_W;
#[doc = "Field `DAC12RST` writer - DAC block reset Set and reset by software."]
pub use GPIOARST_W as DAC12RST_W;
#[doc = "Field `HASHRST` writer - HASH block reset Set and reset by software."]
pub use GPIOARST_W as HASHRST_W;
#[doc = "Field `RNGRST` writer - RNG block reset Set and reset by software."]
pub use GPIOARST_W as RNGRST_W;
impl R {
    #[doc = "Bit 0 - GPIOA block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOH block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC block reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC block reset Set and reset by software."]
    #[inline(always)]
    pub fn dac12rst(&self) -> DAC12RST_R {
        DAC12RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH block reset Set and reset by software."]
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG block reset Set and reset by software."]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<AHB2RSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOB block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<AHB2RSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOC block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<AHB2RSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOD block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<AHB2RSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    #[doc = "Bit 7 - GPIOH block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<AHB2RSTRrs> {
        GPIOHRST_W::new(self, 7)
    }
    #[doc = "Bit 10 - ADC block reset"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<AHB2RSTRrs> {
        ADCRST_W::new(self, 10)
    }
    #[doc = "Bit 11 - DAC block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac12rst(&mut self) -> DAC12RST_W<AHB2RSTRrs> {
        DAC12RST_W::new(self, 11)
    }
    #[doc = "Bit 17 - HASH block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hashrst(&mut self) -> HASHRST_W<AHB2RSTRrs> {
        HASHRST_W::new(self, 17)
    }
    #[doc = "Bit 18 - RNG block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<AHB2RSTRrs> {
        RNGRST_W::new(self, 18)
    }
}
#[doc = "RCC AHB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2RSTRrs;
impl crate::RegisterSpec for AHB2RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rstr::R`](R) reader structure"]
impl crate::Readable for AHB2RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure"]
impl crate::Writable for AHB2RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for AHB2RSTRrs {
    const RESET_VALUE: u32 = 0;
}
