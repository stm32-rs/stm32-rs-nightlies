#[doc = "Register `AHB3RSTR` reader"]
pub type R = crate::R<AHB3RSTRrs>;
#[doc = "Register `AHB3RSTR` writer"]
pub type W = crate::W<AHB3RSTRrs>;
#[doc = "Field `QSPIRST` reader - Quad SPI memory interface reset"]
pub type QSPIRST_R = crate::BitReader;
#[doc = "Field `QSPIRST` writer - Quad SPI memory interface reset"]
pub type QSPIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKARST` reader - PKA interface reset"]
pub type PKARST_R = crate::BitReader;
#[doc = "Field `PKARST` writer - PKA interface reset"]
pub type PKARST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES2RST` reader - AES2 interface reset"]
pub type AES2RST_R = crate::BitReader;
#[doc = "Field `AES2RST` writer - AES2 interface reset"]
pub type AES2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGRST` reader - RNG interface reset"]
pub type RNGRST_R = crate::BitReader;
#[doc = "Field `RNGRST` writer - RNG interface reset"]
pub type RNGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEMRST` reader - HSEM interface reset"]
pub type HSEMRST_R = crate::BitReader;
#[doc = "Field `HSEMRST` writer - HSEM interface reset"]
pub type HSEMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPCCRST` reader - IPCC interface reset"]
pub type IPCCRST_R = crate::BitReader;
#[doc = "Field `IPCCRST` writer - IPCC interface reset"]
pub type IPCCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHRST` reader - Flash interface reset"]
pub type FLASHRST_R = crate::BitReader;
#[doc = "Field `FLASHRST` writer - Flash interface reset"]
pub type FLASHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Quad SPI memory interface reset"]
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - PKA interface reset"]
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AES2 interface reset"]
    #[inline(always)]
    pub fn aes2rst(&self) -> AES2RST_R {
        AES2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG interface reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSEM interface reset"]
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IPCC interface reset"]
    #[inline(always)]
    pub fn ipccrst(&self) -> IPCCRST_R {
        IPCCRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - Flash interface reset"]
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Quad SPI memory interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn qspirst(&mut self) -> QSPIRST_W<AHB3RSTRrs> {
        QSPIRST_W::new(self, 8)
    }
    #[doc = "Bit 16 - PKA interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn pkarst(&mut self) -> PKARST_W<AHB3RSTRrs> {
        PKARST_W::new(self, 16)
    }
    #[doc = "Bit 17 - AES2 interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn aes2rst(&mut self) -> AES2RST_W<AHB3RSTRrs> {
        AES2RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - RNG interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<AHB3RSTRrs> {
        RNGRST_W::new(self, 18)
    }
    #[doc = "Bit 19 - HSEM interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn hsemrst(&mut self) -> HSEMRST_W<AHB3RSTRrs> {
        HSEMRST_W::new(self, 19)
    }
    #[doc = "Bit 20 - IPCC interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn ipccrst(&mut self) -> IPCCRST_W<AHB3RSTRrs> {
        IPCCRST_W::new(self, 20)
    }
    #[doc = "Bit 25 - Flash interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FLASHRST_W<AHB3RSTRrs> {
        FLASHRST_W::new(self, 25)
    }
}
#[doc = "AHB3 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3RSTRrs;
impl crate::RegisterSpec for AHB3RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3rstr::R`](R) reader structure"]
impl crate::Readable for AHB3RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure"]
impl crate::Writable for AHB3RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3RSTR to value 0"]
impl crate::Resettable for AHB3RSTRrs {
    const RESET_VALUE: u32 = 0;
}
