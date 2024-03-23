#[doc = "Register `RCC_AHB5RSTSETR` reader"]
pub type R = crate::R<RCC_AHB5RSTSETRrs>;
#[doc = "Register `RCC_AHB5RSTSETR` writer"]
pub type W = crate::W<RCC_AHB5RSTSETRrs>;
#[doc = "Field `GPIOZRST` reader - GPIOZRST"]
pub type GPIOZRST_R = crate::BitReader;
#[doc = "Field `GPIOZRST` writer - GPIOZRST"]
pub type GPIOZRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYP1RST` reader - CRYP1RST"]
pub type CRYP1RST_R = crate::BitReader;
#[doc = "Field `CRYP1RST` writer - CRYP1RST"]
pub type CRYP1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH1RST` reader - HASH1RST"]
pub type HASH1RST_R = crate::BitReader;
#[doc = "Field `HASH1RST` writer - HASH1RST"]
pub type HASH1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG1RST` reader - RNG1RST"]
pub type RNG1RST_R = crate::BitReader;
#[doc = "Field `RNG1RST` writer - RNG1RST"]
pub type RNG1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIMCRST` reader - AXIMCRST"]
pub type AXIMCRST_R = crate::BitReader;
#[doc = "Field `AXIMCRST` writer - AXIMCRST"]
pub type AXIMCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOZRST"]
    #[inline(always)]
    pub fn gpiozrst(&self) -> GPIOZRST_R {
        GPIOZRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYP1RST"]
    #[inline(always)]
    pub fn cryp1rst(&self) -> CRYP1RST_R {
        CRYP1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH1RST"]
    #[inline(always)]
    pub fn hash1rst(&self) -> HASH1RST_R {
        HASH1RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG1RST"]
    #[inline(always)]
    pub fn rng1rst(&self) -> RNG1RST_R {
        RNG1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - AXIMCRST"]
    #[inline(always)]
    pub fn aximcrst(&self) -> AXIMCRST_R {
        AXIMCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiozrst(&mut self) -> GPIOZRST_W<RCC_AHB5RSTSETRrs> {
        GPIOZRST_W::new(self, 0)
    }
    #[doc = "Bit 4 - CRYP1RST"]
    #[inline(always)]
    #[must_use]
    pub fn cryp1rst(&mut self) -> CRYP1RST_W<RCC_AHB5RSTSETRrs> {
        CRYP1RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - HASH1RST"]
    #[inline(always)]
    #[must_use]
    pub fn hash1rst(&mut self) -> HASH1RST_W<RCC_AHB5RSTSETRrs> {
        HASH1RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - RNG1RST"]
    #[inline(always)]
    #[must_use]
    pub fn rng1rst(&mut self) -> RNG1RST_W<RCC_AHB5RSTSETRrs> {
        RNG1RST_W::new(self, 6)
    }
    #[doc = "Bit 16 - AXIMCRST"]
    #[inline(always)]
    #[must_use]
    pub fn aximcrst(&mut self) -> AXIMCRST_W<RCC_AHB5RSTSETRrs> {
        AXIMCRST_W::new(self, 16)
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb5rstsetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb5rstsetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB5RSTSETRrs;
impl crate::RegisterSpec for RCC_AHB5RSTSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb5rstsetr::R`](R) reader structure"]
impl crate::Readable for RCC_AHB5RSTSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb5rstsetr::W`](W) writer structure"]
impl crate::Writable for RCC_AHB5RSTSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB5RSTSETR to value 0"]
impl crate::Resettable for RCC_AHB5RSTSETRrs {
    const RESET_VALUE: u32 = 0;
}
