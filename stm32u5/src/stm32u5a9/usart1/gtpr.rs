#[doc = "Register `GTPR` reader"]
pub type R = crate::R<GTPRrs>;
#[doc = "Register `GTPR` writer"]
pub type W = crate::W<GTPRrs>;
#[doc = "Field `PSC` reader - Prescaler value"]
pub type PSC_R = crate::FieldReader;
#[doc = "Field `PSC` writer - Prescaler value"]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GT` reader - Guard time value"]
pub type GT_R = crate::FieldReader;
#[doc = "Field `GT` writer - Guard time value"]
pub type GT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Guard time value"]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<GTPRrs> {
        PSC_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Guard time value"]
    #[inline(always)]
    #[must_use]
    pub fn gt(&mut self) -> GT_W<GTPRrs> {
        GT_W::new(self, 8)
    }
}
#[doc = "Guard time and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTPRrs;
impl crate::RegisterSpec for GTPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtpr::R`](R) reader structure"]
impl crate::Readable for GTPRrs {}
#[doc = "`write(|w| ..)` method takes [`gtpr::W`](W) writer structure"]
impl crate::Writable for GTPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTPR to value 0"]
impl crate::Resettable for GTPRrs {
    const RESET_VALUE: u32 = 0;
}
