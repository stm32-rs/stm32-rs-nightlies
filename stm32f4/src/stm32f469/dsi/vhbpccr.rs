#[doc = "Register `VHBPCCR` reader"]
pub type R = crate::R<VHBPCCRrs>;
#[doc = "Register `VHBPCCR` writer"]
pub type W = crate::W<VHBPCCRrs>;
#[doc = "Field `HBP` reader - Horizontal Back-Porch duration"]
pub type HBP_R = crate::FieldReader<u16>;
#[doc = "Field `HBP` writer - Horizontal Back-Porch duration"]
pub type HBP_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Horizontal Back-Porch duration"]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Horizontal Back-Porch duration"]
    #[inline(always)]
    #[must_use]
    pub fn hbp(&mut self) -> HBP_W<VHBPCCRrs> {
        HBP_W::new(self, 0)
    }
}
#[doc = "DSI Host Video HBP Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vhbpccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vhbpccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VHBPCCRrs;
impl crate::RegisterSpec for VHBPCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vhbpccr::R`](R) reader structure"]
impl crate::Readable for VHBPCCRrs {}
#[doc = "`write(|w| ..)` method takes [`vhbpccr::W`](W) writer structure"]
impl crate::Writable for VHBPCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VHBPCCR to value 0"]
impl crate::Resettable for VHBPCCRrs {
    const RESET_VALUE: u32 = 0;
}
