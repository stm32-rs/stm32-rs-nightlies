#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICRrs>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `EOAIC` reader - End of acquisition interrupt clear"]
pub type EOAIC_R = crate::BitReader;
#[doc = "Field `EOAIC` writer - End of acquisition interrupt clear"]
pub type EOAIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEIC` reader - Max count error interrupt clear"]
pub type MCEIC_R = crate::BitReader;
#[doc = "Field `MCEIC` writer - Max count error interrupt clear"]
pub type MCEIC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End of acquisition interrupt clear"]
    #[inline(always)]
    pub fn eoaic(&self) -> EOAIC_R {
        EOAIC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max count error interrupt clear"]
    #[inline(always)]
    pub fn mceic(&self) -> MCEIC_R {
        MCEIC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of acquisition interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn eoaic(&mut self) -> EOAIC_W<ICRrs> {
        EOAIC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Max count error interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn mceic(&mut self) -> MCEIC_W<ICRrs> {
        MCEIC_W::new(self, 1)
    }
}
#[doc = "interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICRrs {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
