#[doc = "Register `PCTLR` reader"]
pub type R = crate::R<PCTLRrs>;
#[doc = "Register `PCTLR` writer"]
pub type W = crate::W<PCTLRrs>;
#[doc = "Field `DEN` reader - DEN"]
pub type DEN_R = crate::BitReader;
#[doc = "Field `DEN` writer - DEN"]
pub type DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKE` reader - CKE"]
pub type CKE_R = crate::BitReader;
#[doc = "Field `CKE` writer - CKE"]
pub type CKE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - DEN"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CKE"]
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DEN"]
    #[inline(always)]
    #[must_use]
    pub fn den(&mut self) -> DEN_W<PCTLRrs> {
        DEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CKE"]
    #[inline(always)]
    #[must_use]
    pub fn cke(&mut self) -> CKE_W<PCTLRrs> {
        CKE_W::new(self, 2)
    }
}
#[doc = "DSI Host PHY control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCTLRrs;
impl crate::RegisterSpec for PCTLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pctlr::R`](R) reader structure"]
impl crate::Readable for PCTLRrs {}
#[doc = "`write(|w| ..)` method takes [`pctlr::W`](W) writer structure"]
impl crate::Writable for PCTLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCTLR to value 0"]
impl crate::Resettable for PCTLRrs {
    const RESET_VALUE: u32 = 0;
}
