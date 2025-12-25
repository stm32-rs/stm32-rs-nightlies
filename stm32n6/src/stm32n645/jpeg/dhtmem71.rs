///Register `DHTMEM71` reader
pub type R = crate::R<DHTMEM71rs>;
///Register `DHTMEM71` writer
pub type W = crate::W<DHTMEM71rs>;
///Field `DATA284` reader - Huffman table data 284
pub type DATA284_R = crate::FieldReader;
///Field `DATA284` writer - Huffman table data 284
pub type DATA284_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA285` reader - Huffman table data 285
pub type DATA285_R = crate::FieldReader;
///Field `DATA285` writer - Huffman table data 285
pub type DATA285_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA286` reader - Huffman table data 286
pub type DATA286_R = crate::FieldReader;
///Field `DATA286` writer - Huffman table data 286
pub type DATA286_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA287` reader - Huffman table data 287
pub type DATA287_R = crate::FieldReader;
///Field `DATA287` writer - Huffman table data 287
pub type DATA287_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 284
    #[inline(always)]
    pub fn data284(&self) -> DATA284_R {
        DATA284_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 285
    #[inline(always)]
    pub fn data285(&self) -> DATA285_R {
        DATA285_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 286
    #[inline(always)]
    pub fn data286(&self) -> DATA286_R {
        DATA286_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 287
    #[inline(always)]
    pub fn data287(&self) -> DATA287_R {
        DATA287_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM71")
            .field("data284", &self.data284())
            .field("data285", &self.data285())
            .field("data286", &self.data286())
            .field("data287", &self.data287())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 284
    #[inline(always)]
    pub fn data284(&mut self) -> DATA284_W<'_, DHTMEM71rs> {
        DATA284_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 285
    #[inline(always)]
    pub fn data285(&mut self) -> DATA285_W<'_, DHTMEM71rs> {
        DATA285_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 286
    #[inline(always)]
    pub fn data286(&mut self) -> DATA286_W<'_, DHTMEM71rs> {
        DATA286_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 287
    #[inline(always)]
    pub fn data287(&mut self) -> DATA287_W<'_, DHTMEM71rs> {
        DATA287_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem71::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem71::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM71)*/
pub struct DHTMEM71rs;
impl crate::RegisterSpec for DHTMEM71rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem71::R`](R) reader structure
impl crate::Readable for DHTMEM71rs {}
///`write(|w| ..)` method takes [`dhtmem71::W`](W) writer structure
impl crate::Writable for DHTMEM71rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM71 to value 0
impl crate::Resettable for DHTMEM71rs {}
