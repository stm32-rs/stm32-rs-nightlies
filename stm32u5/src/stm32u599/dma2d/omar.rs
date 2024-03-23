#[doc = "Register `OMAR` reader"]
pub type R = crate::R<OMARrs>;
#[doc = "Register `OMAR` writer"]
pub type W = crate::W<OMARrs>;
#[doc = "Field `MA` reader - Memory Address"]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory Address"]
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory Address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory Address"]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<OMARrs> {
        MA_W::new(self, 0)
    }
}
#[doc = "output memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OMARrs;
impl crate::RegisterSpec for OMARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`omar::R`](R) reader structure"]
impl crate::Readable for OMARrs {}
#[doc = "`write(|w| ..)` method takes [`omar::W`](W) writer structure"]
impl crate::Writable for OMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OMAR to value 0"]
impl crate::Resettable for OMARrs {
    const RESET_VALUE: u32 = 0;
}
