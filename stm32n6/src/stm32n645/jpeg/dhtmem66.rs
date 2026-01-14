///Register `DHTMEM66` reader
pub type R = crate::R<DHTMEM66rs>;
///Register `DHTMEM66` writer
pub type W = crate::W<DHTMEM66rs>;
///Field `DATA264` reader - Huffman table data 264
pub type DATA264_R = crate::FieldReader;
///Field `DATA264` writer - Huffman table data 264
pub type DATA264_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA265` reader - Huffman table data 265
pub type DATA265_R = crate::FieldReader;
///Field `DATA265` writer - Huffman table data 265
pub type DATA265_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA266` reader - Huffman table data 266
pub type DATA266_R = crate::FieldReader;
///Field `DATA266` writer - Huffman table data 266
pub type DATA266_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA267` reader - Huffman table data 267
pub type DATA267_R = crate::FieldReader;
///Field `DATA267` writer - Huffman table data 267
pub type DATA267_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 264
    #[inline(always)]
    pub fn data264(&self) -> DATA264_R {
        DATA264_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 265
    #[inline(always)]
    pub fn data265(&self) -> DATA265_R {
        DATA265_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 266
    #[inline(always)]
    pub fn data266(&self) -> DATA266_R {
        DATA266_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 267
    #[inline(always)]
    pub fn data267(&self) -> DATA267_R {
        DATA267_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM66")
            .field("data264", &self.data264())
            .field("data265", &self.data265())
            .field("data266", &self.data266())
            .field("data267", &self.data267())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 264
    #[inline(always)]
    pub fn data264(&mut self) -> DATA264_W<'_, DHTMEM66rs> {
        DATA264_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 265
    #[inline(always)]
    pub fn data265(&mut self) -> DATA265_W<'_, DHTMEM66rs> {
        DATA265_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 266
    #[inline(always)]
    pub fn data266(&mut self) -> DATA266_W<'_, DHTMEM66rs> {
        DATA266_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 267
    #[inline(always)]
    pub fn data267(&mut self) -> DATA267_W<'_, DHTMEM66rs> {
        DATA267_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem66::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem66::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM66)*/
pub struct DHTMEM66rs;
impl crate::RegisterSpec for DHTMEM66rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem66::R`](R) reader structure
impl crate::Readable for DHTMEM66rs {}
///`write(|w| ..)` method takes [`dhtmem66::W`](W) writer structure
impl crate::Writable for DHTMEM66rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM66 to value 0
impl crate::Resettable for DHTMEM66rs {}
