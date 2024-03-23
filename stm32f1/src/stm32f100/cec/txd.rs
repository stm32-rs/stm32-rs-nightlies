#[doc = "Register `TXD` reader"]
pub type R = crate::R<TXDrs>;
#[doc = "Register `TXD` writer"]
pub type W = crate::W<TXDrs>;
#[doc = "Field `TXD` reader - Tx Data register"]
pub type TXD_R = crate::FieldReader;
#[doc = "Field `TXD` writer - Tx Data register"]
pub type TXD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Tx Data register"]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Tx Data register"]
    #[inline(always)]
    #[must_use]
    pub fn txd(&mut self) -> TXD_W<TXDrs> {
        TXD_W::new(self, 0)
    }
}
#[doc = "CEC Tx data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDrs;
impl crate::RegisterSpec for TXDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txd::R`](R) reader structure"]
impl crate::Readable for TXDrs {}
#[doc = "`write(|w| ..)` method takes [`txd::W`](W) writer structure"]
impl crate::Writable for TXDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXD to value 0"]
impl crate::Resettable for TXDrs {
    const RESET_VALUE: u32 = 0;
}
