#[doc = "Register `AHB4RSTR` reader"]
pub type R = crate::R<AHB4RSTRrs>;
#[doc = "Register `AHB4RSTR` writer"]
pub type W = crate::W<AHB4RSTRrs>;
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
#[doc = "Field `GPIOERST` reader - GPIOE block reset Set and reset by software."]
pub use GPIOARST_R as GPIOERST_R;
#[doc = "Field `GPIOFRST` reader - GPIOF block reset Set and reset by software."]
pub use GPIOARST_R as GPIOFRST_R;
#[doc = "Field `GPIOGRST` reader - GPIOG block reset Set and reset by software."]
pub use GPIOARST_R as GPIOGRST_R;
#[doc = "Field `GPIOHRST` reader - GPIOH block reset Set and reset by software."]
pub use GPIOARST_R as GPIOHRST_R;
#[doc = "Field `GPIOIRST` reader - GPIOI block reset Set and reset by software."]
pub use GPIOARST_R as GPIOIRST_R;
#[doc = "Field `GPIOJRST` reader - GPIOJ block reset Set and reset by software."]
pub use GPIOARST_R as GPIOJRST_R;
#[doc = "Field `GPIOKRST` reader - GPIOK block reset Set and reset by software."]
pub use GPIOARST_R as GPIOKRST_R;
#[doc = "Field `BDMA2RST` reader - SmartRun domain DMA and DMAMUX blocks reset Set and reset by software."]
pub use GPIOARST_R as BDMA2RST_R;
#[doc = "Field `GPIOBRST` writer - GPIOB block reset Set and reset by software."]
pub use GPIOARST_W as GPIOBRST_W;
#[doc = "Field `GPIOCRST` writer - GPIOC block reset Set and reset by software."]
pub use GPIOARST_W as GPIOCRST_W;
#[doc = "Field `GPIODRST` writer - GPIOD block reset Set and reset by software."]
pub use GPIOARST_W as GPIODRST_W;
#[doc = "Field `GPIOERST` writer - GPIOE block reset Set and reset by software."]
pub use GPIOARST_W as GPIOERST_W;
#[doc = "Field `GPIOFRST` writer - GPIOF block reset Set and reset by software."]
pub use GPIOARST_W as GPIOFRST_W;
#[doc = "Field `GPIOGRST` writer - GPIOG block reset Set and reset by software."]
pub use GPIOARST_W as GPIOGRST_W;
#[doc = "Field `GPIOHRST` writer - GPIOH block reset Set and reset by software."]
pub use GPIOARST_W as GPIOHRST_W;
#[doc = "Field `GPIOIRST` writer - GPIOI block reset Set and reset by software."]
pub use GPIOARST_W as GPIOIRST_W;
#[doc = "Field `GPIOJRST` writer - GPIOJ block reset Set and reset by software."]
pub use GPIOARST_W as GPIOJRST_W;
#[doc = "Field `GPIOKRST` writer - GPIOK block reset Set and reset by software."]
pub use GPIOARST_W as GPIOKRST_W;
#[doc = "Field `BDMA2RST` writer - SmartRun domain DMA and DMAMUX blocks reset Set and reset by software."]
pub use GPIOARST_W as BDMA2RST_W;
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
    #[doc = "Bit 4 - GPIOE block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOF block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOG block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOH block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOI block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOJ block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiojrst(&self) -> GPIOJRST_R {
        GPIOJRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOK block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpiokrst(&self) -> GPIOKRST_R {
        GPIOKRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 21 - SmartRun domain DMA and DMAMUX blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn bdma2rst(&self) -> BDMA2RST_R {
        BDMA2RST_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<AHB4RSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOB block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<AHB4RSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOC block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<AHB4RSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOD block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<AHB4RSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOE block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<AHB4RSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOF block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<AHB4RSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOG block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<AHB4RSTRrs> {
        GPIOGRST_W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOH block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<AHB4RSTRrs> {
        GPIOHRST_W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIOI block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<AHB4RSTRrs> {
        GPIOIRST_W::new(self, 8)
    }
    #[doc = "Bit 9 - GPIOJ block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiojrst(&mut self) -> GPIOJRST_W<AHB4RSTRrs> {
        GPIOJRST_W::new(self, 9)
    }
    #[doc = "Bit 10 - GPIOK block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiokrst(&mut self) -> GPIOKRST_W<AHB4RSTRrs> {
        GPIOKRST_W::new(self, 10)
    }
    #[doc = "Bit 21 - SmartRun domain DMA and DMAMUX blocks reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn bdma2rst(&mut self) -> BDMA2RST_W<AHB4RSTRrs> {
        BDMA2RST_W::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
