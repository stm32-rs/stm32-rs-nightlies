#[doc = "Register `CMP1CR` reader"]
pub type R = crate::R<CMP1CRrs>;
#[doc = "Register `CMP1CR` writer"]
pub type W = crate::W<CMP1CRrs>;
#[doc = "Field `CMP1x` reader - Timerx Compare 1 value"]
pub type CMP1X_R = crate::FieldReader<u16>;
#[doc = "Field `CMP1x` writer - Timerx Compare 1 value"]
pub type CMP1X_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    pub fn cmp1x(&self) -> CMP1X_R {
        CMP1X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1x(&mut self) -> CMP1X_W<CMP1CRrs> {
        CMP1X_W::new(self, 0)
    }
}
#[doc = "Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP1CRrs;
impl crate::RegisterSpec for CMP1CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1cr::R`](R) reader structure"]
impl crate::Readable for CMP1CRrs {}
#[doc = "`write(|w| ..)` method takes [`cmp1cr::W`](W) writer structure"]
impl crate::Writable for CMP1CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP1CR to value 0"]
impl crate::Resettable for CMP1CRrs {
    const RESET_VALUE: u32 = 0;
}
