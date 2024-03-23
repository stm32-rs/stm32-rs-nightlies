#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<AHB2RSTRrs>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<AHB2RSTRrs>;
#[doc = "Field `GPIOARST` reader - IO port A reset"]
pub type GPIOARST_R = crate::BitReader;
#[doc = "Field `GPIOARST` writer - IO port A reset"]
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - IO port B reset"]
pub type GPIOBRST_R = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - IO port B reset"]
pub type GPIOBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCRST` reader - IO port C reset"]
pub type GPIOCRST_R = crate::BitReader;
#[doc = "Field `GPIOCRST` writer - IO port C reset"]
pub type GPIOCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODRST` reader - IO port D reset"]
pub type GPIODRST_R = crate::BitReader;
#[doc = "Field `GPIODRST` writer - IO port D reset"]
pub type GPIODRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOERST` reader - IO port E reset"]
pub type GPIOERST_R = crate::BitReader;
#[doc = "Field `GPIOERST` writer - IO port E reset"]
pub type GPIOERST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHRST` reader - IO port H reset"]
pub type GPIOHRST_R = crate::BitReader;
#[doc = "Field `GPIOHRST` writer - IO port H reset"]
pub type GPIOHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub type ADCRST_R = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub type ADCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES1RST` reader - AES1 hardware accelerator reset"]
pub type AES1RST_R = crate::BitReader;
#[doc = "Field `AES1RST` writer - AES1 hardware accelerator reset"]
pub type AES1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 16 - AES1 hardware accelerator reset"]
    #[inline(always)]
    pub fn aes1rst(&self) -> AES1RST_R {
        AES1RST_R::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 16 - AES1 hardware accelerator reset"]
    #[inline(always)]
    #[must_use]
    pub fn aes1rst(&mut self) -> AES1RST_W<AHB2RSTRrs> {
        AES1RST_W::new(self, 16)
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
