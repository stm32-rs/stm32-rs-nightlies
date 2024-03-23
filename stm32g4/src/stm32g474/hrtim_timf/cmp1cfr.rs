#[doc = "Register `CMP1CFR` reader"]
pub type R = crate::R<CMP1CFRrs>;
#[doc = "Register `CMP1CFR` writer"]
pub type W = crate::W<CMP1CFRrs>;
#[doc = "Field `CMP1x` reader - Timerx Compare 1 value"]
pub type CMP1X_R = crate::FieldReader<u16>;
#[doc = "Field `CMP1x` writer - Timerx Compare 1 value"]
pub type CMP1X_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REPx` reader - Timerx Repetition value (aliased from HRTIM_REPx register)"]
pub type REPX_R = crate::FieldReader;
#[doc = "Field `REPx` writer - Timerx Repetition value (aliased from HRTIM_REPx register)"]
pub type REPX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    pub fn cmp1x(&self) -> CMP1X_R {
        CMP1X_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)"]
    #[inline(always)]
    pub fn repx(&self) -> REPX_R {
        REPX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1x(&mut self) -> CMP1X_W<CMP1CFRrs> {
        CMP1X_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)"]
    #[inline(always)]
    #[must_use]
    pub fn repx(&mut self) -> REPX_W<CMP1CFRrs> {
        REPX_W::new(self, 16)
    }
}
#[doc = "Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP1CFRrs;
impl crate::RegisterSpec for CMP1CFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1cfr::R`](R) reader structure"]
impl crate::Readable for CMP1CFRrs {}
#[doc = "`write(|w| ..)` method takes [`cmp1cfr::W`](W) writer structure"]
impl crate::Writable for CMP1CFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP1CFR to value 0"]
impl crate::Resettable for CMP1CFRrs {
    const RESET_VALUE: u32 = 0;
}
