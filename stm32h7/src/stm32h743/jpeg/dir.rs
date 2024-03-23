#[doc = "Register `DIR` writer"]
pub type W = crate::W<DIRrs>;
#[doc = "Field `DATAIN` writer - Data Input FIFO Input FIFO data register."]
pub type DATAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Data Input FIFO Input FIFO data register."]
    #[inline(always)]
    #[must_use]
    pub fn datain(&mut self) -> DATAIN_W<DIRrs> {
        DATAIN_W::new(self, 0)
    }
}
#[doc = "JPEG data input register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIRrs;
impl crate::RegisterSpec for DIRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dir::W`](W) writer structure"]
impl crate::Writable for DIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIR to value 0"]
impl crate::Resettable for DIRrs {
    const RESET_VALUE: u32 = 0;
}
