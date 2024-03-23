#[doc = "Register `ALR%sBINR` reader"]
pub type R = crate::R<ALRBINRrs>;
#[doc = "Register `ALR%sBINR` writer"]
pub type W = crate::W<ALRBINRrs>;
#[doc = "Field `SS` reader - Synchronous counter alarm value in Binary mode"]
pub type SS_R = crate::FieldReader<u32>;
#[doc = "Field `SS` writer - Synchronous counter alarm value in Binary mode"]
pub type SS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Synchronous counter alarm value in Binary mode"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Synchronous counter alarm value in Binary mode"]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<ALRBINRrs> {
        SS_W::new(self, 0)
    }
}
#[doc = "Alarm %s binary mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrbinr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrbinr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRBINRrs;
impl crate::RegisterSpec for ALRBINRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrbinr::R`](R) reader structure"]
impl crate::Readable for ALRBINRrs {}
#[doc = "`write(|w| ..)` method takes [`alrbinr::W`](W) writer structure"]
impl crate::Writable for ALRBINRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALR%sBINR to value 0"]
impl crate::Resettable for ALRBINRrs {
    const RESET_VALUE: u32 = 0;
}
