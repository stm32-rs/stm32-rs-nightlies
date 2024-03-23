#[doc = "Register `RCC_AHB3RSTSETR` reader"]
pub type R = crate::R<RCC_AHB3RSTSETRrs>;
#[doc = "Register `RCC_AHB3RSTSETR` writer"]
pub type W = crate::W<RCC_AHB3RSTSETRrs>;
#[doc = "Field `DCMIRST` reader - DCMIRST"]
pub type DCMIRST_R = crate::BitReader;
#[doc = "Field `DCMIRST` writer - DCMIRST"]
pub type DCMIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYP2RST` reader - CRYP2RST"]
pub type CRYP2RST_R = crate::BitReader;
#[doc = "Field `CRYP2RST` writer - CRYP2RST"]
pub type CRYP2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH2RST` reader - HASH2RST"]
pub type HASH2RST_R = crate::BitReader;
#[doc = "Field `HASH2RST` writer - HASH2RST"]
pub type HASH2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG2RST` reader - RNG2RST"]
pub type RNG2RST_R = crate::BitReader;
#[doc = "Field `RNG2RST` writer - RNG2RST"]
pub type RNG2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC2RST` reader - CRC2RST"]
pub type CRC2RST_R = crate::BitReader;
#[doc = "Field `CRC2RST` writer - CRC2RST"]
pub type CRC2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEMRST` reader - HSEMRST"]
pub type HSEMRST_R = crate::BitReader;
#[doc = "Field `HSEMRST` writer - HSEMRST"]
pub type HSEMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPCCRST` reader - IPCCRST"]
pub type IPCCRST_R = crate::BitReader;
#[doc = "Field `IPCCRST` writer - IPCCRST"]
pub type IPCCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DCMIRST"]
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYP2RST"]
    #[inline(always)]
    pub fn cryp2rst(&self) -> CRYP2RST_R {
        CRYP2RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH2RST"]
    #[inline(always)]
    pub fn hash2rst(&self) -> HASH2RST_R {
        HASH2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG2RST"]
    #[inline(always)]
    pub fn rng2rst(&self) -> RNG2RST_R {
        RNG2RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC2RST"]
    #[inline(always)]
    pub fn crc2rst(&self) -> CRC2RST_R {
        CRC2RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - HSEMRST"]
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IPCCRST"]
    #[inline(always)]
    pub fn ipccrst(&self) -> IPCCRST_R {
        IPCCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMIRST"]
    #[inline(always)]
    #[must_use]
    pub fn dcmirst(&mut self) -> DCMIRST_W<RCC_AHB3RSTSETRrs> {
        DCMIRST_W::new(self, 0)
    }
    #[doc = "Bit 4 - CRYP2RST"]
    #[inline(always)]
    #[must_use]
    pub fn cryp2rst(&mut self) -> CRYP2RST_W<RCC_AHB3RSTSETRrs> {
        CRYP2RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - HASH2RST"]
    #[inline(always)]
    #[must_use]
    pub fn hash2rst(&mut self) -> HASH2RST_W<RCC_AHB3RSTSETRrs> {
        HASH2RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - RNG2RST"]
    #[inline(always)]
    #[must_use]
    pub fn rng2rst(&mut self) -> RNG2RST_W<RCC_AHB3RSTSETRrs> {
        RNG2RST_W::new(self, 6)
    }
    #[doc = "Bit 7 - CRC2RST"]
    #[inline(always)]
    #[must_use]
    pub fn crc2rst(&mut self) -> CRC2RST_W<RCC_AHB3RSTSETRrs> {
        CRC2RST_W::new(self, 7)
    }
    #[doc = "Bit 11 - HSEMRST"]
    #[inline(always)]
    #[must_use]
    pub fn hsemrst(&mut self) -> HSEMRST_W<RCC_AHB3RSTSETRrs> {
        HSEMRST_W::new(self, 11)
    }
    #[doc = "Bit 12 - IPCCRST"]
    #[inline(always)]
    #[must_use]
    pub fn ipccrst(&mut self) -> IPCCRST_W<RCC_AHB3RSTSETRrs> {
        IPCCRST_W::new(self, 12)
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb3rstsetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb3rstsetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB3RSTSETRrs;
impl crate::RegisterSpec for RCC_AHB3RSTSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb3rstsetr::R`](R) reader structure"]
impl crate::Readable for RCC_AHB3RSTSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb3rstsetr::W`](W) writer structure"]
impl crate::Writable for RCC_AHB3RSTSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB3RSTSETR to value 0"]
impl crate::Resettable for RCC_AHB3RSTSETRrs {
    const RESET_VALUE: u32 = 0;
}
