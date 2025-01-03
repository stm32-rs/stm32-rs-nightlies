///Register `FDCAN_TXESC` reader
pub type R = crate::R<FDCAN_TXESCrs>;
///Register `FDCAN_TXESC` writer
pub type W = crate::W<FDCAN_TXESCrs>;
///Field `TBDS` reader - Tx Buffer Data Field Size:
pub type TBDS_R = crate::FieldReader;
///Field `TBDS` writer - Tx Buffer Data Field Size:
pub type TBDS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Tx Buffer Data Field Size:
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXESC")
            .field("tbds", &self.tbds())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Tx Buffer Data Field Size:
    #[inline(always)]
    pub fn tbds(&mut self) -> TBDS_W<FDCAN_TXESCrs> {
        TBDS_W::new(self, 0)
    }
}
/**FDCAN Tx Buffer Element Size Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#CAN_CCU:FDCAN_TXESC)*/
pub struct FDCAN_TXESCrs;
impl crate::RegisterSpec for FDCAN_TXESCrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txesc::R`](R) reader structure
impl crate::Readable for FDCAN_TXESCrs {}
///`write(|w| ..)` method takes [`fdcan_txesc::W`](W) writer structure
impl crate::Writable for FDCAN_TXESCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_TXESC to value 0
impl crate::Resettable for FDCAN_TXESCrs {
    const RESET_VALUE: u32 = 0;
}
