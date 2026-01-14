///Register `HUFFMIN%s` reader
pub type R = crate::R<HUFFMINrs>;
///Register `HUFFMIN%s` writer
pub type W = crate::W<HUFFMINrs>;
///Field `HuffMin_RAM` reader - HuffMin RAM
pub type HUFF_MIN_RAM_R = crate::FieldReader<u32>;
///Field `HuffMin_RAM` writer - HuffMin RAM
pub type HUFF_MIN_RAM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - HuffMin RAM
    #[inline(always)]
    pub fn huff_min_ram(&self) -> HUFF_MIN_RAM_R {
        HUFF_MIN_RAM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFMIN")
            .field("huff_min_ram", &self.huff_min_ram())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - HuffMin RAM
    #[inline(always)]
    pub fn huff_min_ram(&mut self) -> HUFF_MIN_RAM_W<'_, HUFFMINrs> {
        HUFF_MIN_RAM_W::new(self, 0)
    }
}
/**JPEG HuffMin tables

You can [`read`](crate::Reg::read) this register and get [`huffmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#JPEG:HUFFMIN[0])*/
pub struct HUFFMINrs;
impl crate::RegisterSpec for HUFFMINrs {
    type Ux = u32;
}
///`read()` method returns [`huffmin::R`](R) reader structure
impl crate::Readable for HUFFMINrs {}
///`write(|w| ..)` method takes [`huffmin::W`](W) writer structure
impl crate::Writable for HUFFMINrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFMIN%s to value 0
impl crate::Resettable for HUFFMINrs {}
