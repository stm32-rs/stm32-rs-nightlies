#[doc = "Register `SDMMC_IDMALAR` reader"]
pub type R = crate::R<SDMMC_IDMALARrs>;
#[doc = "Register `SDMMC_IDMALAR` writer"]
pub type W = crate::W<SDMMC_IDMALARrs>;
#[doc = "Field `IDMALA` reader - IDMALA"]
pub type IDMALA_R = crate::FieldReader<u16>;
#[doc = "Field `IDMALA` writer - IDMALA"]
pub type IDMALA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `ABR` reader - ABR"]
pub type ABR_R = crate::BitReader;
#[doc = "Field `ABR` writer - ABR"]
pub type ABR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULS` reader - ULS"]
pub type ULS_R = crate::BitReader;
#[doc = "Field `ULS` writer - ULS"]
pub type ULS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULA` reader - ULA"]
pub type ULA_R = crate::BitReader;
#[doc = "Field `ULA` writer - ULA"]
pub type ULA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - IDMALA"]
    #[inline(always)]
    pub fn idmala(&self) -> IDMALA_R {
        IDMALA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 29 - ABR"]
    #[inline(always)]
    pub fn abr(&self) -> ABR_R {
        ABR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ULS"]
    #[inline(always)]
    pub fn uls(&self) -> ULS_R {
        ULS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ULA"]
    #[inline(always)]
    pub fn ula(&self) -> ULA_R {
        ULA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - IDMALA"]
    #[inline(always)]
    #[must_use]
    pub fn idmala(&mut self) -> IDMALA_W<SDMMC_IDMALARrs> {
        IDMALA_W::new(self, 2)
    }
    #[doc = "Bit 29 - ABR"]
    #[inline(always)]
    #[must_use]
    pub fn abr(&mut self) -> ABR_W<SDMMC_IDMALARrs> {
        ABR_W::new(self, 29)
    }
    #[doc = "Bit 30 - ULS"]
    #[inline(always)]
    #[must_use]
    pub fn uls(&mut self) -> ULS_W<SDMMC_IDMALARrs> {
        ULS_W::new(self, 30)
    }
    #[doc = "Bit 31 - ULA"]
    #[inline(always)]
    #[must_use]
    pub fn ula(&mut self) -> ULA_W<SDMMC_IDMALARrs> {
        ULA_W::new(self, 31)
    }
}
#[doc = "SDMMC IDMA linked list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idmalar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idmalar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_IDMALARrs;
impl crate::RegisterSpec for SDMMC_IDMALARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_idmalar::R`](R) reader structure"]
impl crate::Readable for SDMMC_IDMALARrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_idmalar::W`](W) writer structure"]
impl crate::Writable for SDMMC_IDMALARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_IDMALAR to value 0"]
impl crate::Resettable for SDMMC_IDMALARrs {
    const RESET_VALUE: u32 = 0;
}
