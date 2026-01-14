///Register `DHTMEM74` reader
pub type R = crate::R<DHTMEM74rs>;
///Register `DHTMEM74` writer
pub type W = crate::W<DHTMEM74rs>;
///Field `DATA296` reader - Huffman table data 296
pub type DATA296_R = crate::FieldReader;
///Field `DATA296` writer - Huffman table data 296
pub type DATA296_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA297` reader - Huffman table data 297
pub type DATA297_R = crate::FieldReader;
///Field `DATA297` writer - Huffman table data 297
pub type DATA297_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA298` reader - Huffman table data 298
pub type DATA298_R = crate::FieldReader;
///Field `DATA298` writer - Huffman table data 298
pub type DATA298_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA299` reader - Huffman table data 299
pub type DATA299_R = crate::FieldReader;
///Field `DATA299` writer - Huffman table data 299
pub type DATA299_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 296
    #[inline(always)]
    pub fn data296(&self) -> DATA296_R {
        DATA296_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 297
    #[inline(always)]
    pub fn data297(&self) -> DATA297_R {
        DATA297_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 298
    #[inline(always)]
    pub fn data298(&self) -> DATA298_R {
        DATA298_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 299
    #[inline(always)]
    pub fn data299(&self) -> DATA299_R {
        DATA299_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM74")
            .field("data296", &self.data296())
            .field("data297", &self.data297())
            .field("data298", &self.data298())
            .field("data299", &self.data299())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 296
    #[inline(always)]
    pub fn data296(&mut self) -> DATA296_W<'_, DHTMEM74rs> {
        DATA296_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 297
    #[inline(always)]
    pub fn data297(&mut self) -> DATA297_W<'_, DHTMEM74rs> {
        DATA297_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 298
    #[inline(always)]
    pub fn data298(&mut self) -> DATA298_W<'_, DHTMEM74rs> {
        DATA298_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 299
    #[inline(always)]
    pub fn data299(&mut self) -> DATA299_W<'_, DHTMEM74rs> {
        DATA299_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem74::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem74::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM74)*/
pub struct DHTMEM74rs;
impl crate::RegisterSpec for DHTMEM74rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem74::R`](R) reader structure
impl crate::Readable for DHTMEM74rs {}
///`write(|w| ..)` method takes [`dhtmem74::W`](W) writer structure
impl crate::Writable for DHTMEM74rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM74 to value 0
impl crate::Resettable for DHTMEM74rs {}
