///Register `ECR` reader
pub type R = crate::R<ECRrs>;
///Register `ECR` writer
pub type W = crate::W<ECRrs>;
///Field `TEC` reader - Transmit error counter Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented.
pub type TEC_R = crate::FieldReader;
///Field `REC` reader - Receive error counter Actual state of the receive error counter, values between 0 and 127.
pub type REC_R = crate::FieldReader;
///Field `RP` reader - Receive error passive
pub type RP_R = crate::BitReader;
/**Field `CEL` reader - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\[ELO\]. Access type is RX: reset on read.

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type CEL_R = crate::FieldReader;
///Field `CEL` writer - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\[ELO\]. Access type is RX: reset on read.
pub type CEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Transmit error counter Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented.
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - Receive error counter Actual state of the receive error counter, values between 0 and 127.
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 15 - Receive error passive
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\[ELO\]. Access type is RX: reset on read.
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECR")
            .field("tec", &self.tec())
            .field("rec", &self.rec())
            .field("rp", &self.rp())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\[ELO\]. Access type is RX: reset on read.
    #[inline(always)]
    pub fn cel(&mut self) -> CEL_W<'_, ECRrs> {
        CEL_W::new(self, 16)
    }
}
/**FDCAN error counter register

You can [`read`](crate::Reg::read) this register and get [`ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#FDCAN1:ECR)*/
pub struct ECRrs;
impl crate::RegisterSpec for ECRrs {
    type Ux = u32;
}
///`read()` method returns [`ecr::R`](R) reader structure
impl crate::Readable for ECRrs {}
///`write(|w| ..)` method takes [`ecr::W`](W) writer structure
impl crate::Writable for ECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECR to value 0
impl crate::Resettable for ECRrs {}
