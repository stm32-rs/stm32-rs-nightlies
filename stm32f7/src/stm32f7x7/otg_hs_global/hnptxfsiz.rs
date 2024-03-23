#[doc = "Register `HNPTXFSIZ` reader"]
pub type R = crate::R<HNPTXFSIZrs>;
#[doc = "Register `HNPTXFSIZ` writer"]
pub type W = crate::W<HNPTXFSIZrs>;
#[doc = "Field `NPTXFSA` reader - Nonperiodic transmit RAM start address"]
pub type NPTXFSA_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFSA` writer - Nonperiodic transmit RAM start address"]
pub type NPTXFSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NPTXFD` reader - Nonperiodic TxFIFO depth"]
pub type NPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFD` writer - Nonperiodic TxFIFO depth"]
pub type NPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Nonperiodic transmit RAM start address"]
    #[inline(always)]
    pub fn nptxfsa(&self) -> NPTXFSA_R {
        NPTXFSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Nonperiodic TxFIFO depth"]
    #[inline(always)]
    pub fn nptxfd(&self) -> NPTXFD_R {
        NPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Nonperiodic transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfsa(&mut self) -> NPTXFSA_W<HNPTXFSIZrs> {
        NPTXFSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Nonperiodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfd(&mut self) -> NPTXFD_W<HNPTXFSIZrs> {
        NPTXFD_W::new(self, 16)
    }
}
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hnptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hnptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HNPTXFSIZrs;
impl crate::RegisterSpec for HNPTXFSIZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hnptxfsiz::R`](R) reader structure"]
impl crate::Readable for HNPTXFSIZrs {}
#[doc = "`write(|w| ..)` method takes [`hnptxfsiz::W`](W) writer structure"]
impl crate::Writable for HNPTXFSIZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HNPTXFSIZ to value 0x0200"]
impl crate::Resettable for HNPTXFSIZrs {
    const RESET_VALUE: u32 = 0x0200;
}
