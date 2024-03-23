#[doc = "Register `CCIPR4` reader"]
pub type R = crate::R<CCIPR4rs>;
#[doc = "Register `CCIPR4` writer"]
pub type W = crate::W<CCIPR4rs>;
#[doc = "Field `OCTOSPI1SEL` reader - OCTOSPI1 kernel clock source selection Set and reset by software."]
pub type OCTOSPI1SEL_R = crate::FieldReader;
#[doc = "Field `OCTOSPI1SEL` writer - OCTOSPI1 kernel clock source selection Set and reset by software."]
pub type OCTOSPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYSTICKSEL` reader - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
pub type SYSTICKSEL_R = crate::FieldReader;
#[doc = "Field `SYSTICKSEL` writer - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
pub type SYSTICKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USBSEL` reader - USB kernel clock source selection"]
pub type USBSEL_R = crate::FieldReader;
#[doc = "Field `USBSEL` writer - USB kernel clock source selection"]
pub type USBSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDMMC1SEL` reader - SDMMC1 kernel clock source selection"]
pub type SDMMC1SEL_R = crate::BitReader;
#[doc = "Field `SDMMC1SEL` writer - SDMMC1 kernel clock source selection"]
pub type SDMMC1SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC2SEL` reader - SDMMC2 kernel clock source selection"]
pub type SDMMC2SEL_R = crate::BitReader;
#[doc = "Field `SDMMC2SEL` writer - SDMMC2 kernel clock source selection"]
pub type SDMMC2SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1SEL` reader - I2C1 kernel clock source selection"]
pub type I2C1SEL_R = crate::FieldReader;
#[doc = "Field `I2C1SEL` writer - I2C1 kernel clock source selection"]
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C2SEL` reader - I2C2 kernel clock source selection"]
pub type I2C2SEL_R = crate::FieldReader;
#[doc = "Field `I2C2SEL` writer - I2C2 kernel clock source selection"]
pub type I2C2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C3SEL` reader - I2C3 kernel clock source selection"]
pub type I2C3SEL_R = crate::FieldReader;
#[doc = "Field `I2C3SEL` writer - I2C3 kernel clock source selection"]
pub type I2C3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C4SEL` reader - I2C4 kernel clock source selection"]
pub type I2C4SEL_R = crate::FieldReader;
#[doc = "Field `I2C4SEL` writer - I2C4 kernel clock source selection"]
pub type I2C4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I3C1SEL` reader - I3C1 kernel clock source selection"]
pub type I3C1SEL_R = crate::FieldReader;
#[doc = "Field `I3C1SEL` writer - I3C1 kernel clock source selection"]
pub type I3C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - OCTOSPI1 kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn octospi1sel(&self) -> OCTOSPI1SEL_R {
        OCTOSPI1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
    #[inline(always)]
    pub fn systicksel(&self) -> SYSTICKSEL_R {
        SYSTICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USB kernel clock source selection"]
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SDMMC1 kernel clock source selection"]
    #[inline(always)]
    pub fn sdmmc1sel(&self) -> SDMMC1SEL_R {
        SDMMC1SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SDMMC2 kernel clock source selection"]
    #[inline(always)]
    pub fn sdmmc2sel(&self) -> SDMMC2SEL_R {
        SDMMC2SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:17 - I2C1 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - I2C2 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - I2C3 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - I2C4 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - I3C1 kernel clock source selection"]
    #[inline(always)]
    pub fn i3c1sel(&self) -> I3C1SEL_R {
        I3C1SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - OCTOSPI1 kernel clock source selection Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn octospi1sel(&mut self) -> OCTOSPI1SEL_W<CCIPR4rs> {
        OCTOSPI1SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
    #[inline(always)]
    #[must_use]
    pub fn systicksel(&mut self) -> SYSTICKSEL_W<CCIPR4rs> {
        SYSTICKSEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - USB kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbsel(&mut self) -> USBSEL_W<CCIPR4rs> {
        USBSEL_W::new(self, 4)
    }
    #[doc = "Bit 6 - SDMMC1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1sel(&mut self) -> SDMMC1SEL_W<CCIPR4rs> {
        SDMMC1SEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - SDMMC2 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2sel(&mut self) -> SDMMC2SEL_W<CCIPR4rs> {
        SDMMC2SEL_W::new(self, 7)
    }
    #[doc = "Bits 16:17 - I2C1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<CCIPR4rs> {
        I2C1SEL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - I2C2 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<CCIPR4rs> {
        I2C2SEL_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - I2C3 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<CCIPR4rs> {
        I2C3SEL_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - I2C4 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<CCIPR4rs> {
        I2C4SEL_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - I3C1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1sel(&mut self) -> I3C1SEL_W<CCIPR4rs> {
        I3C1SEL_W::new(self, 24)
    }
}
#[doc = "RCC kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR4rs;
impl crate::RegisterSpec for CCIPR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr4::R`](R) reader structure"]
impl crate::Readable for CCIPR4rs {}
#[doc = "`write(|w| ..)` method takes [`ccipr4::W`](W) writer structure"]
impl crate::Writable for CCIPR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR4 to value 0"]
impl crate::Resettable for CCIPR4rs {
    const RESET_VALUE: u32 = 0;
}
