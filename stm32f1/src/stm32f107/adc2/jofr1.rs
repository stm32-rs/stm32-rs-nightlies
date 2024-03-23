#[doc = "Register `JOFR1` reader"]
pub type R = crate::R<JOFR1rs>;
#[doc = "Register `JOFR1` writer"]
pub type W = crate::W<JOFR1rs>;
#[doc = "Field `JOFFSET1` reader - Data offset for injected channel x"]
pub type JOFFSET1_R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET1` writer - Data offset for injected channel x"]
pub type JOFFSET1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset1(&self) -> JOFFSET1_R {
        JOFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    #[must_use]
    pub fn joffset1(&mut self) -> JOFFSET1_W<JOFR1rs> {
        JOFFSET1_W::new(self, 0)
    }
}
#[doc = "injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JOFR1rs;
impl crate::RegisterSpec for JOFR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jofr1::R`](R) reader structure"]
impl crate::Readable for JOFR1rs {}
#[doc = "`write(|w| ..)` method takes [`jofr1::W`](W) writer structure"]
impl crate::Writable for JOFR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JOFR1 to value 0"]
impl crate::Resettable for JOFR1rs {
    const RESET_VALUE: u32 = 0;
}
