///Register `DHTMEM58` reader
pub type R = crate::R<DHTMEM58rs>;
///Register `DHTMEM58` writer
pub type W = crate::W<DHTMEM58rs>;
///Field `DATA232` reader - Huffman table data 232
pub type DATA232_R = crate::FieldReader;
///Field `DATA232` writer - Huffman table data 232
pub type DATA232_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA233` reader - Huffman table data 233
pub type DATA233_R = crate::FieldReader;
///Field `DATA233` writer - Huffman table data 233
pub type DATA233_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA234` reader - Huffman table data 234
pub type DATA234_R = crate::FieldReader;
///Field `DATA234` writer - Huffman table data 234
pub type DATA234_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA235` reader - Huffman table data 235
pub type DATA235_R = crate::FieldReader;
///Field `DATA235` writer - Huffman table data 235
pub type DATA235_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 232
    #[inline(always)]
    pub fn data232(&self) -> DATA232_R {
        DATA232_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 233
    #[inline(always)]
    pub fn data233(&self) -> DATA233_R {
        DATA233_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 234
    #[inline(always)]
    pub fn data234(&self) -> DATA234_R {
        DATA234_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 235
    #[inline(always)]
    pub fn data235(&self) -> DATA235_R {
        DATA235_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM58")
            .field("data232", &self.data232())
            .field("data233", &self.data233())
            .field("data234", &self.data234())
            .field("data235", &self.data235())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 232
    #[inline(always)]
    pub fn data232(&mut self) -> DATA232_W<'_, DHTMEM58rs> {
        DATA232_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 233
    #[inline(always)]
    pub fn data233(&mut self) -> DATA233_W<'_, DHTMEM58rs> {
        DATA233_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 234
    #[inline(always)]
    pub fn data234(&mut self) -> DATA234_W<'_, DHTMEM58rs> {
        DATA234_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 235
    #[inline(always)]
    pub fn data235(&mut self) -> DATA235_W<'_, DHTMEM58rs> {
        DATA235_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM58)*/
pub struct DHTMEM58rs;
impl crate::RegisterSpec for DHTMEM58rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem58::R`](R) reader structure
impl crate::Readable for DHTMEM58rs {}
///`write(|w| ..)` method takes [`dhtmem58::W`](W) writer structure
impl crate::Writable for DHTMEM58rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM58 to value 0
impl crate::Resettable for DHTMEM58rs {}
