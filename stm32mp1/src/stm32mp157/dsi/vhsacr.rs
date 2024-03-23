#[doc = "Register `VHSACR` reader"]
pub type R = crate::R<VHSACRrs>;
#[doc = "Register `VHSACR` writer"]
pub type W = crate::W<VHSACRrs>;
#[doc = "Field `HSA` reader - HSA"]
pub type HSA_R = crate::FieldReader<u16>;
#[doc = "Field `HSA` writer - HSA"]
pub type HSA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - HSA"]
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - HSA"]
    #[inline(always)]
    #[must_use]
    pub fn hsa(&mut self) -> HSA_W<VHSACRrs> {
        HSA_W::new(self, 0)
    }
}
#[doc = "DSI Host video HSA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vhsacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vhsacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VHSACRrs;
impl crate::RegisterSpec for VHSACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vhsacr::R`](R) reader structure"]
impl crate::Readable for VHSACRrs {}
#[doc = "`write(|w| ..)` method takes [`vhsacr::W`](W) writer structure"]
impl crate::Writable for VHSACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VHSACR to value 0"]
impl crate::Resettable for VHSACRrs {
    const RESET_VALUE: u32 = 0;
}
