///Register `DHTMEM64` reader
pub type R = crate::R<DHTMEM64rs>;
///Register `DHTMEM64` writer
pub type W = crate::W<DHTMEM64rs>;
///Field `DATA256` reader - Huffman table data 256
pub type DATA256_R = crate::FieldReader;
///Field `DATA256` writer - Huffman table data 256
pub type DATA256_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA257` reader - Huffman table data 257
pub type DATA257_R = crate::FieldReader;
///Field `DATA257` writer - Huffman table data 257
pub type DATA257_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA258` reader - Huffman table data 258
pub type DATA258_R = crate::FieldReader;
///Field `DATA258` writer - Huffman table data 258
pub type DATA258_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA259` reader - Huffman table data 259
pub type DATA259_R = crate::FieldReader;
///Field `DATA259` writer - Huffman table data 259
pub type DATA259_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 256
    #[inline(always)]
    pub fn data256(&self) -> DATA256_R {
        DATA256_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 257
    #[inline(always)]
    pub fn data257(&self) -> DATA257_R {
        DATA257_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 258
    #[inline(always)]
    pub fn data258(&self) -> DATA258_R {
        DATA258_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 259
    #[inline(always)]
    pub fn data259(&self) -> DATA259_R {
        DATA259_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM64")
            .field("data256", &self.data256())
            .field("data257", &self.data257())
            .field("data258", &self.data258())
            .field("data259", &self.data259())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 256
    #[inline(always)]
    pub fn data256(&mut self) -> DATA256_W<'_, DHTMEM64rs> {
        DATA256_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 257
    #[inline(always)]
    pub fn data257(&mut self) -> DATA257_W<'_, DHTMEM64rs> {
        DATA257_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 258
    #[inline(always)]
    pub fn data258(&mut self) -> DATA258_W<'_, DHTMEM64rs> {
        DATA258_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 259
    #[inline(always)]
    pub fn data259(&mut self) -> DATA259_W<'_, DHTMEM64rs> {
        DATA259_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM64)*/
pub struct DHTMEM64rs;
impl crate::RegisterSpec for DHTMEM64rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem64::R`](R) reader structure
impl crate::Readable for DHTMEM64rs {}
///`write(|w| ..)` method takes [`dhtmem64::W`](W) writer structure
impl crate::Writable for DHTMEM64rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM64 to value 0
impl crate::Resettable for DHTMEM64rs {}
