///Register `DHTMEM25` reader
pub type R = crate::R<DHTMEM25rs>;
///Register `DHTMEM25` writer
pub type W = crate::W<DHTMEM25rs>;
///Field `DATA100` reader - Huffman table data 100
pub type DATA100_R = crate::FieldReader;
///Field `DATA100` writer - Huffman table data 100
pub type DATA100_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA101` reader - Huffman table data 101
pub type DATA101_R = crate::FieldReader;
///Field `DATA101` writer - Huffman table data 101
pub type DATA101_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA102` reader - Huffman table data 102
pub type DATA102_R = crate::FieldReader;
///Field `DATA102` writer - Huffman table data 102
pub type DATA102_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA103` reader - Huffman table data 103
pub type DATA103_R = crate::FieldReader;
///Field `DATA103` writer - Huffman table data 103
pub type DATA103_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 100
    #[inline(always)]
    pub fn data100(&self) -> DATA100_R {
        DATA100_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 101
    #[inline(always)]
    pub fn data101(&self) -> DATA101_R {
        DATA101_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 102
    #[inline(always)]
    pub fn data102(&self) -> DATA102_R {
        DATA102_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 103
    #[inline(always)]
    pub fn data103(&self) -> DATA103_R {
        DATA103_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM25")
            .field("data100", &self.data100())
            .field("data101", &self.data101())
            .field("data102", &self.data102())
            .field("data103", &self.data103())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 100
    #[inline(always)]
    pub fn data100(&mut self) -> DATA100_W<'_, DHTMEM25rs> {
        DATA100_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 101
    #[inline(always)]
    pub fn data101(&mut self) -> DATA101_W<'_, DHTMEM25rs> {
        DATA101_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 102
    #[inline(always)]
    pub fn data102(&mut self) -> DATA102_W<'_, DHTMEM25rs> {
        DATA102_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 103
    #[inline(always)]
    pub fn data103(&mut self) -> DATA103_W<'_, DHTMEM25rs> {
        DATA103_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM25)*/
pub struct DHTMEM25rs;
impl crate::RegisterSpec for DHTMEM25rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem25::R`](R) reader structure
impl crate::Readable for DHTMEM25rs {}
///`write(|w| ..)` method takes [`dhtmem25::W`](W) writer structure
impl crate::Writable for DHTMEM25rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM25 to value 0
impl crate::Resettable for DHTMEM25rs {}
