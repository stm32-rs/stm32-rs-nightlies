#[doc = "Register `OTG_HNPTXFSIZ` reader"]
pub type R = crate::R<OTG_HNPTXFSIZrs>;
#[doc = "Register `OTG_HNPTXFSIZ` writer"]
pub type W = crate::W<OTG_HNPTXFSIZrs>;
#[doc = "Field `NPTXFSA` reader - NPTXFSA"]
pub type NPTXFSA_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFSA` writer - NPTXFSA"]
pub type NPTXFSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NPTXFD` reader - NPTXFD"]
pub type NPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFD` writer - NPTXFD"]
pub type NPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NPTXFSA"]
    #[inline(always)]
    pub fn nptxfsa(&self) -> NPTXFSA_R {
        NPTXFSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - NPTXFD"]
    #[inline(always)]
    pub fn nptxfd(&self) -> NPTXFD_R {
        NPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NPTXFSA"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfsa(&mut self) -> NPTXFSA_W<OTG_HNPTXFSIZrs> {
        NPTXFSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - NPTXFD"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfd(&mut self) -> NPTXFD_W<OTG_HNPTXFSIZrs> {
        NPTXFD_W::new(self, 16)
    }
}
#[doc = "Host mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hnptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hnptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HNPTXFSIZrs;
impl crate::RegisterSpec for OTG_HNPTXFSIZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hnptxfsiz::R`](R) reader structure"]
impl crate::Readable for OTG_HNPTXFSIZrs {}
#[doc = "`write(|w| ..)` method takes [`otg_hnptxfsiz::W`](W) writer structure"]
impl crate::Writable for OTG_HNPTXFSIZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_HNPTXFSIZ to value 0x0200_0200"]
impl crate::Resettable for OTG_HNPTXFSIZrs {
    const RESET_VALUE: u32 = 0x0200_0200;
}
