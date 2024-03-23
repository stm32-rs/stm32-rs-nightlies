#[doc = "Register `FDCAN_RXF0A` reader"]
pub type R = crate::R<FDCAN_RXF0Ars>;
#[doc = "Register `FDCAN_RXF0A` writer"]
pub type W = crate::W<FDCAN_RXF0Ars>;
#[doc = "Field `F0AI` reader - F0AI"]
pub type F0AI_R = crate::FieldReader;
#[doc = "Field `F0AI` writer - F0AI"]
pub type F0AI_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - F0AI"]
    #[inline(always)]
    pub fn f0ai(&self) -> F0AI_R {
        F0AI_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - F0AI"]
    #[inline(always)]
    #[must_use]
    pub fn f0ai(&mut self) -> F0AI_W<FDCAN_RXF0Ars> {
        F0AI_W::new(self, 0)
    }
}
#[doc = "FDCAN Rx FIFO 0 acknowledge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf0a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf0a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_RXF0Ars;
impl crate::RegisterSpec for FDCAN_RXF0Ars {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf0a::R`](R) reader structure"]
impl crate::Readable for FDCAN_RXF0Ars {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxf0a::W`](W) writer structure"]
impl crate::Writable for FDCAN_RXF0Ars {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_RXF0A to value 0"]
impl crate::Resettable for FDCAN_RXF0Ars {
    const RESET_VALUE: u32 = 0;
}
