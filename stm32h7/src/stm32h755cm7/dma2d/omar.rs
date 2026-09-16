///Register `OMAR` reader
pub type R = crate::R<OMARrs>;
///Register `OMAR` writer
pub type W = crate::W<OMARrs>;
///Field `MA` reader - Memory Address Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned.
pub type MA_R = crate::FieldReader<u32>;
///Field `MA` writer - Memory Address Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned.
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Memory Address Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned.
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OMAR").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - Memory Address Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned.
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<'_, OMARrs> {
        MA_W::new(self, 0)
    }
}
/**DMA2D output memory address register

You can [`read`](crate::Reg::read) this register and get [`omar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`omar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#DMA2D:OMAR)*/
pub struct OMARrs;
impl crate::RegisterSpec for OMARrs {
    type Ux = u32;
}
///`read()` method returns [`omar::R`](R) reader structure
impl crate::Readable for OMARrs {}
///`write(|w| ..)` method takes [`omar::W`](W) writer structure
impl crate::Writable for OMARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OMAR to value 0
impl crate::Resettable for OMARrs {}
