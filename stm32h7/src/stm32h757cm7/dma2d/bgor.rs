///Register `BGOR` reader
pub type R = crate::R<BGORrs>;
///Register `BGOR` writer
pub type W = crate::W<BGORrs>;
///Field `LO` reader - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even.
pub type LO_R = crate::FieldReader<u16>;
///Field `LO` writer - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even.
pub type LO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even.
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BGOR").field("lo", &self.lo()).finish()
    }
}
impl W {
    ///Bits 0:15 - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even.
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W<BGORrs> {
        LO_W::new(self, 0)
    }
}
/**DMA2D background offset register

You can [`read`](crate::Reg::read) this register and get [`bgor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#DMA2D:BGOR)*/
pub struct BGORrs;
impl crate::RegisterSpec for BGORrs {
    type Ux = u32;
}
///`read()` method returns [`bgor::R`](R) reader structure
impl crate::Readable for BGORrs {}
///`write(|w| ..)` method takes [`bgor::W`](W) writer structure
impl crate::Writable for BGORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BGOR to value 0
impl crate::Resettable for BGORrs {}
