#[doc = "Register `CMAR1` reader"]
pub type R = crate::R<CMAR1rs>;
#[doc = "Register `CMAR1` writer"]
pub type W = crate::W<CMAR1rs>;
#[doc = "Field `MA` reader - Memory address"]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory address"]
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<CMAR1rs> {
        MA_W::new(self, 0)
    }
}
#[doc = "channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMAR1rs;
impl crate::RegisterSpec for CMAR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmar1::R`](R) reader structure"]
impl crate::Readable for CMAR1rs {}
#[doc = "`write(|w| ..)` method takes [`cmar1::W`](W) writer structure"]
impl crate::Writable for CMAR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMAR1 to value 0"]
impl crate::Resettable for CMAR1rs {
    const RESET_VALUE: u32 = 0;
}
