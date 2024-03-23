#[doc = "Register `RCC_MSSCKSELR` reader"]
pub type R = crate::R<RCC_MSSCKSELRrs>;
#[doc = "Register `RCC_MSSCKSELR` writer"]
pub type W = crate::W<RCC_MSSCKSELRrs>;
#[doc = "Field `MCUSSRC` reader - MCUSSRC"]
pub type MCUSSRC_R = crate::FieldReader;
#[doc = "Field `MCUSSRC` writer - MCUSSRC"]
pub type MCUSSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCUSSRCRDY` reader - MCUSSRCRDY"]
pub type MCUSSRCRDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - MCUSSRC"]
    #[inline(always)]
    pub fn mcussrc(&self) -> MCUSSRC_R {
        MCUSSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - MCUSSRCRDY"]
    #[inline(always)]
    pub fn mcussrcrdy(&self) -> MCUSSRCRDY_R {
        MCUSSRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MCUSSRC"]
    #[inline(always)]
    #[must_use]
    pub fn mcussrc(&mut self) -> MCUSSRC_W<RCC_MSSCKSELRrs> {
        MCUSSRC_W::new(self, 0)
    }
}
#[doc = "This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mssckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mssckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MSSCKSELRrs;
impl crate::RegisterSpec for RCC_MSSCKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mssckselr::R`](R) reader structure"]
impl crate::Readable for RCC_MSSCKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mssckselr::W`](W) writer structure"]
impl crate::Writable for RCC_MSSCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MSSCKSELR to value 0x8000_0000"]
impl crate::Resettable for RCC_MSSCKSELRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
