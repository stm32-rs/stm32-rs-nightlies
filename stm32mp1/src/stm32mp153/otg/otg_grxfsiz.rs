#[doc = "Register `OTG_GRXFSIZ` reader"]
pub type R = crate::R<OTG_GRXFSIZrs>;
#[doc = "Register `OTG_GRXFSIZ` writer"]
pub type W = crate::W<OTG_GRXFSIZrs>;
#[doc = "Field `RXFD` reader - RXFD"]
pub type RXFD_R = crate::FieldReader<u16>;
#[doc = "Field `RXFD` writer - RXFD"]
pub type RXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RXFD"]
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RXFD"]
    #[inline(always)]
    #[must_use]
    pub fn rxfd(&mut self) -> RXFD_W<OTG_GRXFSIZrs> {
        RXFD_W::new(self, 0)
    }
}
#[doc = "The application can program the RAM size that must be allocated to the Rx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_grxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_grxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_GRXFSIZrs;
impl crate::RegisterSpec for OTG_GRXFSIZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_grxfsiz::R`](R) reader structure"]
impl crate::Readable for OTG_GRXFSIZrs {}
#[doc = "`write(|w| ..)` method takes [`otg_grxfsiz::W`](W) writer structure"]
impl crate::Writable for OTG_GRXFSIZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_GRXFSIZ to value 0x0400"]
impl crate::Resettable for OTG_GRXFSIZrs {
    const RESET_VALUE: u32 = 0x0400;
}
