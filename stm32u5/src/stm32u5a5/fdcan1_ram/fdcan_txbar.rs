///Register `FDCAN_TXBAR` reader
pub type R = crate::R<FDCAN_TXBARrs>;
///Register `FDCAN_TXBAR` writer
pub type W = crate::W<FDCAN_TXBARrs>;
///Field `AR` reader - Add Request
pub type AR_R = crate::FieldReader;
///Field `AR` writer - Add Request
pub type AR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Add Request
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBAR")
            .field("ar", &self.ar())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Add Request
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W<FDCAN_TXBARrs> {
        AR_W::new(self, 0)
    }
}
/**FDCAN Tx Buffer Add Request Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#FDCAN1_RAM:FDCAN_TXBAR)*/
pub struct FDCAN_TXBARrs;
impl crate::RegisterSpec for FDCAN_TXBARrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbar::R`](R) reader structure
impl crate::Readable for FDCAN_TXBARrs {}
///`write(|w| ..)` method takes [`fdcan_txbar::W`](W) writer structure
impl crate::Writable for FDCAN_TXBARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_TXBAR to value 0
impl crate::Resettable for FDCAN_TXBARrs {
    const RESET_VALUE: u32 = 0;
}
