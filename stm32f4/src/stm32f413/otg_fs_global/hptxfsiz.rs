#[doc = "Register `HPTXFSIZ` reader"]
pub type R = crate::R<HPTXFSIZrs>;
#[doc = "Register `HPTXFSIZ` writer"]
pub type W = crate::W<HPTXFSIZrs>;
#[doc = "Field `PTXSA` reader - Host periodic TxFIFO start address"]
pub type PTXSA_R = crate::FieldReader<u16>;
#[doc = "Field `PTXSA` writer - Host periodic TxFIFO start address"]
pub type PTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PTXFSIZ` reader - Host periodic TxFIFO depth"]
pub type PTXFSIZ_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFSIZ` writer - Host periodic TxFIFO depth"]
pub type PTXFSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&self) -> PTXSA_R {
        PTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfsiz(&self) -> PTXFSIZ_R {
        PTXFSIZ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    #[must_use]
    pub fn ptxsa(&mut self) -> PTXSA_W<HPTXFSIZrs> {
        PTXSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfsiz(&mut self) -> PTXFSIZ_W<HPTXFSIZrs> {
        PTXFSIZ_W::new(self, 16)
    }
}
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPTXFSIZrs;
impl crate::RegisterSpec for HPTXFSIZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxfsiz::R`](R) reader structure"]
impl crate::Readable for HPTXFSIZrs {}
#[doc = "`write(|w| ..)` method takes [`hptxfsiz::W`](W) writer structure"]
impl crate::Writable for HPTXFSIZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPTXFSIZ to value 0x0200_0600"]
impl crate::Resettable for HPTXFSIZrs {
    const RESET_VALUE: u32 = 0x0200_0600;
}
