#[doc = "Register `VHBPCR` reader"]
pub type R = crate::R<VHBPCRrs>;
#[doc = "Register `VHBPCR` writer"]
pub type W = crate::W<VHBPCRrs>;
#[doc = "Field `HBP` reader - HBP"]
pub type HBP_R = crate::FieldReader<u16>;
#[doc = "Field `HBP` writer - HBP"]
pub type HBP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - HBP"]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - HBP"]
    #[inline(always)]
    #[must_use]
    pub fn hbp(&mut self) -> HBP_W<VHBPCRrs> {
        HBP_W::new(self, 0)
    }
}
#[doc = "DSI Host video HBP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vhbpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vhbpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VHBPCRrs;
impl crate::RegisterSpec for VHBPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vhbpcr::R`](R) reader structure"]
impl crate::Readable for VHBPCRrs {}
#[doc = "`write(|w| ..)` method takes [`vhbpcr::W`](W) writer structure"]
impl crate::Writable for VHBPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VHBPCR to value 0"]
impl crate::Resettable for VHBPCRrs {
    const RESET_VALUE: u32 = 0;
}
