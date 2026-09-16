///Register `RXF1C` reader
pub type R = crate::R<RXF1Crs>;
///Register `RXF1C` writer
pub type W = crate::W<RXF1Crs>;
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
///Field `F1OM` reader - FIFO 1 operation mode
pub type F1OM_R = crate::BitReader;
///Field `F1OM` writer - FIFO 1 operation mode
pub type F1OM_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 31 - FIFO 1 operation mode
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF1C")
            .field("f1sa", &self.f1sa())
            .field("f1s", &self.f1s())
            .field("f1wm", &self.f1wm())
            .field("f1om", &self.f1om())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - Rx FIFO 1 Start Address
    #[inline(always)]
    pub fn f1sa(&mut self) -> F1SA_W<'_, RXF1Crs> {
        F1SA_W::new(self, 2)
    }
    ///Bits 16:22 - Rx FIFO 1 Size
    #[inline(always)]
    pub fn f1s(&mut self) -> F1S_W<'_, RXF1Crs> {
        F1S_W::new(self, 16)
    }
    ///Bits 24:30 - Rx FIFO 1 Watermark
    #[inline(always)]
    pub fn f1wm(&mut self) -> F1WM_W<'_, RXF1Crs> {
        F1WM_W::new(self, 24)
    }
    ///Bit 31 - FIFO 1 operation mode
    #[inline(always)]
    pub fn f1om(&mut self) -> F1OM_W<'_, RXF1Crs> {
        F1OM_W::new(self, 31)
    }
}
/**FDCAN Rx FIFO 1 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`rxf1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#FDCAN1:RXF1C)*/
pub struct RXF1Crs;
impl crate::RegisterSpec for RXF1Crs {
    type Ux = u32;
}
///`read()` method returns [`rxf1c::R`](R) reader structure
impl crate::Readable for RXF1Crs {}
///`write(|w| ..)` method takes [`rxf1c::W`](W) writer structure
impl crate::Writable for RXF1Crs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RXF1C to value 0
impl crate::Resettable for RXF1Crs {}
