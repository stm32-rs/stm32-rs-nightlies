#[doc = "Register `CMP3ER` reader"]
pub type R = crate::R<CMP3ERrs>;
#[doc = "Register `CMP3ER` writer"]
pub type W = crate::W<CMP3ERrs>;
#[doc = "Field `CMP3x` reader - Timerx Compare 3 value"]
pub type CMP3X_R = crate::FieldReader<u16>;
#[doc = "Field `CMP3x` writer - Timerx Compare 3 value"]
pub type CMP3X_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    pub fn cmp3x(&self) -> CMP3X_R {
        CMP3X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3x(&mut self) -> CMP3X_W<CMP3ERrs> {
        CMP3X_W::new(self, 0)
    }
}
#[doc = "Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3er::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3er::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP3ERrs;
impl crate::RegisterSpec for CMP3ERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp3er::R`](R) reader structure"]
impl crate::Readable for CMP3ERrs {}
#[doc = "`write(|w| ..)` method takes [`cmp3er::W`](W) writer structure"]
impl crate::Writable for CMP3ERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP3ER to value 0"]
impl crate::Resettable for CMP3ERrs {
    const RESET_VALUE: u32 = 0;
}
