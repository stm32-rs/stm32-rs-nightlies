#[doc = "Register `CMP2AR` reader"]
pub type R = crate::R<CMP2ARrs>;
#[doc = "Register `CMP2AR` writer"]
pub type W = crate::W<CMP2ARrs>;
#[doc = "Field `CMP2x` reader - Timerx Compare 2 value"]
pub type CMP2X_R = crate::FieldReader<u16>;
#[doc = "Field `CMP2x` writer - Timerx Compare 2 value"]
pub type CMP2X_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 2 value"]
    #[inline(always)]
    pub fn cmp2x(&self) -> CMP2X_R {
        CMP2X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 2 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2x(&mut self) -> CMP2X_W<CMP2ARrs> {
        CMP2X_W::new(self, 0)
    }
}
#[doc = "Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP2ARrs;
impl crate::RegisterSpec for CMP2ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp2ar::R`](R) reader structure"]
impl crate::Readable for CMP2ARrs {}
#[doc = "`write(|w| ..)` method takes [`cmp2ar::W`](W) writer structure"]
impl crate::Writable for CMP2ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP2AR to value 0"]
impl crate::Resettable for CMP2ARrs {
    const RESET_VALUE: u32 = 0;
}
