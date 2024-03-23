#[doc = "Register `TIM6_DCR` reader"]
pub type R = crate::R<TIM6_DCRrs>;
#[doc = "Register `TIM6_DCR` writer"]
pub type W = crate::W<TIM6_DCRrs>;
#[doc = "Field `DBA` reader - DBA"]
pub type DBA_R = crate::FieldReader;
#[doc = "Field `DBA` writer - DBA"]
pub type DBA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBL` reader - DBL"]
pub type DBL_R = crate::FieldReader;
#[doc = "Field `DBL` writer - DBL"]
pub type DBL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DBA"]
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DBL"]
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DBA"]
    #[inline(always)]
    #[must_use]
    pub fn dba(&mut self) -> DBA_W<TIM6_DCRrs> {
        DBA_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - DBL"]
    #[inline(always)]
    #[must_use]
    pub fn dbl(&mut self) -> DBL_W<TIM6_DCRrs> {
        DBL_W::new(self, 8)
    }
}
#[doc = "TIM6 DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim6_dcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim6_dcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM6_DCRrs;
impl crate::RegisterSpec for TIM6_DCRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim6_dcr::R`](R) reader structure"]
impl crate::Readable for TIM6_DCRrs {}
#[doc = "`write(|w| ..)` method takes [`tim6_dcr::W`](W) writer structure"]
impl crate::Writable for TIM6_DCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM6_DCR to value 0"]
impl crate::Resettable for TIM6_DCRrs {
    const RESET_VALUE: u16 = 0;
}
