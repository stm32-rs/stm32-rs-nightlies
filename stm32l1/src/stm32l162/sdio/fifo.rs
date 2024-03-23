#[doc = "Register `FIFO` reader"]
pub type R = crate::R<FIFOrs>;
#[doc = "Register `FIFO` writer"]
pub type W = crate::W<FIFOrs>;
#[doc = "Field `FIF0Data` reader - FIF0Data"]
pub type FIF0DATA_R = crate::FieldReader<u32>;
#[doc = "Field `FIF0Data` writer - FIF0Data"]
pub type FIF0DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIF0Data"]
    #[inline(always)]
    pub fn fif0data(&self) -> FIF0DATA_R {
        FIF0DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIF0Data"]
    #[inline(always)]
    #[must_use]
    pub fn fif0data(&mut self) -> FIF0DATA_W<FIFOrs> {
        FIF0DATA_W::new(self, 0)
    }
}
#[doc = "data FIFO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFOrs;
impl crate::RegisterSpec for FIFOrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo::R`](R) reader structure"]
impl crate::Readable for FIFOrs {}
#[doc = "`write(|w| ..)` method takes [`fifo::W`](W) writer structure"]
impl crate::Writable for FIFOrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO to value 0"]
impl crate::Resettable for FIFOrs {
    const RESET_VALUE: u32 = 0;
}
