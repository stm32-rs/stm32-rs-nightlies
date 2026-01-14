///Register `DHTMEM62` reader
pub type R = crate::R<DHTMEM62rs>;
///Register `DHTMEM62` writer
pub type W = crate::W<DHTMEM62rs>;
///Field `DATA248` reader - Huffman table data 248
pub type DATA248_R = crate::FieldReader;
///Field `DATA248` writer - Huffman table data 248
pub type DATA248_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA249` reader - Huffman table data 249
pub type DATA249_R = crate::FieldReader;
///Field `DATA249` writer - Huffman table data 249
pub type DATA249_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA250` reader - Huffman table data 250
pub type DATA250_R = crate::FieldReader;
///Field `DATA250` writer - Huffman table data 250
pub type DATA250_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA251` reader - Huffman table data 251
pub type DATA251_R = crate::FieldReader;
///Field `DATA251` writer - Huffman table data 251
pub type DATA251_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 248
    #[inline(always)]
    pub fn data248(&self) -> DATA248_R {
        DATA248_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 249
    #[inline(always)]
    pub fn data249(&self) -> DATA249_R {
        DATA249_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 250
    #[inline(always)]
    pub fn data250(&self) -> DATA250_R {
        DATA250_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 251
    #[inline(always)]
    pub fn data251(&self) -> DATA251_R {
        DATA251_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM62")
            .field("data248", &self.data248())
            .field("data249", &self.data249())
            .field("data250", &self.data250())
            .field("data251", &self.data251())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 248
    #[inline(always)]
    pub fn data248(&mut self) -> DATA248_W<'_, DHTMEM62rs> {
        DATA248_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 249
    #[inline(always)]
    pub fn data249(&mut self) -> DATA249_W<'_, DHTMEM62rs> {
        DATA249_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 250
    #[inline(always)]
    pub fn data250(&mut self) -> DATA250_W<'_, DHTMEM62rs> {
        DATA250_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 251
    #[inline(always)]
    pub fn data251(&mut self) -> DATA251_W<'_, DHTMEM62rs> {
        DATA251_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem62::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem62::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM62)*/
pub struct DHTMEM62rs;
impl crate::RegisterSpec for DHTMEM62rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem62::R`](R) reader structure
impl crate::Readable for DHTMEM62rs {}
///`write(|w| ..)` method takes [`dhtmem62::W`](W) writer structure
impl crate::Writable for DHTMEM62rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM62 to value 0
impl crate::Resettable for DHTMEM62rs {}
