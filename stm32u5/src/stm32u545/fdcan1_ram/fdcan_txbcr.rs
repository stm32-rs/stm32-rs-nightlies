///Register `FDCAN_TXBCR` reader
pub type R = crate::R<FDCAN_TXBCRrs>;
///Register `FDCAN_TXBCR` writer
pub type W = crate::W<FDCAN_TXBCRrs>;
///Field `CR` reader - Cancellation Request
pub type CR_R = crate::FieldReader;
///Field `CR` writer - Cancellation Request
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Cancellation Request
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBCR")
            .field("cr", &self.cr())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Cancellation Request
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<FDCAN_TXBCRrs> {
        CR_W::new(self, 0)
    }
}
/**FDCAN Tx Buffer Cancellation Request Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#FDCAN1_RAM:FDCAN_TXBCR)*/
pub struct FDCAN_TXBCRrs;
impl crate::RegisterSpec for FDCAN_TXBCRrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbcr::R`](R) reader structure
impl crate::Readable for FDCAN_TXBCRrs {}
///`write(|w| ..)` method takes [`fdcan_txbcr::W`](W) writer structure
impl crate::Writable for FDCAN_TXBCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_TXBCR to value 0
impl crate::Resettable for FDCAN_TXBCRrs {
    const RESET_VALUE: u32 = 0;
}
