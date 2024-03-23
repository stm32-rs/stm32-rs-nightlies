#[doc = "Register `JOFR2` reader"]
pub type R = crate::R<JOFR2rs>;
#[doc = "Register `JOFR2` writer"]
pub type W = crate::W<JOFR2rs>;
#[doc = "Field `JOFFSET2` reader - Data offset for injected channel 2"]
pub type JOFFSET2_R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET2` writer - Data offset for injected channel 2"]
pub type JOFFSET2_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel 2"]
    #[inline(always)]
    pub fn joffset2(&self) -> JOFFSET2_R {
        JOFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn joffset2(&mut self) -> JOFFSET2_W<JOFR2rs> {
        JOFFSET2_W::new(self, 0)
    }
}
#[doc = "injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jofr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jofr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JOFR2rs;
impl crate::RegisterSpec for JOFR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jofr2::R`](R) reader structure"]
impl crate::Readable for JOFR2rs {}
#[doc = "`write(|w| ..)` method takes [`jofr2::W`](W) writer structure"]
impl crate::Writable for JOFR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JOFR2 to value 0"]
impl crate::Resettable for JOFR2rs {
    const RESET_VALUE: u32 = 0;
}
