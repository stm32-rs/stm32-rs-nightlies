#[doc = "Register `GRXFSIZ` reader"]
pub type R = crate::R<GRXFSIZrs>;
#[doc = "Register `GRXFSIZ` writer"]
pub type W = crate::W<GRXFSIZrs>;
#[doc = "Field `RXFD` reader - RxFIFO depth"]
pub type RXFD_R = crate::FieldReader<u16>;
#[doc = "Field `RXFD` writer - RxFIFO depth"]
pub type RXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn rxfd(&mut self) -> RXFD_W<GRXFSIZrs> {
        RXFD_W::new(self, 0)
    }
}
#[doc = "OTG_HS Receive FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRXFSIZrs;
impl crate::RegisterSpec for GRXFSIZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxfsiz::R`](R) reader structure"]
impl crate::Readable for GRXFSIZrs {}
#[doc = "`write(|w| ..)` method takes [`grxfsiz::W`](W) writer structure"]
impl crate::Writable for GRXFSIZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRXFSIZ to value 0x0200"]
impl crate::Resettable for GRXFSIZrs {
    const RESET_VALUE: u32 = 0x0200;
}
