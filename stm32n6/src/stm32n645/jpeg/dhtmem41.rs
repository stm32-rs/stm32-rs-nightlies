///Register `DHTMEM41` reader
pub type R = crate::R<DHTMEM41rs>;
///Register `DHTMEM41` writer
pub type W = crate::W<DHTMEM41rs>;
///Field `DATA164` reader - Huffman table data 164
pub type DATA164_R = crate::FieldReader;
///Field `DATA164` writer - Huffman table data 164
pub type DATA164_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA165` reader - Huffman table data 165
pub type DATA165_R = crate::FieldReader;
///Field `DATA165` writer - Huffman table data 165
pub type DATA165_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA166` reader - Huffman table data 166
pub type DATA166_R = crate::FieldReader;
///Field `DATA166` writer - Huffman table data 166
pub type DATA166_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA167` reader - Huffman table data 167
pub type DATA167_R = crate::FieldReader;
///Field `DATA167` writer - Huffman table data 167
pub type DATA167_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 164
    #[inline(always)]
    pub fn data164(&self) -> DATA164_R {
        DATA164_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 165
    #[inline(always)]
    pub fn data165(&self) -> DATA165_R {
        DATA165_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 166
    #[inline(always)]
    pub fn data166(&self) -> DATA166_R {
        DATA166_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 167
    #[inline(always)]
    pub fn data167(&self) -> DATA167_R {
        DATA167_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM41")
            .field("data164", &self.data164())
            .field("data165", &self.data165())
            .field("data166", &self.data166())
            .field("data167", &self.data167())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 164
    #[inline(always)]
    pub fn data164(&mut self) -> DATA164_W<'_, DHTMEM41rs> {
        DATA164_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 165
    #[inline(always)]
    pub fn data165(&mut self) -> DATA165_W<'_, DHTMEM41rs> {
        DATA165_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 166
    #[inline(always)]
    pub fn data166(&mut self) -> DATA166_W<'_, DHTMEM41rs> {
        DATA166_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 167
    #[inline(always)]
    pub fn data167(&mut self) -> DATA167_W<'_, DHTMEM41rs> {
        DATA167_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem41::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem41::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM41)*/
pub struct DHTMEM41rs;
impl crate::RegisterSpec for DHTMEM41rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem41::R`](R) reader structure
impl crate::Readable for DHTMEM41rs {}
///`write(|w| ..)` method takes [`dhtmem41::W`](W) writer structure
impl crate::Writable for DHTMEM41rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM41 to value 0
impl crate::Resettable for DHTMEM41rs {}
