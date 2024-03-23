#[doc = "Register `FIFOR12` reader"]
pub type R = crate::R<FIFOR12rs>;
#[doc = "Register `FIFOR12` writer"]
pub type W = crate::W<FIFOR12rs>;
#[doc = "Field `FIFODATA` reader - Receive and transmit FIFO data"]
pub type FIFODATA_R = crate::FieldReader<u32>;
#[doc = "Field `FIFODATA` writer - Receive and transmit FIFO data"]
pub type FIFODATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data"]
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data"]
    #[inline(always)]
    #[must_use]
    pub fn fifodata(&mut self) -> FIFODATA_W<FIFOR12rs> {
        FIFODATA_W::new(self, 0)
    }
}
#[doc = "data FIFO register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFOR12rs;
impl crate::RegisterSpec for FIFOR12rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifor12::R`](R) reader structure"]
impl crate::Readable for FIFOR12rs {}
#[doc = "`write(|w| ..)` method takes [`fifor12::W`](W) writer structure"]
impl crate::Writable for FIFOR12rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOR12 to value 0"]
impl crate::Resettable for FIFOR12rs {
    const RESET_VALUE: u32 = 0;
}
