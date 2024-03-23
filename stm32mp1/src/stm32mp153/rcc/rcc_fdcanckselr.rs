#[doc = "Register `RCC_FDCANCKSELR` reader"]
pub type R = crate::R<RCC_FDCANCKSELRrs>;
#[doc = "Register `RCC_FDCANCKSELR` writer"]
pub type W = crate::W<RCC_FDCANCKSELRrs>;
#[doc = "Field `FDCANSRC` reader - FDCANSRC"]
pub type FDCANSRC_R = crate::FieldReader;
#[doc = "Field `FDCANSRC` writer - FDCANSRC"]
pub type FDCANSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - FDCANSRC"]
    #[inline(always)]
    pub fn fdcansrc(&self) -> FDCANSRC_R {
        FDCANSRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FDCANSRC"]
    #[inline(always)]
    #[must_use]
    pub fn fdcansrc(&mut self) -> FDCANSRC_W<RCC_FDCANCKSELRrs> {
        FDCANSRC_W::new(self, 0)
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_fdcanckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_fdcanckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_FDCANCKSELRrs;
impl crate::RegisterSpec for RCC_FDCANCKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_fdcanckselr::R`](R) reader structure"]
impl crate::Readable for RCC_FDCANCKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_fdcanckselr::W`](W) writer structure"]
impl crate::Writable for RCC_FDCANCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_FDCANCKSELR to value 0"]
impl crate::Resettable for RCC_FDCANCKSELRrs {
    const RESET_VALUE: u32 = 0;
}
