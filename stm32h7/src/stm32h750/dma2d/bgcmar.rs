///Register `BGCMAR` reader
pub type R = crate::R<BGCMARrs>;
///Register `BGCMAR` writer
pub type W = crate::W<BGCMARrs>;
///Field `MA` reader - Memory address Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned.
pub type MA_R = crate::FieldReader<u32>;
///Field `MA` writer - Memory address Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned.
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Memory address Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned.
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BGCMAR").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - Memory address Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned.
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<BGCMARrs> {
        MA_W::new(self, 0)
    }
}
/**DMA2D background CLUT memory address register

You can [`read`](crate::Reg::read) this register and get [`bgcmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#DMA2D:BGCMAR)*/
pub struct BGCMARrs;
impl crate::RegisterSpec for BGCMARrs {
    type Ux = u32;
}
///`read()` method returns [`bgcmar::R`](R) reader structure
impl crate::Readable for BGCMARrs {}
///`write(|w| ..)` method takes [`bgcmar::W`](W) writer structure
impl crate::Writable for BGCMARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BGCMAR to value 0
impl crate::Resettable for BGCMARrs {}
