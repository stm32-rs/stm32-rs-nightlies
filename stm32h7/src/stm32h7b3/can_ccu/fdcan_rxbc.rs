///Register `FDCAN_RXBC` reader
pub type R = crate::R<FDCAN_RXBCrs>;
///Register `FDCAN_RXBC` writer
pub type W = crate::W<FDCAN_RXBCrs>;
///Field `RBSA` reader - Rx Buffer Start Address
pub type RBSA_R = crate::FieldReader<u16>;
///Field `RBSA` writer - Rx Buffer Start Address
pub type RBSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 2:15 - Rx Buffer Start Address
    #[inline(always)]
    pub fn rbsa(&self) -> RBSA_R {
        RBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXBC")
            .field("rbsa", &self.rbsa())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - Rx Buffer Start Address
    #[inline(always)]
    pub fn rbsa(&mut self) -> RBSA_W<FDCAN_RXBCrs> {
        RBSA_W::new(self, 2)
    }
}
/**FDCAN Rx Buffer Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_RXBC)*/
pub struct FDCAN_RXBCrs;
impl crate::RegisterSpec for FDCAN_RXBCrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_rxbc::R`](R) reader structure
impl crate::Readable for FDCAN_RXBCrs {}
///`write(|w| ..)` method takes [`fdcan_rxbc::W`](W) writer structure
impl crate::Writable for FDCAN_RXBCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_RXBC to value 0
impl crate::Resettable for FDCAN_RXBCrs {
    const RESET_VALUE: u32 = 0;
}
