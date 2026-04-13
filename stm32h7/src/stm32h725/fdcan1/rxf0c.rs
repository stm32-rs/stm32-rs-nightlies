///Register `RXF0C` reader
pub type R = crate::R<RXF0Crs>;
///Register `RXF0C` writer
pub type W = crate::W<RXF0Crs>;
///Field `F0SA` reader - Rx FIFO 0 Start Address
pub type F0SA_R = crate::FieldReader<u16>;
///Field `F0SA` writer - Rx FIFO 0 Start Address
pub type F0SA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `F0S` reader - Rx FIFO 0 Size
pub type F0S_R = crate::FieldReader;
///Field `F0S` writer - Rx FIFO 0 Size
pub type F0S_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `F0WM` reader - FIFO 0 Watermark
pub type F0WM_R = crate::FieldReader;
///Field `F0WM` writer - FIFO 0 Watermark
pub type F0WM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `F0OM` reader - FIFO 0 operation mode
pub type F0OM_R = crate::BitReader;
///Field `F0OM` writer - FIFO 0 operation mode
pub type F0OM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 2:15 - Rx FIFO 0 Start Address
    #[inline(always)]
    pub fn f0sa(&self) -> F0SA_R {
        F0SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:22 - Rx FIFO 0 Size
    #[inline(always)]
    pub fn f0s(&self) -> F0S_R {
        F0S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - FIFO 0 Watermark
    #[inline(always)]
    pub fn f0wm(&self) -> F0WM_R {
        F0WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    ///Bit 31 - FIFO 0 operation mode
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF0C")
            .field("f0sa", &self.f0sa())
            .field("f0s", &self.f0s())
            .field("f0wm", &self.f0wm())
            .field("f0om", &self.f0om())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - Rx FIFO 0 Start Address
    #[inline(always)]
    pub fn f0sa(&mut self) -> F0SA_W<'_, RXF0Crs> {
        F0SA_W::new(self, 2)
    }
    ///Bits 16:22 - Rx FIFO 0 Size
    #[inline(always)]
    pub fn f0s(&mut self) -> F0S_W<'_, RXF0Crs> {
        F0S_W::new(self, 16)
    }
    ///Bits 24:30 - FIFO 0 Watermark
    #[inline(always)]
    pub fn f0wm(&mut self) -> F0WM_W<'_, RXF0Crs> {
        F0WM_W::new(self, 24)
    }
    ///Bit 31 - FIFO 0 operation mode
    #[inline(always)]
    pub fn f0om(&mut self) -> F0OM_W<'_, RXF0Crs> {
        F0OM_W::new(self, 31)
    }
}
/**FDCAN Rx FIFO 0 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`rxf0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#FDCAN1:RXF0C)*/
pub struct RXF0Crs;
impl crate::RegisterSpec for RXF0Crs {
    type Ux = u32;
}
///`read()` method returns [`rxf0c::R`](R) reader structure
impl crate::Readable for RXF0Crs {}
///`write(|w| ..)` method takes [`rxf0c::W`](W) writer structure
impl crate::Writable for RXF0Crs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RXF0C to value 0
impl crate::Resettable for RXF0Crs {}
