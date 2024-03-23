#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<AHB2RSTRrs>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<AHB2RSTRrs>;
#[doc = "IO port A reset\n\nValue on reset: 0"]
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
#[doc = "Field `GPIOARST` reader - IO port A reset"]
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
#[doc = "Field `GPIOARST` writer - IO port A reset"]
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
#[doc = "Field `GPIOBRST` reader - IO port B reset"]
pub use GPIOARST_R as GPIOBRST_R;
#[doc = "Field `GPIOCRST` reader - IO port C reset"]
pub use GPIOARST_R as GPIOCRST_R;
#[doc = "Field `GPIODRST` reader - IO port D reset"]
pub use GPIOARST_R as GPIODRST_R;
#[doc = "Field `GPIOERST` reader - IO port E reset"]
pub use GPIOARST_R as GPIOERST_R;
#[doc = "Field `GPIOFRST` reader - IO port F reset"]
pub use GPIOARST_R as GPIOFRST_R;
#[doc = "Field `GPIOGRST` reader - IO port G reset"]
pub use GPIOARST_R as GPIOGRST_R;
#[doc = "Field `GPIOHRST` reader - IO port H reset"]
pub use GPIOARST_R as GPIOHRST_R;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub use GPIOARST_R as ADCRST_R;
#[doc = "Field `AESRST` reader - AES hardware accelerator reset"]
pub use GPIOARST_R as AESRST_R;
#[doc = "Field `HASHRST` reader - Hash reset"]
pub use GPIOARST_R as HASHRST_R;
#[doc = "Field `RNGRST` reader - Random number generator reset"]
pub use GPIOARST_R as RNGRST_R;
#[doc = "Field `PKARST` reader - PKARST"]
pub use GPIOARST_R as PKARST_R;
#[doc = "Field `OTFDEC1RST` reader - OTFDEC1RST"]
pub use GPIOARST_R as OTFDEC1RST_R;
#[doc = "Field `SDMMC1RST` reader - SDMMC1 reset"]
pub use GPIOARST_R as SDMMC1RST_R;
#[doc = "Field `GPIOBRST` writer - IO port B reset"]
pub use GPIOARST_W as GPIOBRST_W;
#[doc = "Field `GPIOCRST` writer - IO port C reset"]
pub use GPIOARST_W as GPIOCRST_W;
#[doc = "Field `GPIODRST` writer - IO port D reset"]
pub use GPIOARST_W as GPIODRST_W;
#[doc = "Field `GPIOERST` writer - IO port E reset"]
pub use GPIOARST_W as GPIOERST_W;
#[doc = "Field `GPIOFRST` writer - IO port F reset"]
pub use GPIOARST_W as GPIOFRST_W;
#[doc = "Field `GPIOGRST` writer - IO port G reset"]
pub use GPIOARST_W as GPIOGRST_W;
#[doc = "Field `GPIOHRST` writer - IO port H reset"]
pub use GPIOARST_W as GPIOHRST_W;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub use GPIOARST_W as ADCRST_W;
#[doc = "Field `AESRST` writer - AES hardware accelerator reset"]
pub use GPIOARST_W as AESRST_W;
#[doc = "Field `HASHRST` writer - Hash reset"]
pub use GPIOARST_W as HASHRST_W;
#[doc = "Field `RNGRST` writer - Random number generator reset"]
pub use GPIOARST_W as RNGRST_W;
#[doc = "Field `PKARST` writer - PKARST"]
pub use GPIOARST_W as PKARST_W;
#[doc = "Field `OTFDEC1RST` writer - OTFDEC1RST"]
pub use GPIOARST_W as OTFDEC1RST_W;
#[doc = "Field `SDMMC1RST` writer - SDMMC1 reset"]
pub use GPIOARST_W as SDMMC1RST_W;
impl R {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - AES hardware accelerator reset"]
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Hash reset"]
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Random number generator reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PKARST"]
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OTFDEC1RST"]
    #[inline(always)]
    pub fn otfdec1rst(&self) -> OTFDEC1RST_R {
        OTFDEC1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMMC1 reset"]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<AHB2RSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<AHB2RSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<AHB2RSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<AHB2RSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<AHB2RSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<AHB2RSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<AHB2RSTRrs> {
        GPIOGRST_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<AHB2RSTRrs> {
        GPIOHRST_W::new(self, 7)
    }
    #[doc = "Bit 13 - ADC reset"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<AHB2RSTRrs> {
        ADCRST_W::new(self, 13)
    }
    #[doc = "Bit 16 - AES hardware accelerator reset"]
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<AHB2RSTRrs> {
        AESRST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Hash reset"]
    #[inline(always)]
    #[must_use]
    pub fn hashrst(&mut self) -> HASHRST_W<AHB2RSTRrs> {
        HASHRST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Random number generator reset"]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<AHB2RSTRrs> {
        RNGRST_W::new(self, 18)
    }
    #[doc = "Bit 19 - PKARST"]
    #[inline(always)]
    #[must_use]
    pub fn pkarst(&mut self) -> PKARST_W<AHB2RSTRrs> {
        PKARST_W::new(self, 19)
    }
    #[doc = "Bit 21 - OTFDEC1RST"]
    #[inline(always)]
    #[must_use]
    pub fn otfdec1rst(&mut self) -> OTFDEC1RST_W<AHB2RSTRrs> {
        OTFDEC1RST_W::new(self, 21)
    }
    #[doc = "Bit 22 - SDMMC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<AHB2RSTRrs> {
        SDMMC1RST_W::new(self, 22)
    }
}
#[doc = "AHB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
