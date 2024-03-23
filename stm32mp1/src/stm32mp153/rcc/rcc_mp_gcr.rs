#[doc = "Register `RCC_MP_GCR` reader"]
pub type R = crate::R<RCC_MP_GCRrs>;
#[doc = "Register `RCC_MP_GCR` writer"]
pub type W = crate::W<RCC_MP_GCRrs>;
#[doc = "Field `BOOT_MCU` reader - BOOT_MCU"]
pub type BOOT_MCU_R = crate::BitReader;
#[doc = "Field `BOOT_MCU` writer - BOOT_MCU"]
pub type BOOT_MCU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BOOT_MCU"]
    #[inline(always)]
    pub fn boot_mcu(&self) -> BOOT_MCU_R {
        BOOT_MCU_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOOT_MCU"]
    #[inline(always)]
    #[must_use]
    pub fn boot_mcu(&mut self) -> BOOT_MCU_W<RCC_MP_GCRrs> {
        BOOT_MCU_W::new(self, 0)
    }
}
#[doc = "The register contains global control bits. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_GCRrs;
impl crate::RegisterSpec for RCC_MP_GCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_gcr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_GCRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_gcr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_GCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_GCR to value 0"]
impl crate::Resettable for RCC_MP_GCRrs {
    const RESET_VALUE: u32 = 0;
}
