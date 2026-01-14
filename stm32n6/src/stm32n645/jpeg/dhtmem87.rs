///Register `DHTMEM87` reader
pub type R = crate::R<DHTMEM87rs>;
///Register `DHTMEM87` writer
pub type W = crate::W<DHTMEM87rs>;
///Field `DATA348` reader - Huffman table data 348
pub type DATA348_R = crate::FieldReader;
///Field `DATA348` writer - Huffman table data 348
pub type DATA348_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA349` reader - Huffman table data 349
pub type DATA349_R = crate::FieldReader;
///Field `DATA349` writer - Huffman table data 349
pub type DATA349_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA350` reader - Huffman table data 350
pub type DATA350_R = crate::FieldReader;
///Field `DATA350` writer - Huffman table data 350
pub type DATA350_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA351` reader - Huffman table data 351
pub type DATA351_R = crate::FieldReader;
///Field `DATA351` writer - Huffman table data 351
pub type DATA351_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 348
    #[inline(always)]
    pub fn data348(&self) -> DATA348_R {
        DATA348_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 349
    #[inline(always)]
    pub fn data349(&self) -> DATA349_R {
        DATA349_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 350
    #[inline(always)]
    pub fn data350(&self) -> DATA350_R {
        DATA350_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 351
    #[inline(always)]
    pub fn data351(&self) -> DATA351_R {
        DATA351_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM87")
            .field("data348", &self.data348())
            .field("data349", &self.data349())
            .field("data350", &self.data350())
            .field("data351", &self.data351())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 348
    #[inline(always)]
    pub fn data348(&mut self) -> DATA348_W<'_, DHTMEM87rs> {
        DATA348_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 349
    #[inline(always)]
    pub fn data349(&mut self) -> DATA349_W<'_, DHTMEM87rs> {
        DATA349_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 350
    #[inline(always)]
    pub fn data350(&mut self) -> DATA350_W<'_, DHTMEM87rs> {
        DATA350_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 351
    #[inline(always)]
    pub fn data351(&mut self) -> DATA351_W<'_, DHTMEM87rs> {
        DATA351_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem87::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem87::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM87)*/
pub struct DHTMEM87rs;
impl crate::RegisterSpec for DHTMEM87rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem87::R`](R) reader structure
impl crate::Readable for DHTMEM87rs {}
///`write(|w| ..)` method takes [`dhtmem87::W`](W) writer structure
impl crate::Writable for DHTMEM87rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM87 to value 0
impl crate::Resettable for DHTMEM87rs {}
