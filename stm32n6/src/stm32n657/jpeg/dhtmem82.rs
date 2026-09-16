///Register `DHTMEM82` reader
pub type R = crate::R<DHTMEM82rs>;
///Register `DHTMEM82` writer
pub type W = crate::W<DHTMEM82rs>;
///Field `DATA328` reader - Huffman table data 328
pub type DATA328_R = crate::FieldReader;
///Field `DATA328` writer - Huffman table data 328
pub type DATA328_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA329` reader - Huffman table data 329
pub type DATA329_R = crate::FieldReader;
///Field `DATA329` writer - Huffman table data 329
pub type DATA329_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA330` reader - Huffman table data 330
pub type DATA330_R = crate::FieldReader;
///Field `DATA330` writer - Huffman table data 330
pub type DATA330_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA331` reader - Huffman table data 331
pub type DATA331_R = crate::FieldReader;
///Field `DATA331` writer - Huffman table data 331
pub type DATA331_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 328
    #[inline(always)]
    pub fn data328(&self) -> DATA328_R {
        DATA328_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 329
    #[inline(always)]
    pub fn data329(&self) -> DATA329_R {
        DATA329_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 330
    #[inline(always)]
    pub fn data330(&self) -> DATA330_R {
        DATA330_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 331
    #[inline(always)]
    pub fn data331(&self) -> DATA331_R {
        DATA331_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM82")
            .field("data328", &self.data328())
            .field("data329", &self.data329())
            .field("data330", &self.data330())
            .field("data331", &self.data331())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 328
    #[inline(always)]
    pub fn data328(&mut self) -> DATA328_W<'_, DHTMEM82rs> {
        DATA328_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 329
    #[inline(always)]
    pub fn data329(&mut self) -> DATA329_W<'_, DHTMEM82rs> {
        DATA329_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 330
    #[inline(always)]
    pub fn data330(&mut self) -> DATA330_W<'_, DHTMEM82rs> {
        DATA330_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 331
    #[inline(always)]
    pub fn data331(&mut self) -> DATA331_W<'_, DHTMEM82rs> {
        DATA331_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem82::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem82::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM82)*/
pub struct DHTMEM82rs;
impl crate::RegisterSpec for DHTMEM82rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem82::R`](R) reader structure
impl crate::Readable for DHTMEM82rs {}
///`write(|w| ..)` method takes [`dhtmem82::W`](W) writer structure
impl crate::Writable for DHTMEM82rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM82 to value 0
impl crate::Resettable for DHTMEM82rs {}
