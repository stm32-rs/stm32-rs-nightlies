#[doc = "Register `CLRFR` reader"]
pub type R = crate::R<CLRFRrs>;
#[doc = "Register `CLRFR` writer"]
pub type W = crate::W<CLRFRrs>;
#[doc = "Field `PROCENDFC` reader - Clear PKA End of Operation flag"]
pub type PROCENDFC_R = crate::BitReader;
#[doc = "Field `PROCENDFC` writer - Clear PKA End of Operation flag"]
pub type PROCENDFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERRFC` reader - Clear RAM error flag"]
pub type RAMERRFC_R = crate::BitReader;
#[doc = "Field `RAMERRFC` writer - Clear RAM error flag"]
pub type RAMERRFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRERRFC` reader - Clear Address error flag"]
pub type ADDRERRFC_R = crate::BitReader;
#[doc = "Field `ADDRERRFC` writer - Clear Address error flag"]
pub type ADDRERRFC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17 - Clear PKA End of Operation flag"]
    #[inline(always)]
    pub fn procendfc(&self) -> PROCENDFC_R {
        PROCENDFC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear RAM error flag"]
    #[inline(always)]
    pub fn ramerrfc(&self) -> RAMERRFC_R {
        RAMERRFC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Clear Address error flag"]
    #[inline(always)]
    pub fn addrerrfc(&self) -> ADDRERRFC_R {
        ADDRERRFC_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Clear PKA End of Operation flag"]
    #[inline(always)]
    #[must_use]
    pub fn procendfc(&mut self) -> PROCENDFC_W<CLRFRrs> {
        PROCENDFC_W::new(self, 17)
    }
    #[doc = "Bit 19 - Clear RAM error flag"]
    #[inline(always)]
    #[must_use]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W<CLRFRrs> {
        RAMERRFC_W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear Address error flag"]
    #[inline(always)]
    #[must_use]
    pub fn addrerrfc(&mut self) -> ADDRERRFC_W<CLRFRrs> {
        ADDRERRFC_W::new(self, 20)
    }
}
#[doc = "PKA clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clrfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLRFRrs;
impl crate::RegisterSpec for CLRFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clrfr::R`](R) reader structure"]
impl crate::Readable for CLRFRrs {}
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
