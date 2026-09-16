///Register `DHTMEM69` reader
pub type R = crate::R<DHTMEM69rs>;
///Register `DHTMEM69` writer
pub type W = crate::W<DHTMEM69rs>;
///Field `DATA276` reader - Huffman table data 276
pub type DATA276_R = crate::FieldReader;
///Field `DATA276` writer - Huffman table data 276
pub type DATA276_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA277` reader - Huffman table data 277
pub type DATA277_R = crate::FieldReader;
///Field `DATA277` writer - Huffman table data 277
pub type DATA277_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA278` reader - Huffman table data 278
pub type DATA278_R = crate::FieldReader;
///Field `DATA278` writer - Huffman table data 278
pub type DATA278_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA279` reader - Huffman table data 279
pub type DATA279_R = crate::FieldReader;
///Field `DATA279` writer - Huffman table data 279
pub type DATA279_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 276
    #[inline(always)]
    pub fn data276(&self) -> DATA276_R {
        DATA276_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 277
    #[inline(always)]
    pub fn data277(&self) -> DATA277_R {
        DATA277_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 278
    #[inline(always)]
    pub fn data278(&self) -> DATA278_R {
        DATA278_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 279
    #[inline(always)]
    pub fn data279(&self) -> DATA279_R {
        DATA279_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM69")
            .field("data276", &self.data276())
            .field("data277", &self.data277())
            .field("data278", &self.data278())
            .field("data279", &self.data279())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 276
    #[inline(always)]
    pub fn data276(&mut self) -> DATA276_W<'_, DHTMEM69rs> {
        DATA276_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 277
    #[inline(always)]
    pub fn data277(&mut self) -> DATA277_W<'_, DHTMEM69rs> {
        DATA277_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 278
    #[inline(always)]
    pub fn data278(&mut self) -> DATA278_W<'_, DHTMEM69rs> {
        DATA278_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 279
    #[inline(always)]
    pub fn data279(&mut self) -> DATA279_W<'_, DHTMEM69rs> {
        DATA279_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem69::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem69::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM69)*/
pub struct DHTMEM69rs;
impl crate::RegisterSpec for DHTMEM69rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem69::R`](R) reader structure
impl crate::Readable for DHTMEM69rs {}
///`write(|w| ..)` method takes [`dhtmem69::W`](W) writer structure
impl crate::Writable for DHTMEM69rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM69 to value 0
impl crate::Resettable for DHTMEM69rs {}
