///Register `DHTMEM91` reader
pub type R = crate::R<DHTMEM91rs>;
///Register `DHTMEM91` writer
pub type W = crate::W<DHTMEM91rs>;
///Field `DATA364` reader - Huffman table data 364
pub type DATA364_R = crate::FieldReader;
///Field `DATA364` writer - Huffman table data 364
pub type DATA364_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA365` reader - Huffman table data 365
pub type DATA365_R = crate::FieldReader;
///Field `DATA365` writer - Huffman table data 365
pub type DATA365_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA366` reader - Huffman table data 366
pub type DATA366_R = crate::FieldReader;
///Field `DATA366` writer - Huffman table data 366
pub type DATA366_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA367` reader - Huffman table data 367
pub type DATA367_R = crate::FieldReader;
///Field `DATA367` writer - Huffman table data 367
pub type DATA367_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 364
    #[inline(always)]
    pub fn data364(&self) -> DATA364_R {
        DATA364_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 365
    #[inline(always)]
    pub fn data365(&self) -> DATA365_R {
        DATA365_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 366
    #[inline(always)]
    pub fn data366(&self) -> DATA366_R {
        DATA366_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 367
    #[inline(always)]
    pub fn data367(&self) -> DATA367_R {
        DATA367_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM91")
            .field("data364", &self.data364())
            .field("data365", &self.data365())
            .field("data366", &self.data366())
            .field("data367", &self.data367())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 364
    #[inline(always)]
    pub fn data364(&mut self) -> DATA364_W<'_, DHTMEM91rs> {
        DATA364_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 365
    #[inline(always)]
    pub fn data365(&mut self) -> DATA365_W<'_, DHTMEM91rs> {
        DATA365_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 366
    #[inline(always)]
    pub fn data366(&mut self) -> DATA366_W<'_, DHTMEM91rs> {
        DATA366_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 367
    #[inline(always)]
    pub fn data367(&mut self) -> DATA367_W<'_, DHTMEM91rs> {
        DATA367_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem91::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem91::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM91)*/
pub struct DHTMEM91rs;
impl crate::RegisterSpec for DHTMEM91rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem91::R`](R) reader structure
impl crate::Readable for DHTMEM91rs {}
///`write(|w| ..)` method takes [`dhtmem91::W`](W) writer structure
impl crate::Writable for DHTMEM91rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM91 to value 0
impl crate::Resettable for DHTMEM91rs {}
