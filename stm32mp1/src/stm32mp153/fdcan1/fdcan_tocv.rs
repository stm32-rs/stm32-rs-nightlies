#[doc = "Register `FDCAN_TOCV` reader"]
pub type R = crate::R<FDCAN_TOCVrs>;
#[doc = "Register `FDCAN_TOCV` writer"]
pub type W = crate::W<FDCAN_TOCVrs>;
#[doc = "Field `TOC` reader - TOC"]
pub type TOC_R = crate::FieldReader<u16>;
#[doc = "Field `TOC` writer - TOC"]
pub type TOC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TOC"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<FDCAN_TOCVrs> {
        TOC_W::new(self, 0)
    }
}
#[doc = "FDCAN timeout counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tocv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tocv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TOCVrs;
impl crate::RegisterSpec for FDCAN_TOCVrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tocv::R`](R) reader structure"]
impl crate::Readable for FDCAN_TOCVrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tocv::W`](W) writer structure"]
impl crate::Writable for FDCAN_TOCVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TOCV to value 0xffff"]
impl crate::Resettable for FDCAN_TOCVrs {
    const RESET_VALUE: u32 = 0xffff;
}
