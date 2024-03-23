#[doc = "Register `OAR` reader"]
pub type R = crate::R<OARrs>;
#[doc = "Register `OAR` writer"]
pub type W = crate::W<OARrs>;
#[doc = "Field `OA` reader - Own address"]
pub type OA_R = crate::FieldReader;
#[doc = "Field `OA` writer - Own address"]
pub type OA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Own address"]
    #[inline(always)]
    pub fn oa(&self) -> OA_R {
        OA_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Own address"]
    #[inline(always)]
    #[must_use]
    pub fn oa(&mut self) -> OA_W<OARrs> {
        OA_W::new(self, 0)
    }
}
#[doc = "CEC own address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OARrs;
impl crate::RegisterSpec for OARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar::R`](R) reader structure"]
impl crate::Readable for OARrs {}
#[doc = "`write(|w| ..)` method takes [`oar::W`](W) writer structure"]
impl crate::Writable for OARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OAR to value 0"]
impl crate::Resettable for OARrs {
    const RESET_VALUE: u32 = 0;
}
