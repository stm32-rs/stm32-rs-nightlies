#[doc = "Register `JOFR3` reader"]
pub type R = crate::R<JOFR3rs>;
#[doc = "Register `JOFR3` writer"]
pub type W = crate::W<JOFR3rs>;
#[doc = "Field `JOFFSET3` reader - Data offset for injected channel 3"]
pub type JOFFSET3_R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET3` writer - Data offset for injected channel 3"]
pub type JOFFSET3_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel 3"]
    #[inline(always)]
    pub fn joffset3(&self) -> JOFFSET3_R {
        JOFFSET3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn joffset3(&mut self) -> JOFFSET3_W<JOFR3rs> {
        JOFFSET3_W::new(self, 0)
    }
}
#[doc = "injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JOFR3rs;
impl crate::RegisterSpec for JOFR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jofr3::R`](R) reader structure"]
impl crate::Readable for JOFR3rs {}
#[doc = "`write(|w| ..)` method takes [`jofr3::W`](W) writer structure"]
impl crate::Writable for JOFR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JOFR3 to value 0"]
impl crate::Resettable for JOFR3rs {
    const RESET_VALUE: u32 = 0;
}
