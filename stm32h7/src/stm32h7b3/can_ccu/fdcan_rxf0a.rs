///Register `FDCAN_RXF0A` reader
pub type R = crate::R<FDCAN_RXF0Ars>;
///Register `FDCAN_RXF0A` writer
pub type W = crate::W<FDCAN_RXF0Ars>;
///Field `FA01` reader - Rx FIFO 0 Acknowledge Index
pub type FA01_R = crate::FieldReader;
///Field `FA01` writer - Rx FIFO 0 Acknowledge Index
pub type FA01_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - Rx FIFO 0 Acknowledge Index
    #[inline(always)]
    pub fn fa01(&self) -> FA01_R {
        FA01_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF0A")
            .field("fa01", &self.fa01())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Rx FIFO 0 Acknowledge Index
    #[inline(always)]
    pub fn fa01(&mut self) -> FA01_W<FDCAN_RXF0Ars> {
        FA01_W::new(self, 0)
    }
}
/**CAN Rx FIFO 0 Acknowledge Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_RXF0A)*/
pub struct FDCAN_RXF0Ars;
impl crate::RegisterSpec for FDCAN_RXF0Ars {
    type Ux = u32;
}
///`read()` method returns [`fdcan_rxf0a::R`](R) reader structure
impl crate::Readable for FDCAN_RXF0Ars {}
///`write(|w| ..)` method takes [`fdcan_rxf0a::W`](W) writer structure
impl crate::Writable for FDCAN_RXF0Ars {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_RXF0A to value 0
impl crate::Resettable for FDCAN_RXF0Ars {
    const RESET_VALUE: u32 = 0;
}
