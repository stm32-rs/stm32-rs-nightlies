///Register `FDCAN_RXF1C` reader
pub type R = crate::R<FDCAN_RXF1Crs>;
///Register `FDCAN_RXF1C` writer
pub type W = crate::W<FDCAN_RXF1Crs>;
///Field `F1SA` reader - Rx FIFO 1 Start Address
pub type F1SA_R = crate::FieldReader<u16>;
///Field `F1SA` writer - Rx FIFO 1 Start Address
pub type F1SA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `F1S` reader - Rx FIFO 1 Size
pub type F1S_R = crate::FieldReader;
///Field `F1S` writer - Rx FIFO 1 Size
pub type F1S_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `F1WM` reader - Rx FIFO 1 Watermark
pub type F1WM_R = crate::FieldReader;
///Field `F1WM` writer - Rx FIFO 1 Watermark
pub type F1WM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 2:15 - Rx FIFO 1 Start Address
    #[inline(always)]
    pub fn f1sa(&self) -> F1SA_R {
        F1SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:22 - Rx FIFO 1 Size
    #[inline(always)]
    pub fn f1s(&self) -> F1S_R {
        F1S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - Rx FIFO 1 Watermark
    #[inline(always)]
    pub fn f1wm(&self) -> F1WM_R {
        F1WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF1C")
            .field("f1sa", &self.f1sa())
            .field("f1s", &self.f1s())
            .field("f1wm", &self.f1wm())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - Rx FIFO 1 Start Address
    #[inline(always)]
    pub fn f1sa(&mut self) -> F1SA_W<FDCAN_RXF1Crs> {
        F1SA_W::new(self, 2)
    }
    ///Bits 16:22 - Rx FIFO 1 Size
    #[inline(always)]
    pub fn f1s(&mut self) -> F1S_W<FDCAN_RXF1Crs> {
        F1S_W::new(self, 16)
    }
    ///Bits 24:30 - Rx FIFO 1 Watermark
    #[inline(always)]
    pub fn f1wm(&mut self) -> F1WM_W<FDCAN_RXF1Crs> {
        F1WM_W::new(self, 24)
    }
}
/**FDCAN Rx FIFO 1 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#CAN_CCU:FDCAN_RXF1C)*/
pub struct FDCAN_RXF1Crs;
impl crate::RegisterSpec for FDCAN_RXF1Crs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_rxf1c::R`](R) reader structure
impl crate::Readable for FDCAN_RXF1Crs {}
///`write(|w| ..)` method takes [`fdcan_rxf1c::W`](W) writer structure
impl crate::Writable for FDCAN_RXF1Crs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_RXF1C to value 0
impl crate::Resettable for FDCAN_RXF1Crs {
    const RESET_VALUE: u32 = 0;
}
