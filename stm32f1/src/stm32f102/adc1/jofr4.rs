#[doc = "Register `JOFR4` reader"]
pub type R = crate::R<JOFR4rs>;
#[doc = "Register `JOFR4` writer"]
pub type W = crate::W<JOFR4rs>;
#[doc = "Field `JOFFSET4` reader - Data offset for injected channel x"]
pub type JOFFSET4_R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET4` writer - Data offset for injected channel x"]
pub type JOFFSET4_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset4(&self) -> JOFFSET4_R {
        JOFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    #[must_use]
    pub fn joffset4(&mut self) -> JOFFSET4_W<JOFR4rs> {
        JOFFSET4_W::new(self, 0)
    }
}
#[doc = "injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JOFR4rs;
impl crate::RegisterSpec for JOFR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jofr4::R`](R) reader structure"]
impl crate::Readable for JOFR4rs {}
#[doc = "`write(|w| ..)` method takes [`jofr4::W`](W) writer structure"]
impl crate::Writable for JOFR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JOFR4 to value 0"]
impl crate::Resettable for JOFR4rs {
    const RESET_VALUE: u32 = 0;
}
