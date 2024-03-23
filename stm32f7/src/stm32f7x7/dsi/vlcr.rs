#[doc = "Register `VLCR` reader"]
pub type R = crate::R<VLCRrs>;
#[doc = "Register `VLCR` writer"]
pub type W = crate::W<VLCRrs>;
#[doc = "Field `HLINE` reader - Horizontal Line duration"]
pub type HLINE_R = crate::FieldReader<u16>;
#[doc = "Field `HLINE` writer - Horizontal Line duration"]
pub type HLINE_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Horizontal Line duration"]
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal Line duration"]
    #[inline(always)]
    #[must_use]
    pub fn hline(&mut self) -> HLINE_W<VLCRrs> {
        HLINE_W::new(self, 0)
    }
}
#[doc = "DSI Host Video Line Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VLCRrs;
impl crate::RegisterSpec for VLCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vlcr::R`](R) reader structure"]
impl crate::Readable for VLCRrs {}
#[doc = "`write(|w| ..)` method takes [`vlcr::W`](W) writer structure"]
impl crate::Writable for VLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VLCR to value 0"]
impl crate::Resettable for VLCRrs {
    const RESET_VALUE: u32 = 0;
}
