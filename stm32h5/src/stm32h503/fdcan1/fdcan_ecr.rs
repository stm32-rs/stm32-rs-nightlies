#[doc = "Register `FDCAN_ECR` reader"]
pub type R = crate::R<FDCAN_ECRrs>;
#[doc = "Register `FDCAN_ECR` writer"]
pub type W = crate::W<FDCAN_ECRrs>;
#[doc = "Field `TEC` reader - Transmit error counter Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `REC` reader - Receive error counter Actual state of the receive error counter, values between 0 and 127."]
pub type REC_R = crate::FieldReader;
#[doc = "Field `RP` reader - Receive error passive"]
pub type RP_R = crate::BitReader;
#[doc = "Field `CEL` reader - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
pub type CEL_R = crate::FieldReader;
#[doc = "Field `CEL` writer - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
pub type CEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit error counter Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive error counter Actual state of the receive error counter, values between 0 and 127."]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive error passive"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
    #[inline(always)]
    #[must_use]
    pub fn cel(&mut self) -> CEL_W<FDCAN_ECRrs> {
        CEL_W::new(self, 16)
    }
}
#[doc = "FDCAN error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_ECRrs;
impl crate::RegisterSpec for FDCAN_ECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ecr::R`](R) reader structure"]
impl crate::Readable for FDCAN_ECRrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ecr::W`](W) writer structure"]
impl crate::Writable for FDCAN_ECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_ECR to value 0"]
impl crate::Resettable for FDCAN_ECRrs {
    const RESET_VALUE: u32 = 0;
}
