///Register `HUFFENC_DC1%s` reader
pub type R = crate::R<HUFFENC_DC1rs>;
///Register `HUFFENC_DC1%s` writer
pub type W = crate::W<HUFFENC_DC1rs>;
///Field `DHTMem_RAM` reader - DHTMem RAM
pub type DHTMEM_RAM_R = crate::FieldReader<u32>;
///Field `DHTMem_RAM` writer - DHTMem RAM
pub type DHTMEM_RAM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DHTMem RAM
    #[inline(always)]
    pub fn dhtmem_ram(&self) -> DHTMEM_RAM_R {
        DHTMEM_RAM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_DC1")
            .field("dhtmem_ram", &self.dhtmem_ram())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DHTMem RAM
    #[inline(always)]
    pub fn dhtmem_ram(&mut self) -> DHTMEM_RAM_W<'_, HUFFENC_DC1rs> {
        DHTMEM_RAM_W::new(self, 0)
    }
}
/**JPEG encoder, DC Huffman table 1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#JPEG:HUFFENC_DC1[0])*/
pub struct HUFFENC_DC1rs;
impl crate::RegisterSpec for HUFFENC_DC1rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_dc1::R`](R) reader structure
impl crate::Readable for HUFFENC_DC1rs {}
///`write(|w| ..)` method takes [`huffenc_dc1::W`](W) writer structure
impl crate::Writable for HUFFENC_DC1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_DC1%s to value 0
impl crate::Resettable for HUFFENC_DC1rs {}
