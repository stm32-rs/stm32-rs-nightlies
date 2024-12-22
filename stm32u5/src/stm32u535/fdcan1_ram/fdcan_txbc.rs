///Register `FDCAN_TXBC` reader
pub type R = crate::R<FDCAN_TXBCrs>;
///Register `FDCAN_TXBC` writer
pub type W = crate::W<FDCAN_TXBCrs>;
///Field `TFQM` reader - Tx FIFO/Queue Mode
pub type TFQM_R = crate::BitReader;
///Field `TFQM` writer - Tx FIFO/Queue Mode
pub type TFQM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 24 - Tx FIFO/Queue Mode
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBC")
            .field("tfqm", &self.tfqm())
            .finish()
    }
}
impl W {
    ///Bit 24 - Tx FIFO/Queue Mode
    #[inline(always)]
    pub fn tfqm(&mut self) -> TFQM_W<FDCAN_TXBCrs> {
        TFQM_W::new(self, 24)
    }
}
/**FDCAN Tx buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#FDCAN1_RAM:FDCAN_TXBC)*/
pub struct FDCAN_TXBCrs;
impl crate::RegisterSpec for FDCAN_TXBCrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbc::R`](R) reader structure
impl crate::Readable for FDCAN_TXBCrs {}
///`write(|w| ..)` method takes [`fdcan_txbc::W`](W) writer structure
impl crate::Writable for FDCAN_TXBCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_TXBC to value 0
impl crate::Resettable for FDCAN_TXBCrs {
    const RESET_VALUE: u32 = 0;
}
