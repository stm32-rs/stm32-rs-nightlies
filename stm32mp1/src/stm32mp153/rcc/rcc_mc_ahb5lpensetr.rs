#[doc = "Register `RCC_MC_AHB5LPENSETR` reader"]
pub type R = crate::R<RCC_MC_AHB5LPENSETRrs>;
#[doc = "Register `RCC_MC_AHB5LPENSETR` writer"]
pub type W = crate::W<RCC_MC_AHB5LPENSETRrs>;
#[doc = "Field `GPIOZLPEN` reader - GPIOZLPEN"]
pub type GPIOZLPEN_R = crate::BitReader;
#[doc = "Field `GPIOZLPEN` writer - GPIOZLPEN"]
pub type GPIOZLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYP1LPEN` reader - CRYP1LPEN"]
pub type CRYP1LPEN_R = crate::BitReader;
#[doc = "Field `CRYP1LPEN` writer - CRYP1LPEN"]
pub type CRYP1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH1LPEN` reader - HASH1LPEN"]
pub type HASH1LPEN_R = crate::BitReader;
#[doc = "Field `HASH1LPEN` writer - HASH1LPEN"]
pub type HASH1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG1LPEN` reader - RNG1LPEN"]
pub type RNG1LPEN_R = crate::BitReader;
#[doc = "Field `RNG1LPEN` writer - RNG1LPEN"]
pub type RNG1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPSRAMLPEN` reader - BKPSRAMLPEN"]
pub type BKPSRAMLPEN_R = crate::BitReader;
#[doc = "Field `BKPSRAMLPEN` writer - BKPSRAMLPEN"]
pub type BKPSRAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOZLPEN"]
    #[inline(always)]
    pub fn gpiozlpen(&self) -> GPIOZLPEN_R {
        GPIOZLPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYP1LPEN"]
    #[inline(always)]
    pub fn cryp1lpen(&self) -> CRYP1LPEN_R {
        CRYP1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH1LPEN"]
    #[inline(always)]
    pub fn hash1lpen(&self) -> HASH1LPEN_R {
        HASH1LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG1LPEN"]
    #[inline(always)]
    pub fn rng1lpen(&self) -> RNG1LPEN_R {
        RNG1LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - BKPSRAMLPEN"]
    #[inline(always)]
    pub fn bkpsramlpen(&self) -> BKPSRAMLPEN_R {
        BKPSRAMLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiozlpen(&mut self) -> GPIOZLPEN_W<RCC_MC_AHB5LPENSETRrs> {
        GPIOZLPEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - CRYP1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn cryp1lpen(&mut self) -> CRYP1LPEN_W<RCC_MC_AHB5LPENSETRrs> {
        CRYP1LPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - HASH1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn hash1lpen(&mut self) -> HASH1LPEN_W<RCC_MC_AHB5LPENSETRrs> {
        HASH1LPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - RNG1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn rng1lpen(&mut self) -> RNG1LPEN_W<RCC_MC_AHB5LPENSETRrs> {
        RNG1LPEN_W::new(self, 6)
    }
    #[doc = "Bit 8 - BKPSRAMLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn bkpsramlpen(&mut self) -> BKPSRAMLPEN_W<RCC_MC_AHB5LPENSETRrs> {
        BKPSRAMLPEN_W::new(self, 8)
    }
}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb5lpensetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb5lpensetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_AHB5LPENSETRrs;
impl crate::RegisterSpec for RCC_MC_AHB5LPENSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_ahb5lpensetr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_AHB5LPENSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_ahb5lpensetr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_AHB5LPENSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_AHB5LPENSETR to value 0x0171"]
impl crate::Resettable for RCC_MC_AHB5LPENSETRrs {
    const RESET_VALUE: u32 = 0x0171;
}
