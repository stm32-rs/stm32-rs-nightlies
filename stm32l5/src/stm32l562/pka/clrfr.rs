#[doc = "Register `CLRFR` writer"]
pub type W = crate::W<CLRFRrs>;
#[doc = "Field `PROCENDFC` writer - clear PKA end of operation flag"]
pub type PROCENDFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERRFC` writer - CLEAR PKA RAM ERROR FLAG"]
pub type RAMERRFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRERRFC` writer - clear address error flag"]
pub type ADDRERRFC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 17 - clear PKA end of operation flag"]
    #[inline(always)]
    #[must_use]
    pub fn procendfc(&mut self) -> PROCENDFC_W<CLRFRrs> {
        PROCENDFC_W::new(self, 17)
    }
    #[doc = "Bit 19 - CLEAR PKA RAM ERROR FLAG"]
    #[inline(always)]
    #[must_use]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W<CLRFRrs> {
        RAMERRFC_W::new(self, 19)
    }
    #[doc = "Bit 20 - clear address error flag"]
    #[inline(always)]
    #[must_use]
    pub fn addrerrfc(&mut self) -> ADDRERRFC_W<CLRFRrs> {
        ADDRERRFC_W::new(self, 20)
    }
}
#[doc = "PKA clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLRFRrs;
impl crate::RegisterSpec for CLRFRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clrfr::W`](W) writer structure"]
impl crate::Writable for CLRFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLRFR to value 0"]
impl crate::Resettable for CLRFRrs {
    const RESET_VALUE: u32 = 0;
}
