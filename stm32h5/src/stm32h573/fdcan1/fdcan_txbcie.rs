///Register `FDCAN_TXBCIE` reader
pub type R = crate::R<FDCAN_TXBCIErs>;
///Register `FDCAN_TXBCIE` writer
pub type W = crate::W<FDCAN_TXBCIErs>;
///Field `CFIE` reader - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit.
pub type CFIE_R = crate::FieldReader;
///Field `CFIE` writer - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit.
pub type CFIE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit.
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBCIE")
            .field("cfie", &self.cfie())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit.
    #[inline(always)]
    pub fn cfie(&mut self) -> CFIE_W<FDCAN_TXBCIErs> {
        CFIE_W::new(self, 0)
    }
}
/**FDCAN Tx buffer cancellation finished interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FDCAN1:FDCAN_TXBCIE)*/
pub struct FDCAN_TXBCIErs;
impl crate::RegisterSpec for FDCAN_TXBCIErs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbcie::R`](R) reader structure
impl crate::Readable for FDCAN_TXBCIErs {}
///`write(|w| ..)` method takes [`fdcan_txbcie::W`](W) writer structure
impl crate::Writable for FDCAN_TXBCIErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_TXBCIE to value 0
impl crate::Resettable for FDCAN_TXBCIErs {
    const RESET_VALUE: u32 = 0;
}
