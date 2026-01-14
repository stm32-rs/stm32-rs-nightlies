///Register `HUFFENC_AC0%s` reader
pub type R = crate::R<HUFFENC_AC0rs>;
///Register `HUFFENC_AC0%s` writer
pub type W = crate::W<HUFFENC_AC0rs>;
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
        f.debug_struct("HUFFENC_AC0")
            .field("dhtmem_ram", &self.dhtmem_ram())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DHTMem RAM
    #[inline(always)]
    pub fn dhtmem_ram(&mut self) -> DHTMEM_RAM_W<'_, HUFFENC_AC0rs> {
        DHTMEM_RAM_W::new(self, 0)
    }
}
/**JPEG encoder, AC Huffman table 0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#JPEG:HUFFENC_AC0[0])*/
pub struct HUFFENC_AC0rs;
impl crate::RegisterSpec for HUFFENC_AC0rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0rs {}
///`write(|w| ..)` method takes [`huffenc_ac0::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0%s to value 0
impl crate::Resettable for HUFFENC_AC0rs {}
