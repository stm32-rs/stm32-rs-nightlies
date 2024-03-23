#[doc = "Register `TOCV` reader"]
pub type R = crate::R<TOCVrs>;
#[doc = "Register `TOCV` writer"]
pub type W = crate::W<TOCVrs>;
#[doc = "Field `TOC` reader - Timeout Counter"]
pub type TOC_R = crate::FieldReader<u16>;
#[doc = "Field `TOC` writer - Timeout Counter"]
pub type TOC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timeout Counter"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<TOCVrs> {
        TOC_W::new(self, 0)
    }
}
#[doc = "FDCAN Timeout Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOCVrs;
impl crate::RegisterSpec for TOCVrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocv::R`](R) reader structure"]
impl crate::Readable for TOCVrs {}
#[doc = "`write(|w| ..)` method takes [`tocv::W`](W) writer structure"]
impl crate::Writable for TOCVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOCV to value 0"]
impl crate::Resettable for TOCVrs {
    const RESET_VALUE: u32 = 0;
}
