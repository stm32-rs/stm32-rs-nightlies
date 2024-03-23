#[doc = "Register `RCC_MC_AHB5ENSETR` reader"]
pub type R = crate::R<RCC_MC_AHB5ENSETRrs>;
#[doc = "Register `RCC_MC_AHB5ENSETR` writer"]
pub type W = crate::W<RCC_MC_AHB5ENSETRrs>;
#[doc = "Field `GPIOZEN` reader - GPIOZEN"]
pub type GPIOZEN_R = crate::BitReader;
#[doc = "Field `GPIOZEN` writer - GPIOZEN"]
pub type GPIOZEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYP1EN` reader - CRYP1EN"]
pub type CRYP1EN_R = crate::BitReader;
#[doc = "Field `CRYP1EN` writer - CRYP1EN"]
pub type CRYP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH1EN` reader - HASH1EN"]
pub type HASH1EN_R = crate::BitReader;
#[doc = "Field `HASH1EN` writer - HASH1EN"]
pub type HASH1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG1EN` reader - RNG1EN"]
pub type RNG1EN_R = crate::BitReader;
#[doc = "Field `RNG1EN` writer - RNG1EN"]
pub type RNG1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPSRAMEN` reader - BKPSRAMEN"]
pub type BKPSRAMEN_R = crate::BitReader;
#[doc = "Field `BKPSRAMEN` writer - BKPSRAMEN"]
pub type BKPSRAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOZEN"]
    #[inline(always)]
    pub fn gpiozen(&self) -> GPIOZEN_R {
        GPIOZEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYP1EN"]
    #[inline(always)]
    pub fn cryp1en(&self) -> CRYP1EN_R {
        CRYP1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH1EN"]
    #[inline(always)]
    pub fn hash1en(&self) -> HASH1EN_R {
        HASH1EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG1EN"]
    #[inline(always)]
    pub fn rng1en(&self) -> RNG1EN_R {
        RNG1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - BKPSRAMEN"]
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiozen(&mut self) -> GPIOZEN_W<RCC_MC_AHB5ENSETRrs> {
        GPIOZEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - CRYP1EN"]
    #[inline(always)]
    #[must_use]
    pub fn cryp1en(&mut self) -> CRYP1EN_W<RCC_MC_AHB5ENSETRrs> {
        CRYP1EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - HASH1EN"]
    #[inline(always)]
    #[must_use]
    pub fn hash1en(&mut self) -> HASH1EN_W<RCC_MC_AHB5ENSETRrs> {
        HASH1EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - RNG1EN"]
    #[inline(always)]
    #[must_use]
    pub fn rng1en(&mut self) -> RNG1EN_W<RCC_MC_AHB5ENSETRrs> {
        RNG1EN_W::new(self, 6)
    }
    #[doc = "Bit 8 - BKPSRAMEN"]
    #[inline(always)]
    #[must_use]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W<RCC_MC_AHB5ENSETRrs> {
        BKPSRAMEN_W::new(self, 8)
    }
}
#[doc = "This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb5ensetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb5ensetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_AHB5ENSETRrs;
impl crate::RegisterSpec for RCC_MC_AHB5ENSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_ahb5ensetr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_AHB5ENSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_ahb5ensetr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_AHB5ENSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_AHB5ENSETR to value 0"]
impl crate::Resettable for RCC_MC_AHB5ENSETRrs {
    const RESET_VALUE: u32 = 0;
}
