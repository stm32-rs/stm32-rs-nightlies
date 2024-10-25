///Register `FDCAN_RXF0C` reader
pub type R = crate::R<FDCAN_RXF0Crs>;
///Register `FDCAN_RXF0C` writer
pub type W = crate::W<FDCAN_RXF0Crs>;
///Field `F0SA` reader - Rx FIFO 0 Start Address
pub type F0SA_R = crate::FieldReader<u16>;
///Field `F0SA` writer - Rx FIFO 0 Start Address
pub type F0SA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `F0S` reader - Rx FIFO 0 Size
pub type F0S_R = crate::FieldReader;
///Field `F0S` writer - Rx FIFO 0 Size
pub type F0S_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `F0WM` reader - FIFO 0 Watermark
pub type F0WM_R = crate::FieldReader;
///Field `F0WM` writer - FIFO 0 Watermark
pub type F0WM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 2:15 - Rx FIFO 0 Start Address
    #[inline(always)]
    pub fn f0sa(&self) -> F0SA_R {
        F0SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:23 - Rx FIFO 0 Size
    #[inline(always)]
    pub fn f0s(&self) -> F0S_R {
        F0S_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - FIFO 0 Watermark
    #[inline(always)]
    pub fn f0wm(&self) -> F0WM_R {
        F0WM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF0C")
            .field("f0sa", &self.f0sa())
            .field("f0s", &self.f0s())
            .field("f0wm", &self.f0wm())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - Rx FIFO 0 Start Address
    #[inline(always)]
    #[must_use]
    pub fn f0sa(&mut self) -> F0SA_W<FDCAN_RXF0Crs> {
        F0SA_W::new(self, 2)
    }
    ///Bits 16:23 - Rx FIFO 0 Size
    #[inline(always)]
    #[must_use]
    pub fn f0s(&mut self) -> F0S_W<FDCAN_RXF0Crs> {
        F0S_W::new(self, 16)
    }
    ///Bits 24:31 - FIFO 0 Watermark
    #[inline(always)]
    #[must_use]
    pub fn f0wm(&mut self) -> F0WM_W<FDCAN_RXF0Crs> {
        F0WM_W::new(self, 24)
    }
}
/**FDCAN Rx FIFO 0 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_RXF0C)*/
pub struct FDCAN_RXF0Crs;
impl crate::RegisterSpec for FDCAN_RXF0Crs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_rxf0c::R`](R) reader structure
impl crate::Readable for FDCAN_RXF0Crs {}
///`write(|w| ..)` method takes [`fdcan_rxf0c::W`](W) writer structure
impl crate::Writable for FDCAN_RXF0Crs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_RXF0C to value 0
impl crate::Resettable for FDCAN_RXF0Crs {
    const RESET_VALUE: u32 = 0;
}
