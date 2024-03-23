#[doc = "Register `MDR` reader"]
pub type R = crate::R<MDRrs>;
#[doc = "Register `MDR` writer"]
pub type W = crate::W<MDRrs>;
#[doc = "Field `MDR` reader - Mask data"]
pub type MDR_R = crate::FieldReader<u32>;
#[doc = "Field `MDR` writer - Mask data"]
pub type MDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask data"]
    #[inline(always)]
    pub fn mdr(&self) -> MDR_R {
        MDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask data"]
    #[inline(always)]
    #[must_use]
    pub fn mdr(&mut self) -> MDR_W<MDRrs> {
        MDR_W::new(self, 0)
    }
}
#[doc = "MDMA channel x Mask Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDRrs;
impl crate::RegisterSpec for MDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdr::R`](R) reader structure"]
impl crate::Readable for MDRrs {}
#[doc = "`write(|w| ..)` method takes [`mdr::W`](W) writer structure"]
impl crate::Writable for MDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDR to value 0"]
impl crate::Resettable for MDRrs {
    const RESET_VALUE: u32 = 0;
}
