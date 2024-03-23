#[doc = "Register `MAR` reader"]
pub type R = crate::R<MARrs>;
#[doc = "Register `MAR` writer"]
pub type W = crate::W<MARrs>;
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
    pub fn ma(&mut self) -> MA_W<MARrs> {
        MA_W::new(self, 0)
    }
}
#[doc = "DMA channel 1 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MARrs;
impl crate::RegisterSpec for MARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mar::R`](R) reader structure"]
impl crate::Readable for MARrs {}
#[doc = "`write(|w| ..)` method takes [`mar::W`](W) writer structure"]
impl crate::Writable for MARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAR to value 0"]
impl crate::Resettable for MARrs {
    const RESET_VALUE: u32 = 0;
}
