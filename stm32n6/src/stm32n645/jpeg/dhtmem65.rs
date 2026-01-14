///Register `DHTMEM65` reader
pub type R = crate::R<DHTMEM65rs>;
///Register `DHTMEM65` writer
pub type W = crate::W<DHTMEM65rs>;
///Field `DATA260` reader - Huffman table data 260
pub type DATA260_R = crate::FieldReader;
///Field `DATA260` writer - Huffman table data 260
pub type DATA260_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA261` reader - Huffman table data 261
pub type DATA261_R = crate::FieldReader;
///Field `DATA261` writer - Huffman table data 261
pub type DATA261_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA262` reader - Huffman table data 262
pub type DATA262_R = crate::FieldReader;
///Field `DATA262` writer - Huffman table data 262
pub type DATA262_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA263` reader - Huffman table data 263
pub type DATA263_R = crate::FieldReader;
///Field `DATA263` writer - Huffman table data 263
pub type DATA263_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 260
    #[inline(always)]
    pub fn data260(&self) -> DATA260_R {
        DATA260_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 261
    #[inline(always)]
    pub fn data261(&self) -> DATA261_R {
        DATA261_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 262
    #[inline(always)]
    pub fn data262(&self) -> DATA262_R {
        DATA262_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 263
    #[inline(always)]
    pub fn data263(&self) -> DATA263_R {
        DATA263_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM65")
            .field("data260", &self.data260())
            .field("data261", &self.data261())
            .field("data262", &self.data262())
            .field("data263", &self.data263())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 260
    #[inline(always)]
    pub fn data260(&mut self) -> DATA260_W<'_, DHTMEM65rs> {
        DATA260_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 261
    #[inline(always)]
    pub fn data261(&mut self) -> DATA261_W<'_, DHTMEM65rs> {
        DATA261_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 262
    #[inline(always)]
    pub fn data262(&mut self) -> DATA262_W<'_, DHTMEM65rs> {
        DATA262_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 263
    #[inline(always)]
    pub fn data263(&mut self) -> DATA263_W<'_, DHTMEM65rs> {
        DATA263_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem65::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem65::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM65)*/
pub struct DHTMEM65rs;
impl crate::RegisterSpec for DHTMEM65rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem65::R`](R) reader structure
impl crate::Readable for DHTMEM65rs {}
///`write(|w| ..)` method takes [`dhtmem65::W`](W) writer structure
impl crate::Writable for DHTMEM65rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM65 to value 0
impl crate::Resettable for DHTMEM65rs {}
