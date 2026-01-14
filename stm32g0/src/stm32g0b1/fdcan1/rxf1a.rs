///Register `RXF1A` reader
pub type R = crate::R<RXF1Ars>;
///Register `RXF1A` writer
pub type W = crate::W<RXF1Ars>;
///Field `F1AI` reader - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO1 get index RXF1S\[F1GI\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\[F1FL\].
pub type F1AI_R = crate::FieldReader;
///Field `F1AI` writer - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO1 get index RXF1S\[F1GI\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\[F1FL\].
pub type F1AI_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO1 get index RXF1S\[F1GI\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\[F1FL\].
    #[inline(always)]
    pub fn f1ai(&self) -> F1AI_R {
        F1AI_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF1A").field("f1ai", &self.f1ai()).finish()
    }
}
impl W {
    ///Bits 0:2 - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO1 get index RXF1S\[F1GI\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\[F1FL\].
    #[inline(always)]
    pub fn f1ai(&mut self) -> F1AI_W<'_, RXF1Ars> {
        F1AI_W::new(self, 0)
    }
}
/**FDCAN Rx FIFO 1 acknowledge register

You can [`read`](crate::Reg::read) this register and get [`rxf1a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#FDCAN1:RXF1A)*/
pub struct RXF1Ars;
impl crate::RegisterSpec for RXF1Ars {
    type Ux = u32;
}
///`read()` method returns [`rxf1a::R`](R) reader structure
impl crate::Readable for RXF1Ars {}
///`write(|w| ..)` method takes [`rxf1a::W`](W) writer structure
impl crate::Writable for RXF1Ars {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RXF1A to value 0
impl crate::Resettable for RXF1Ars {}
