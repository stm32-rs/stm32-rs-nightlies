#[doc = "Register `CMAR2` reader"]
pub type R = crate::R<CMAR2rs>;
#[doc = "Register `CMAR2` writer"]
pub type W = crate::W<CMAR2rs>;
#[doc = "Field `MA` reader - peripheral address"]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - peripheral address"]
pub type MA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - peripheral address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - peripheral address"]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<CMAR2rs> {
        MA_W::new(self, 0)
    }
}
#[doc = "channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMAR2rs;
impl crate::RegisterSpec for CMAR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmar2::R`](R) reader structure"]
impl crate::Readable for CMAR2rs {}
#[doc = "`write(|w| ..)` method takes [`cmar2::W`](W) writer structure"]
impl crate::Writable for CMAR2rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMAR2 to value 0"]
impl crate::Resettable for CMAR2rs {
    const RESET_VALUE: u32 = 0;
}
