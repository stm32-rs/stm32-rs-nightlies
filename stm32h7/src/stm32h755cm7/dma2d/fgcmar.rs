///Register `FGCMAR` reader
pub type R = crate::R<FGCMARrs>;
///Register `FGCMAR` writer
pub type W = crate::W<FGCMARrs>;
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
        f.debug_struct("FGCMAR").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned.
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<FGCMARrs> {
        MA_W::new(self, 0)
    }
}
/**DMA2D foreground CLUT memory address register

You can [`read`](crate::Reg::read) this register and get [`fgcmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgcmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#DMA2D:FGCMAR)*/
pub struct FGCMARrs;
impl crate::RegisterSpec for FGCMARrs {
    type Ux = u32;
}
///`read()` method returns [`fgcmar::R`](R) reader structure
impl crate::Readable for FGCMARrs {}
///`write(|w| ..)` method takes [`fgcmar::W`](W) writer structure
impl crate::Writable for FGCMARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FGCMAR to value 0
impl crate::Resettable for FGCMARrs {}
