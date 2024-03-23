#[doc = "Register `ABR` reader"]
pub type R = crate::R<ABRrs>;
#[doc = "Register `ABR` writer"]
pub type W = crate::W<ABRrs>;
#[doc = "Field `TIMEOUT` reader - Timeout period"]
pub type TIMEOUT_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUT` writer - Timeout period"]
pub type TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timeout period"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout period"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<ABRrs> {
        TIMEOUT_W::new(self, 0)
    }
}
#[doc = "alternate bytes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`abr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`abr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ABRrs;
impl crate::RegisterSpec for ABRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`abr::R`](R) reader structure"]
impl crate::Readable for ABRrs {}
#[doc = "`write(|w| ..)` method takes [`abr::W`](W) writer structure"]
impl crate::Writable for ABRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABR to value 0"]
impl crate::Resettable for ABRrs {
    const RESET_VALUE: u32 = 0;
}
