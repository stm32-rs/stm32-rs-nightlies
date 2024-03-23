#[doc = "Register `JOFR%s` reader"]
pub type R = crate::R<JOFRrs>;
#[doc = "Register `JOFR%s` writer"]
pub type W = crate::W<JOFRrs>;
#[doc = "Field `JOFFSET` reader - Data offset for injected channel x"]
pub type JOFFSET_R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET` writer - Data offset for injected channel x"]
pub type JOFFSET_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset(&self) -> JOFFSET_R {
        JOFFSET_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    #[must_use]
    pub fn joffset(&mut self) -> JOFFSET_W<JOFRrs> {
        JOFFSET_W::new(self, 0)
    }
}
#[doc = "injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JOFRrs;
impl crate::RegisterSpec for JOFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jofr::R`](R) reader structure"]
impl crate::Readable for JOFRrs {}
#[doc = "`write(|w| ..)` method takes [`jofr::W`](W) writer structure"]
impl crate::Writable for JOFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JOFR%s to value 0"]
impl crate::Resettable for JOFRrs {
    const RESET_VALUE: u32 = 0;
}
