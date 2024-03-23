#[doc = "Register `FCR` reader"]
pub type R = crate::R<FCRrs>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCRrs>;
#[doc = "Field `DL` reader - Data length"]
pub type DL_R = crate::FieldReader<u32>;
#[doc = "Field `DL` writer - Data length"]
pub type DL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data length"]
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data length"]
    #[inline(always)]
    #[must_use]
    pub fn dl(&mut self) -> DL_W<FCRrs> {
        DL_W::new(self, 0)
    }
}
#[doc = "flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FCRrs {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCRrs {
    const RESET_VALUE: u32 = 0;
}
