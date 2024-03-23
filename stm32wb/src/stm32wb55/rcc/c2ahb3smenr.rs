#[doc = "Register `C2AHB3SMENR` reader"]
pub type R = crate::R<C2AHB3SMENRrs>;
#[doc = "Register `C2AHB3SMENR` writer"]
pub type W = crate::W<C2AHB3SMENRrs>;
#[doc = "Field `PKASMEN` reader - PKA accelerator clocks enable during CPU2 sleep modes"]
pub type PKASMEN_R = crate::BitReader;
#[doc = "Field `PKASMEN` writer - PKA accelerator clocks enable during CPU2 sleep modes"]
pub type PKASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES2SMEN` reader - AES2 accelerator clocks enable during CPU2 sleep modes"]
pub type AES2SMEN_R = crate::BitReader;
#[doc = "Field `AES2SMEN` writer - AES2 accelerator clocks enable during CPU2 sleep modes"]
pub type AES2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGSMEN` reader - True RNG clocks enable during CPU2 sleep modes"]
pub type RNGSMEN_R = crate::BitReader;
#[doc = "Field `RNGSMEN` writer - True RNG clocks enable during CPU2 sleep modes"]
pub type RNGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2SMEN` reader - SRAM2a and SRAM2b memory interface clocks enable during CPU2 sleep modes"]
pub type SRAM2SMEN_R = crate::BitReader;
#[doc = "Field `SRAM2SMEN` writer - SRAM2a and SRAM2b memory interface clocks enable during CPU2 sleep modes"]
pub type SRAM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHSMEN` reader - Flash interface clocks enable during CPU2 sleep modes"]
pub type FLASHSMEN_R = crate::BitReader;
#[doc = "Field `FLASHSMEN` writer - Flash interface clocks enable during CPU2 sleep modes"]
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - PKA accelerator clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AES2 accelerator clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn aes2smen(&self) -> AES2SMEN_R {
        AES2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - True RNG clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - SRAM2a and SRAM2b memory interface clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Flash interface clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - PKA accelerator clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    #[must_use]
    pub fn pkasmen(&mut self) -> PKASMEN_W<C2AHB3SMENRrs> {
        PKASMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - AES2 accelerator clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    #[must_use]
    pub fn aes2smen(&mut self) -> AES2SMEN_W<C2AHB3SMENRrs> {
        AES2SMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - True RNG clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    #[must_use]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<C2AHB3SMENRrs> {
        RNGSMEN_W::new(self, 18)
    }
    #[doc = "Bit 24 - SRAM2a and SRAM2b memory interface clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<C2AHB3SMENRrs> {
        SRAM2SMEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Flash interface clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<C2AHB3SMENRrs> {
        FLASHSMEN_W::new(self, 25)
    }
}
#[doc = "CPU2 AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ahb3smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ahb3smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2AHB3SMENRrs;
impl crate::RegisterSpec for C2AHB3SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2ahb3smenr::R`](R) reader structure"]
impl crate::Readable for C2AHB3SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`c2ahb3smenr::W`](W) writer structure"]
impl crate::Writable for C2AHB3SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2AHB3SMENR to value 0x0307_0000"]
impl crate::Resettable for C2AHB3SMENRrs {
    const RESET_VALUE: u32 = 0x0307_0000;
}
