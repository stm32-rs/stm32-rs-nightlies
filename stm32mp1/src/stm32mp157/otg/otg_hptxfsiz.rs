#[doc = "Register `OTG_HPTXFSIZ` reader"]
pub type R = crate::R<OTG_HPTXFSIZrs>;
#[doc = "Register `OTG_HPTXFSIZ` writer"]
pub type W = crate::W<OTG_HPTXFSIZrs>;
#[doc = "Field `PTXSA` reader - PTXSA"]
pub type PTXSA_R = crate::FieldReader<u16>;
#[doc = "Field `PTXSA` writer - PTXSA"]
pub type PTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PTXFSIZ` reader - PTXFSIZ"]
pub type PTXFSIZ_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFSIZ` writer - PTXFSIZ"]
pub type PTXFSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PTXSA"]
    #[inline(always)]
    pub fn ptxsa(&self) -> PTXSA_R {
        PTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PTXFSIZ"]
    #[inline(always)]
    pub fn ptxfsiz(&self) -> PTXFSIZ_R {
        PTXFSIZ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PTXSA"]
    #[inline(always)]
    #[must_use]
    pub fn ptxsa(&mut self) -> PTXSA_W<OTG_HPTXFSIZrs> {
        PTXSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - PTXFSIZ"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfsiz(&mut self) -> PTXFSIZ_W<OTG_HPTXFSIZrs> {
        PTXFSIZ_W::new(self, 16)
    }
}
#[doc = "OTG host periodic transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HPTXFSIZrs;
impl crate::RegisterSpec for OTG_HPTXFSIZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hptxfsiz::R`](R) reader structure"]
impl crate::Readable for OTG_HPTXFSIZrs {}
#[doc = "`write(|w| ..)` method takes [`otg_hptxfsiz::W`](W) writer structure"]
impl crate::Writable for OTG_HPTXFSIZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_HPTXFSIZ to value 0x0200_0400"]
impl crate::Resettable for OTG_HPTXFSIZrs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
