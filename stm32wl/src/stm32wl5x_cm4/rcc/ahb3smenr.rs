#[doc = "Register `AHB3SMENR` reader"]
pub type R = crate::R<AHB3SMENRrs>;
#[doc = "Register `AHB3SMENR` writer"]
pub type W = crate::W<AHB3SMENRrs>;
#[doc = "Field `PKASMEN` reader - PKA accelerator clock enable during CPU1 CSleep mode."]
pub type PKASMEN_R = crate::BitReader;
#[doc = "Field `PKASMEN` writer - PKA accelerator clock enable during CPU1 CSleep mode."]
pub type PKASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESSMEN` reader - AES accelerator clock enable during CPU1 CSleep mode."]
pub type AESSMEN_R = crate::BitReader;
#[doc = "Field `AESSMEN` writer - AES accelerator clock enable during CPU1 CSleep mode."]
pub type AESSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGSMEN` reader - True RNG clocks enable during CPU1 Csleep and CStop modes"]
pub type RNGSMEN_R = crate::BitReader;
#[doc = "Field `RNGSMEN` writer - True RNG clocks enable during CPU1 Csleep and CStop modes"]
pub type RNGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1SMEN` reader - SRAM1 interface clock enable during CPU1 CSleep mode."]
pub type SRAM1SMEN_R = crate::BitReader;
#[doc = "Field `SRAM1SMEN` writer - SRAM1 interface clock enable during CPU1 CSleep mode."]
pub type SRAM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2SMEN` reader - SRAM2 memory interface clock enable during CPU1 CSleep mode"]
pub type SRAM2SMEN_R = crate::BitReader;
#[doc = "Field `SRAM2SMEN` writer - SRAM2 memory interface clock enable during CPU1 CSleep mode"]
pub type SRAM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHSMEN` reader - Flash interface clock enable during CPU1 CSleep mode."]
pub type FLASHSMEN_R = crate::BitReader;
#[doc = "Field `FLASHSMEN` writer - Flash interface clock enable during CPU1 CSleep mode."]
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - PKA accelerator clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AES accelerator clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - True RNG clocks enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - SRAM1 interface clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SRAM2 memory interface clock enable during CPU1 CSleep mode"]
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Flash interface clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - PKA accelerator clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn pkasmen(&mut self) -> PKASMEN_W<AHB3SMENRrs> {
        PKASMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - AES accelerator clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn aessmen(&mut self) -> AESSMEN_W<AHB3SMENRrs> {
        AESSMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - True RNG clocks enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    #[must_use]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<AHB3SMENRrs> {
        RNGSMEN_W::new(self, 18)
    }
    #[doc = "Bit 23 - SRAM1 interface clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<AHB3SMENRrs> {
        SRAM1SMEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - SRAM2 memory interface clock enable during CPU1 CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<AHB3SMENRrs> {
        SRAM2SMEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Flash interface clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<AHB3SMENRrs> {
        FLASHSMEN_W::new(self, 25)
    }
}
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3SMENRrs;
impl crate::RegisterSpec for AHB3SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3smenr::R`](R) reader structure"]
impl crate::Readable for AHB3SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3smenr::W`](W) writer structure"]
impl crate::Writable for AHB3SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3SMENR to value 0x0387_0000"]
impl crate::Resettable for AHB3SMENRrs {
    const RESET_VALUE: u32 = 0x0387_0000;
}
