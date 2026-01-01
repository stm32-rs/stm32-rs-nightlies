///Register `DHTMEM51` reader
pub type R = crate::R<DHTMEM51rs>;
///Register `DHTMEM51` writer
pub type W = crate::W<DHTMEM51rs>;
///Field `DATA204` reader - Huffman table data 204
pub type DATA204_R = crate::FieldReader;
///Field `DATA204` writer - Huffman table data 204
pub type DATA204_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA205` reader - Huffman table data 205
pub type DATA205_R = crate::FieldReader;
///Field `DATA205` writer - Huffman table data 205
pub type DATA205_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA206` reader - Huffman table data 206
pub type DATA206_R = crate::FieldReader;
///Field `DATA206` writer - Huffman table data 206
pub type DATA206_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA207` reader - Huffman table data 207
pub type DATA207_R = crate::FieldReader;
///Field `DATA207` writer - Huffman table data 207
pub type DATA207_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 204
    #[inline(always)]
    pub fn data204(&self) -> DATA204_R {
        DATA204_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 205
    #[inline(always)]
    pub fn data205(&self) -> DATA205_R {
        DATA205_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 206
    #[inline(always)]
    pub fn data206(&self) -> DATA206_R {
        DATA206_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 207
    #[inline(always)]
    pub fn data207(&self) -> DATA207_R {
        DATA207_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM51")
            .field("data204", &self.data204())
            .field("data205", &self.data205())
            .field("data206", &self.data206())
            .field("data207", &self.data207())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 204
    #[inline(always)]
    pub fn data204(&mut self) -> DATA204_W<'_, DHTMEM51rs> {
        DATA204_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 205
    #[inline(always)]
    pub fn data205(&mut self) -> DATA205_W<'_, DHTMEM51rs> {
        DATA205_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 206
    #[inline(always)]
    pub fn data206(&mut self) -> DATA206_W<'_, DHTMEM51rs> {
        DATA206_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 207
    #[inline(always)]
    pub fn data207(&mut self) -> DATA207_W<'_, DHTMEM51rs> {
        DATA207_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem51::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem51::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM51)*/
pub struct DHTMEM51rs;
impl crate::RegisterSpec for DHTMEM51rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem51::R`](R) reader structure
impl crate::Readable for DHTMEM51rs {}
///`write(|w| ..)` method takes [`dhtmem51::W`](W) writer structure
impl crate::Writable for DHTMEM51rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM51 to value 0
impl crate::Resettable for DHTMEM51rs {}
