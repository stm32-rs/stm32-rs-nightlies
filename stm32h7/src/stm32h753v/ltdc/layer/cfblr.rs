#[doc = "Register `CFBLR` reader"]
pub type R = crate::R<CFBLRrs>;
#[doc = "Register `CFBLR` writer"]
pub type W = crate::W<CFBLRrs>;
#[doc = "Field `CFBLL` reader - Color Frame Buffer Line Length"]
pub type CFBLL_R = crate::FieldReader<u16>;
#[doc = "Field `CFBLL` writer - Color Frame Buffer Line Length"]
pub type CFBLL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 13, u16>;
#[doc = "Field `CFBP` reader - Color Frame Buffer Pitch in bytes"]
pub type CFBP_R = crate::FieldReader<u16>;
#[doc = "Field `CFBP` writer - Color Frame Buffer Pitch in bytes"]
pub type CFBP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 13, u16>;
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
    pub fn cfbll(&mut self) -> CFBLL_W<CFBLRrs> {
        CFBLL_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Color Frame Buffer Pitch in bytes"]
    #[inline(always)]
    #[must_use]
    pub fn cfbp(&mut self) -> CFBP_W<CFBLRrs> {
        CFBP_W::new(self, 16)
    }
}
#[doc = "Layerx Color Frame Buffer Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfblr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfblr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFBLRrs;
impl crate::RegisterSpec for CFBLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfblr::R`](R) reader structure"]
impl crate::Readable for CFBLRrs {}
#[doc = "`write(|w| ..)` method takes [`cfblr::W`](W) writer structure"]
impl crate::Writable for CFBLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFBLR to value 0"]
impl crate::Resettable for CFBLRrs {
    const RESET_VALUE: u32 = 0;
}
