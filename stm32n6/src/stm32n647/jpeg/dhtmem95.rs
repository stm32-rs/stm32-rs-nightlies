///Register `DHTMEM95` reader
pub type R = crate::R<DHTMEM95rs>;
///Register `DHTMEM95` writer
pub type W = crate::W<DHTMEM95rs>;
///Field `DATA380` reader - Huffman table data 380
pub type DATA380_R = crate::FieldReader;
///Field `DATA380` writer - Huffman table data 380
pub type DATA380_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA381` reader - Huffman table data 381
pub type DATA381_R = crate::FieldReader;
///Field `DATA381` writer - Huffman table data 381
pub type DATA381_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA382` reader - Huffman table data 382
pub type DATA382_R = crate::FieldReader;
///Field `DATA382` writer - Huffman table data 382
pub type DATA382_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA383` reader - Huffman table data 383
pub type DATA383_R = crate::FieldReader;
///Field `DATA383` writer - Huffman table data 383
pub type DATA383_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 380
    #[inline(always)]
    pub fn data380(&self) -> DATA380_R {
        DATA380_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 381
    #[inline(always)]
    pub fn data381(&self) -> DATA381_R {
        DATA381_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 382
    #[inline(always)]
    pub fn data382(&self) -> DATA382_R {
        DATA382_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 383
    #[inline(always)]
    pub fn data383(&self) -> DATA383_R {
        DATA383_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM95")
            .field("data380", &self.data380())
            .field("data381", &self.data381())
            .field("data382", &self.data382())
            .field("data383", &self.data383())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 380
    #[inline(always)]
    pub fn data380(&mut self) -> DATA380_W<'_, DHTMEM95rs> {
        DATA380_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 381
    #[inline(always)]
    pub fn data381(&mut self) -> DATA381_W<'_, DHTMEM95rs> {
        DATA381_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 382
    #[inline(always)]
    pub fn data382(&mut self) -> DATA382_W<'_, DHTMEM95rs> {
        DATA382_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 383
    #[inline(always)]
    pub fn data383(&mut self) -> DATA383_W<'_, DHTMEM95rs> {
        DATA383_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem95::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem95::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM95)*/
pub struct DHTMEM95rs;
impl crate::RegisterSpec for DHTMEM95rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem95::R`](R) reader structure
impl crate::Readable for DHTMEM95rs {}
///`write(|w| ..)` method takes [`dhtmem95::W`](W) writer structure
impl crate::Writable for DHTMEM95rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM95 to value 0
impl crate::Resettable for DHTMEM95rs {}
