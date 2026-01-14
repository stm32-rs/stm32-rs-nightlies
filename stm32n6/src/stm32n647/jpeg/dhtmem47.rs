///Register `DHTMEM47` reader
pub type R = crate::R<DHTMEM47rs>;
///Register `DHTMEM47` writer
pub type W = crate::W<DHTMEM47rs>;
///Field `DATA188` reader - Huffman table data 188
pub type DATA188_R = crate::FieldReader;
///Field `DATA188` writer - Huffman table data 188
pub type DATA188_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA189` reader - Huffman table data 189
pub type DATA189_R = crate::FieldReader;
///Field `DATA189` writer - Huffman table data 189
pub type DATA189_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA190` reader - Huffman table data 190
pub type DATA190_R = crate::FieldReader;
///Field `DATA190` writer - Huffman table data 190
pub type DATA190_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA191` reader - Huffman table data 191
pub type DATA191_R = crate::FieldReader;
///Field `DATA191` writer - Huffman table data 191
pub type DATA191_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 188
    #[inline(always)]
    pub fn data188(&self) -> DATA188_R {
        DATA188_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 189
    #[inline(always)]
    pub fn data189(&self) -> DATA189_R {
        DATA189_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 190
    #[inline(always)]
    pub fn data190(&self) -> DATA190_R {
        DATA190_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 191
    #[inline(always)]
    pub fn data191(&self) -> DATA191_R {
        DATA191_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM47")
            .field("data188", &self.data188())
            .field("data189", &self.data189())
            .field("data190", &self.data190())
            .field("data191", &self.data191())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 188
    #[inline(always)]
    pub fn data188(&mut self) -> DATA188_W<'_, DHTMEM47rs> {
        DATA188_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 189
    #[inline(always)]
    pub fn data189(&mut self) -> DATA189_W<'_, DHTMEM47rs> {
        DATA189_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 190
    #[inline(always)]
    pub fn data190(&mut self) -> DATA190_W<'_, DHTMEM47rs> {
        DATA190_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 191
    #[inline(always)]
    pub fn data191(&mut self) -> DATA191_W<'_, DHTMEM47rs> {
        DATA191_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem47::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem47::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM47)*/
pub struct DHTMEM47rs;
impl crate::RegisterSpec for DHTMEM47rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem47::R`](R) reader structure
impl crate::Readable for DHTMEM47rs {}
///`write(|w| ..)` method takes [`dhtmem47::W`](W) writer structure
impl crate::Writable for DHTMEM47rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM47 to value 0
impl crate::Resettable for DHTMEM47rs {}
