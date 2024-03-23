#[doc = "Register `DLYB_CR` reader"]
pub type R = crate::R<DLYB_CRrs>;
#[doc = "Register `DLYB_CR` writer"]
pub type W = crate::W<DLYB_CRrs>;
#[doc = "Field `DEN` reader - DEN"]
pub type DEN_R = crate::BitReader;
#[doc = "Field `DEN` writer - DEN"]
pub type DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEN` reader - SEN"]
pub type SEN_R = crate::BitReader;
#[doc = "Field `SEN` writer - SEN"]
pub type SEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DEN"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SEN"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DEN"]
    #[inline(always)]
    #[must_use]
    pub fn den(&mut self) -> DEN_W<DLYB_CRrs> {
        DEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - SEN"]
    #[inline(always)]
    #[must_use]
    pub fn sen(&mut self) -> SEN_W<DLYB_CRrs> {
        SEN_W::new(self, 1)
    }
}
#[doc = "DLYB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlyb_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlyb_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLYB_CRrs;
impl crate::RegisterSpec for DLYB_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlyb_cr::R`](R) reader structure"]
impl crate::Readable for DLYB_CRrs {}
#[doc = "`write(|w| ..)` method takes [`dlyb_cr::W`](W) writer structure"]
impl crate::Writable for DLYB_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLYB_CR to value 0"]
impl crate::Resettable for DLYB_CRrs {
    const RESET_VALUE: u32 = 0;
}
