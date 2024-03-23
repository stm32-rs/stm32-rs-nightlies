#[doc = "Register `CMP4BR` reader"]
pub type R = crate::R<CMP4BRrs>;
#[doc = "Register `CMP4BR` writer"]
pub type W = crate::W<CMP4BRrs>;
#[doc = "Field `CMP4x` reader - Timerx Compare 4 value"]
pub type CMP4X_R = crate::FieldReader<u16>;
#[doc = "Field `CMP4x` writer - Timerx Compare 4 value"]
pub type CMP4X_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
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
    pub fn cmp4x(&mut self) -> CMP4X_W<CMP4BRrs> {
        CMP4X_W::new(self, 0)
    }
}
#[doc = "Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP4BRrs;
impl crate::RegisterSpec for CMP4BRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp4br::R`](R) reader structure"]
impl crate::Readable for CMP4BRrs {}
#[doc = "`write(|w| ..)` method takes [`cmp4br::W`](W) writer structure"]
impl crate::Writable for CMP4BRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP4BR to value 0"]
impl crate::Resettable for CMP4BRrs {
    const RESET_VALUE: u32 = 0;
}
