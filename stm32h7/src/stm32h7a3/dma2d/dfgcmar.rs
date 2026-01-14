///Register `DFGCMAR` reader
pub type R = crate::R<DFGCMARrs>;
///Register `DFGCMAR` writer
pub type W = crate::W<DFGCMARrs>;
///Field `MA` reader - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned.
pub type MA_R = crate::FieldReader<u32>;
///Field `MA` writer - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned.
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned.
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFGCMAR").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned.
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<'_, DFGCMARrs> {
        MA_W::new(self, 0)
    }
}
/**DMA2D foreground CLUT memory address register

You can [`read`](crate::Reg::read) this register and get [`dfgcmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfgcmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#DMA2D:DFGCMAR)*/
pub struct DFGCMARrs;
impl crate::RegisterSpec for DFGCMARrs {
    type Ux = u32;
}
///`read()` method returns [`dfgcmar::R`](R) reader structure
impl crate::Readable for DFGCMARrs {}
///`write(|w| ..)` method takes [`dfgcmar::W`](W) writer structure
impl crate::Writable for DFGCMARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFGCMAR to value 0
impl crate::Resettable for DFGCMARrs {}
