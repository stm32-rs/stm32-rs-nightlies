#[doc = "Register `RCC_ASSCKSELR` reader"]
pub type R = crate::R<RCC_ASSCKSELRrs>;
#[doc = "Register `RCC_ASSCKSELR` writer"]
pub type W = crate::W<RCC_ASSCKSELRrs>;
#[doc = "Field `AXISSRC` reader - AXISSRC"]
pub type AXISSRC_R = crate::FieldReader;
#[doc = "Field `AXISSRC` writer - AXISSRC"]
pub type AXISSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AXISSRCRDY` reader - AXISSRCRDY"]
pub type AXISSRCRDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - AXISSRC"]
    #[inline(always)]
    pub fn axissrc(&self) -> AXISSRC_R {
        AXISSRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - AXISSRCRDY"]
    #[inline(always)]
    pub fn axissrcrdy(&self) -> AXISSRCRDY_R {
        AXISSRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AXISSRC"]
    #[inline(always)]
    #[must_use]
    pub fn axissrc(&mut self) -> AXISSRC_W<RCC_ASSCKSELRrs> {
        AXISSRC_W::new(self, 0)
    }
}
#[doc = "This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_assckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_assckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_ASSCKSELRrs;
impl crate::RegisterSpec for RCC_ASSCKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_assckselr::R`](R) reader structure"]
impl crate::Readable for RCC_ASSCKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_assckselr::W`](W) writer structure"]
impl crate::Writable for RCC_ASSCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_ASSCKSELR to value 0x8000_0000"]
impl crate::Resettable for RCC_ASSCKSELRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
