///Register `DHTMEM10` reader
pub type R = crate::R<DHTMEM10rs>;
///Register `DHTMEM10` writer
pub type W = crate::W<DHTMEM10rs>;
///Field `DATA40` reader - Huffman table data 40
pub type DATA40_R = crate::FieldReader;
///Field `DATA40` writer - Huffman table data 40
pub type DATA40_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA41` reader - Huffman table data 41
pub type DATA41_R = crate::FieldReader;
///Field `DATA41` writer - Huffman table data 41
pub type DATA41_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA42` reader - Huffman table data 42
pub type DATA42_R = crate::FieldReader;
///Field `DATA42` writer - Huffman table data 42
pub type DATA42_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA43` reader - Huffman table data 43
pub type DATA43_R = crate::FieldReader;
///Field `DATA43` writer - Huffman table data 43
pub type DATA43_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 40
    #[inline(always)]
    pub fn data40(&self) -> DATA40_R {
        DATA40_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 41
    #[inline(always)]
    pub fn data41(&self) -> DATA41_R {
        DATA41_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 42
    #[inline(always)]
    pub fn data42(&self) -> DATA42_R {
        DATA42_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 43
    #[inline(always)]
    pub fn data43(&self) -> DATA43_R {
        DATA43_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM10")
            .field("data40", &self.data40())
            .field("data41", &self.data41())
            .field("data42", &self.data42())
            .field("data43", &self.data43())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 40
    #[inline(always)]
    pub fn data40(&mut self) -> DATA40_W<'_, DHTMEM10rs> {
        DATA40_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 41
    #[inline(always)]
    pub fn data41(&mut self) -> DATA41_W<'_, DHTMEM10rs> {
        DATA41_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 42
    #[inline(always)]
    pub fn data42(&mut self) -> DATA42_W<'_, DHTMEM10rs> {
        DATA42_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 43
    #[inline(always)]
    pub fn data43(&mut self) -> DATA43_W<'_, DHTMEM10rs> {
        DATA43_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM10)*/
pub struct DHTMEM10rs;
impl crate::RegisterSpec for DHTMEM10rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem10::R`](R) reader structure
impl crate::Readable for DHTMEM10rs {}
///`write(|w| ..)` method takes [`dhtmem10::W`](W) writer structure
impl crate::Writable for DHTMEM10rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM10 to value 0
impl crate::Resettable for DHTMEM10rs {}
