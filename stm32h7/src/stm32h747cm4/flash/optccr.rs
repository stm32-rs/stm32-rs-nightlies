#[doc = "Register `OPTCCR` reader"]
pub type R = crate::R<OPTCCRrs>;
#[doc = "Register `OPTCCR` writer"]
pub type W = crate::W<OPTCCRrs>;
#[doc = "Field `CLR_OPTCHANGEERR` reader - OPTCHANGEERR reset bit"]
pub type CLR_OPTCHANGEERR_R = crate::BitReader;
#[doc = "Field `CLR_OPTCHANGEERR` writer - OPTCHANGEERR reset bit"]
pub type CLR_OPTCHANGEERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - OPTCHANGEERR reset bit"]
    #[inline(always)]
    pub fn clr_optchangeerr(&self) -> CLR_OPTCHANGEERR_R {
        CLR_OPTCHANGEERR_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - OPTCHANGEERR reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_optchangeerr(&mut self) -> CLR_OPTCHANGEERR_W<OPTCCRrs> {
        CLR_OPTCHANGEERR_W::new(self, 30)
    }
}
#[doc = "FLASH option clear control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTCCRrs;
impl crate::RegisterSpec for OPTCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optccr::R`](R) reader structure"]
impl crate::Readable for OPTCCRrs {}
#[doc = "`write(|w| ..)` method takes [`optccr::W`](W) writer structure"]
impl crate::Writable for OPTCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTCCR to value 0"]
impl crate::Resettable for OPTCCRrs {
    const RESET_VALUE: u32 = 0;
}
