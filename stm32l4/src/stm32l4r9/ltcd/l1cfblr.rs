#[doc = "Register `L1CFBLR` reader"]
pub type R = crate::R<L1CFBLRrs>;
#[doc = "Register `L1CFBLR` writer"]
pub type W = crate::W<L1CFBLRrs>;
#[doc = "Field `CFBLL` reader - Color Frame Buffer Line Length"]
pub type CFBLL_R = crate::FieldReader<u16>;
#[doc = "Field `CFBLL` writer - Color Frame Buffer Line Length"]
pub type CFBLL_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `CFBP` reader - Color Frame Buffer Pitch in bytes"]
pub type CFBP_R = crate::FieldReader<u16>;
#[doc = "Field `CFBP` writer - Color Frame Buffer Pitch in bytes"]
pub type CFBP_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Color Frame Buffer Line Length"]
    #[inline(always)]
    pub fn cfbll(&self) -> CFBLL_R {
        CFBLL_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Color Frame Buffer Pitch in bytes"]
    #[inline(always)]
    pub fn cfbp(&self) -> CFBP_R {
        CFBP_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Color Frame Buffer Line Length"]
    #[inline(always)]
    #[must_use]
    pub fn cfbll(&mut self) -> CFBLL_W<L1CFBLRrs> {
        CFBLL_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Color Frame Buffer Pitch in bytes"]
    #[inline(always)]
    #[must_use]
    pub fn cfbp(&mut self) -> CFBP_W<L1CFBLRrs> {
        CFBP_W::new(self, 16)
    }
}
#[doc = "LTDC Layer Color Frame Buffer Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cfblr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cfblr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CFBLRrs;
impl crate::RegisterSpec for L1CFBLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1cfblr::R`](R) reader structure"]
impl crate::Readable for L1CFBLRrs {}
#[doc = "`write(|w| ..)` method takes [`l1cfblr::W`](W) writer structure"]
impl crate::Writable for L1CFBLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1CFBLR to value 0"]
impl crate::Resettable for L1CFBLRrs {
    const RESET_VALUE: u32 = 0;
}
