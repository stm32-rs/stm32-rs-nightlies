///Register `FDCAN_TXBTIE` reader
pub type R = crate::R<FDCAN_TXBTIErs>;
///Register `FDCAN_TXBTIE` writer
pub type W = crate::W<FDCAN_TXBTIErs>;
///Field `TIE` reader - Transmission Interrupt Enable
pub type TIE_R = crate::FieldReader<u32>;
///Field `TIE` writer - Transmission Interrupt Enable
pub type TIE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Transmission Interrupt Enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBTIE")
            .field("tie", &self.tie())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Transmission Interrupt Enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<FDCAN_TXBTIErs> {
        TIE_W::new(self, 0)
    }
}
/**FDCAN Tx Buffer Transmission Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbtie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbtie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXBTIE)*/
pub struct FDCAN_TXBTIErs;
impl crate::RegisterSpec for FDCAN_TXBTIErs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbtie::R`](R) reader structure
impl crate::Readable for FDCAN_TXBTIErs {}
///`write(|w| ..)` method takes [`fdcan_txbtie::W`](W) writer structure
impl crate::Writable for FDCAN_TXBTIErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_TXBTIE to value 0
impl crate::Resettable for FDCAN_TXBTIErs {
    const RESET_VALUE: u32 = 0;
}
