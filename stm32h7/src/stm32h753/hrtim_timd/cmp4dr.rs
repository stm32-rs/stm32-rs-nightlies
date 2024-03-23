#[doc = "Register `CMP4DR` reader"]
pub type R = crate::R<CMP4DRrs>;
#[doc = "Register `CMP4DR` writer"]
pub type W = crate::W<CMP4DRrs>;
#[doc = "Field `CMP4x` reader - Timerx Compare 4 value"]
pub type CMP4X_R = crate::FieldReader<u16>;
#[doc = "Field `CMP4x` writer - Timerx Compare 4 value"]
pub type CMP4X_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 4 value"]
    #[inline(always)]
    pub fn cmp4x(&self) -> CMP4X_R {
        CMP4X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 4 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4x(&mut self) -> CMP4X_W<CMP4DRrs> {
        CMP4X_W::new(self, 0)
    }
}
#[doc = "Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP4DRrs;
impl crate::RegisterSpec for CMP4DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp4dr::R`](R) reader structure"]
impl crate::Readable for CMP4DRrs {}
#[doc = "`write(|w| ..)` method takes [`cmp4dr::W`](W) writer structure"]
impl crate::Writable for CMP4DRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP4DR to value 0"]
impl crate::Resettable for CMP4DRrs {
    const RESET_VALUE: u32 = 0;
}
