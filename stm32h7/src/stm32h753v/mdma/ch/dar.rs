#[doc = "Register `DAR` reader"]
pub type R = crate::R<DARrs>;
#[doc = "Register `DAR` writer"]
pub type W = crate::W<DARrs>;
#[doc = "Field `DAR` reader - Destination adr base"]
pub type DAR_R = crate::FieldReader<u32>;
#[doc = "Field `DAR` writer - Destination adr base"]
pub type DAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination adr base"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination adr base"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<DARrs> {
        DAR_W::new(self, 0)
    }
}
#[doc = "MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DARrs;
impl crate::RegisterSpec for DARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar::R`](R) reader structure"]
impl crate::Readable for DARrs {}
#[doc = "`write(|w| ..)` method takes [`dar::W`](W) writer structure"]
impl crate::Writable for DARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAR to value 0"]
impl crate::Resettable for DARrs {
    const RESET_VALUE: u32 = 0;
}
