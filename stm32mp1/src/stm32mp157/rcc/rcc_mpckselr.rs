#[doc = "Register `RCC_MPCKSELR` reader"]
pub type R = crate::R<RCC_MPCKSELRrs>;
#[doc = "Register `RCC_MPCKSELR` writer"]
pub type W = crate::W<RCC_MPCKSELRrs>;
#[doc = "Field `MPUSRC` reader - MPUSRC"]
pub type MPUSRC_R = crate::FieldReader;
#[doc = "Field `MPUSRC` writer - MPUSRC"]
pub type MPUSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MPUSRCRDY` reader - MPUSRCRDY"]
pub type MPUSRCRDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - MPUSRC"]
    #[inline(always)]
    pub fn mpusrc(&self) -> MPUSRC_R {
        MPUSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - MPUSRCRDY"]
    #[inline(always)]
    pub fn mpusrcrdy(&self) -> MPUSRCRDY_R {
        MPUSRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MPUSRC"]
    #[inline(always)]
    #[must_use]
    pub fn mpusrc(&mut self) -> MPUSRC_W<RCC_MPCKSELRrs> {
        MPUSRC_W::new(self, 0)
    }
}
#[doc = "This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mpckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mpckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MPCKSELRrs;
impl crate::RegisterSpec for RCC_MPCKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mpckselr::R`](R) reader structure"]
impl crate::Readable for RCC_MPCKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mpckselr::W`](W) writer structure"]
impl crate::Writable for RCC_MPCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MPCKSELR to value 0x8000_0000"]
impl crate::Resettable for RCC_MPCKSELRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
