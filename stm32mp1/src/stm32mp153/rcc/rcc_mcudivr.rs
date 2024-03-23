#[doc = "Register `RCC_MCUDIVR` reader"]
pub type R = crate::R<RCC_MCUDIVRrs>;
#[doc = "Register `RCC_MCUDIVR` writer"]
pub type W = crate::W<RCC_MCUDIVRrs>;
#[doc = "Field `MCUDIV` reader - MCUDIV"]
pub type MCUDIV_R = crate::FieldReader;
#[doc = "Field `MCUDIV` writer - MCUDIV"]
pub type MCUDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCUDIVRDY` reader - MCUDIVRDY"]
pub type MCUDIVRDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - MCUDIV"]
    #[inline(always)]
    pub fn mcudiv(&self) -> MCUDIV_R {
        MCUDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - MCUDIVRDY"]
    #[inline(always)]
    pub fn mcudivrdy(&self) -> MCUDIVRDY_R {
        MCUDIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MCUDIV"]
    #[inline(always)]
    #[must_use]
    pub fn mcudiv(&mut self) -> MCUDIV_W<RCC_MCUDIVRrs> {
        MCUDIV_W::new(self, 0)
    }
}
#[doc = "This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mcudivr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mcudivr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MCUDIVRrs;
impl crate::RegisterSpec for RCC_MCUDIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mcudivr::R`](R) reader structure"]
impl crate::Readable for RCC_MCUDIVRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mcudivr::W`](W) writer structure"]
impl crate::Writable for RCC_MCUDIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MCUDIVR to value 0x8000_0000"]
impl crate::Resettable for RCC_MCUDIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
