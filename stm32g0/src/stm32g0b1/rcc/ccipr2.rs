#[doc = "Register `CCIPR2` reader"]
pub type R = crate::R<CCIPR2rs>;
#[doc = "Register `CCIPR2` writer"]
pub type W = crate::W<CCIPR2rs>;
#[doc = "Field `I2S1SEL` reader - 2S1SEL"]
pub type I2S1SEL_R = crate::FieldReader;
#[doc = "Field `I2S1SEL` writer - 2S1SEL"]
pub type I2S1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S2SEL` reader - I2S2SEL"]
pub type I2S2SEL_R = crate::FieldReader;
#[doc = "Field `I2S2SEL` writer - I2S2SEL"]
pub type I2S2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FDCANSEL` reader - FDCANSEL"]
pub type FDCANSEL_R = crate::FieldReader;
#[doc = "Field `FDCANSEL` writer - FDCANSEL"]
pub type FDCANSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USBSEL` reader - USBSEL"]
pub type USBSEL_R = crate::FieldReader;
#[doc = "Field `USBSEL` writer - USBSEL"]
pub type USBSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 2S1SEL"]
    #[inline(always)]
    pub fn i2s1sel(&self) -> I2S1SEL_R {
        I2S1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - I2S2SEL"]
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2S2SEL_R {
        I2S2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - FDCANSEL"]
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - USBSEL"]
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 2S1SEL"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1sel(&mut self) -> I2S1SEL_W<CCIPR2rs> {
        I2S1SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - I2S2SEL"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2sel(&mut self) -> I2S2SEL_W<CCIPR2rs> {
        I2S2SEL_W::new(self, 2)
    }
    #[doc = "Bits 8:9 - FDCANSEL"]
    #[inline(always)]
    #[must_use]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<CCIPR2rs> {
        FDCANSEL_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - USBSEL"]
    #[inline(always)]
    #[must_use]
    pub fn usbsel(&mut self) -> USBSEL_W<CCIPR2rs> {
        USBSEL_W::new(self, 12)
    }
}
#[doc = "Peripherals independent clock configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr2::R`](R) reader structure"]
impl crate::Readable for CCIPR2rs {}
#[doc = "`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure"]
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR2 to value 0"]
impl crate::Resettable for CCIPR2rs {
    const RESET_VALUE: u32 = 0;
}
