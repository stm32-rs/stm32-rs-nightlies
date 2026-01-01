///Register `DHTMEM52` reader
pub type R = crate::R<DHTMEM52rs>;
///Register `DHTMEM52` writer
pub type W = crate::W<DHTMEM52rs>;
///Field `DATA208` reader - Huffman table data 208
pub type DATA208_R = crate::FieldReader;
///Field `DATA208` writer - Huffman table data 208
pub type DATA208_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA209` reader - Huffman table data 209
pub type DATA209_R = crate::FieldReader;
///Field `DATA209` writer - Huffman table data 209
pub type DATA209_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA210` reader - Huffman table data 210
pub type DATA210_R = crate::FieldReader;
///Field `DATA210` writer - Huffman table data 210
pub type DATA210_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA211` reader - Huffman table data 211
pub type DATA211_R = crate::FieldReader;
///Field `DATA211` writer - Huffman table data 211
pub type DATA211_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 208
    #[inline(always)]
    pub fn data208(&self) -> DATA208_R {
        DATA208_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 209
    #[inline(always)]
    pub fn data209(&self) -> DATA209_R {
        DATA209_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 210
    #[inline(always)]
    pub fn data210(&self) -> DATA210_R {
        DATA210_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 211
    #[inline(always)]
    pub fn data211(&self) -> DATA211_R {
        DATA211_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM52")
            .field("data208", &self.data208())
            .field("data209", &self.data209())
            .field("data210", &self.data210())
            .field("data211", &self.data211())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 208
    #[inline(always)]
    pub fn data208(&mut self) -> DATA208_W<'_, DHTMEM52rs> {
        DATA208_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 209
    #[inline(always)]
    pub fn data209(&mut self) -> DATA209_W<'_, DHTMEM52rs> {
        DATA209_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 210
    #[inline(always)]
    pub fn data210(&mut self) -> DATA210_W<'_, DHTMEM52rs> {
        DATA210_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 211
    #[inline(always)]
    pub fn data211(&mut self) -> DATA211_W<'_, DHTMEM52rs> {
        DATA211_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem52::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem52::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM52)*/
pub struct DHTMEM52rs;
impl crate::RegisterSpec for DHTMEM52rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem52::R`](R) reader structure
impl crate::Readable for DHTMEM52rs {}
///`write(|w| ..)` method takes [`dhtmem52::W`](W) writer structure
impl crate::Writable for DHTMEM52rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM52 to value 0
impl crate::Resettable for DHTMEM52rs {}
