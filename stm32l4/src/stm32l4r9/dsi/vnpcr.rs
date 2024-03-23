#[doc = "Register `VNPCR` reader"]
pub type R = crate::R<VNPCRrs>;
#[doc = "Register `VNPCR` writer"]
pub type W = crate::W<VNPCRrs>;
#[doc = "Field `NPSIZE` reader - Null Packet Size"]
pub type NPSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `NPSIZE` writer - Null Packet Size"]
pub type NPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Null Packet Size"]
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Null Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn npsize(&mut self) -> NPSIZE_W<VNPCRrs> {
        NPSIZE_W::new(self, 0)
    }
}
#[doc = "DSI Host Video Null Packet Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vnpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vnpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VNPCRrs;
impl crate::RegisterSpec for VNPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vnpcr::R`](R) reader structure"]
impl crate::Readable for VNPCRrs {}
#[doc = "`write(|w| ..)` method takes [`vnpcr::W`](W) writer structure"]
impl crate::Writable for VNPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VNPCR to value 0"]
impl crate::Resettable for VNPCRrs {
    const RESET_VALUE: u32 = 0;
}
