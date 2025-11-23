///Register `DHTMEM72` reader
pub type R = crate::R<DHTMEM72rs>;
///Register `DHTMEM72` writer
pub type W = crate::W<DHTMEM72rs>;
///Field `DATA288` reader - Huffman table data 288
pub type DATA288_R = crate::FieldReader;
///Field `DATA288` writer - Huffman table data 288
pub type DATA288_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA289` reader - Huffman table data 289
pub type DATA289_R = crate::FieldReader;
///Field `DATA289` writer - Huffman table data 289
pub type DATA289_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA290` reader - Huffman table data 290
pub type DATA290_R = crate::FieldReader;
///Field `DATA290` writer - Huffman table data 290
pub type DATA290_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA291` reader - Huffman table data 291
pub type DATA291_R = crate::FieldReader;
///Field `DATA291` writer - Huffman table data 291
pub type DATA291_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 288
    #[inline(always)]
    pub fn data288(&self) -> DATA288_R {
        DATA288_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 289
    #[inline(always)]
    pub fn data289(&self) -> DATA289_R {
        DATA289_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 290
    #[inline(always)]
    pub fn data290(&self) -> DATA290_R {
        DATA290_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 291
    #[inline(always)]
    pub fn data291(&self) -> DATA291_R {
        DATA291_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM72")
            .field("data288", &self.data288())
            .field("data289", &self.data289())
            .field("data290", &self.data290())
            .field("data291", &self.data291())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 288
    #[inline(always)]
    pub fn data288(&mut self) -> DATA288_W<'_, DHTMEM72rs> {
        DATA288_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 289
    #[inline(always)]
    pub fn data289(&mut self) -> DATA289_W<'_, DHTMEM72rs> {
        DATA289_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 290
    #[inline(always)]
    pub fn data290(&mut self) -> DATA290_W<'_, DHTMEM72rs> {
        DATA290_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 291
    #[inline(always)]
    pub fn data291(&mut self) -> DATA291_W<'_, DHTMEM72rs> {
        DATA291_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem72::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem72::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM72)*/
pub struct DHTMEM72rs;
impl crate::RegisterSpec for DHTMEM72rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem72::R`](R) reader structure
impl crate::Readable for DHTMEM72rs {}
///`write(|w| ..)` method takes [`dhtmem72::W`](W) writer structure
impl crate::Writable for DHTMEM72rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM72 to value 0
impl crate::Resettable for DHTMEM72rs {}
