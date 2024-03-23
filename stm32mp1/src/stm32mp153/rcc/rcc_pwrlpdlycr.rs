#[doc = "Register `RCC_PWRLPDLYCR` reader"]
pub type R = crate::R<RCC_PWRLPDLYCRrs>;
#[doc = "Register `RCC_PWRLPDLYCR` writer"]
pub type W = crate::W<RCC_PWRLPDLYCRrs>;
#[doc = "Field `PWRLP_DLY` reader - PWRLP_DLY"]
pub type PWRLP_DLY_R = crate::FieldReader<u32>;
#[doc = "Field `PWRLP_DLY` writer - PWRLP_DLY"]
pub type PWRLP_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `MCTMPSKP` reader - MCTMPSKP"]
pub type MCTMPSKP_R = crate::BitReader;
#[doc = "Field `MCTMPSKP` writer - MCTMPSKP"]
pub type MCTMPSKP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:21 - PWRLP_DLY"]
    #[inline(always)]
    pub fn pwrlp_dly(&self) -> PWRLP_DLY_R {
        PWRLP_DLY_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 24 - MCTMPSKP"]
    #[inline(always)]
    pub fn mctmpskp(&self) -> MCTMPSKP_R {
        MCTMPSKP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - PWRLP_DLY"]
    #[inline(always)]
    #[must_use]
    pub fn pwrlp_dly(&mut self) -> PWRLP_DLY_W<RCC_PWRLPDLYCRrs> {
        PWRLP_DLY_W::new(self, 0)
    }
    #[doc = "Bit 24 - MCTMPSKP"]
    #[inline(always)]
    #[must_use]
    pub fn mctmpskp(&mut self) -> MCTMPSKP_W<RCC_PWRLPDLYCRrs> {
        MCTMPSKP_W::new(self, 24)
    }
}
#[doc = "This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pwrlpdlycr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pwrlpdlycr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_PWRLPDLYCRrs;
impl crate::RegisterSpec for RCC_PWRLPDLYCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_pwrlpdlycr::R`](R) reader structure"]
impl crate::Readable for RCC_PWRLPDLYCRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_pwrlpdlycr::W`](W) writer structure"]
impl crate::Writable for RCC_PWRLPDLYCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_PWRLPDLYCR to value 0"]
impl crate::Resettable for RCC_PWRLPDLYCRrs {
    const RESET_VALUE: u32 = 0;
}
