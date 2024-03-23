#[doc = "Register `FDCAN_RXF0A` reader"]
pub type R = crate::R<FDCAN_RXF0Ars>;
#[doc = "Register `FDCAN_RXF0A` writer"]
pub type W = crate::W<FDCAN_RXF0Ars>;
#[doc = "Field `FA01` reader - Rx FIFO 0 Acknowledge Index"]
pub type FA01_R = crate::FieldReader;
#[doc = "Field `FA01` writer - Rx FIFO 0 Acknowledge Index"]
pub type FA01_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    pub fn fa01(&self) -> FA01_R {
        FA01_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    #[must_use]
    pub fn fa01(&mut self) -> FA01_W<FDCAN_RXF0Ars> {
        FA01_W::new(self, 0)
    }
}
#[doc = "CAN Rx FIFO 0 Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf0a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf0a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
