///Register `DHTMEM14` reader
pub type R = crate::R<DHTMEM14rs>;
///Register `DHTMEM14` writer
pub type W = crate::W<DHTMEM14rs>;
///Field `DATA56` reader - Huffman table data 56
pub type DATA56_R = crate::FieldReader;
///Field `DATA56` writer - Huffman table data 56
pub type DATA56_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA57` reader - Huffman table data 57
pub type DATA57_R = crate::FieldReader;
///Field `DATA57` writer - Huffman table data 57
pub type DATA57_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA58` reader - Huffman table data 58
pub type DATA58_R = crate::FieldReader;
///Field `DATA58` writer - Huffman table data 58
pub type DATA58_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA59` reader - Huffman table data 59
pub type DATA59_R = crate::FieldReader;
///Field `DATA59` writer - Huffman table data 59
pub type DATA59_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 56
    #[inline(always)]
    pub fn data56(&self) -> DATA56_R {
        DATA56_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 57
    #[inline(always)]
    pub fn data57(&self) -> DATA57_R {
        DATA57_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 58
    #[inline(always)]
    pub fn data58(&self) -> DATA58_R {
        DATA58_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 59
    #[inline(always)]
    pub fn data59(&self) -> DATA59_R {
        DATA59_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM14")
            .field("data56", &self.data56())
            .field("data57", &self.data57())
            .field("data58", &self.data58())
            .field("data59", &self.data59())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 56
    #[inline(always)]
    pub fn data56(&mut self) -> DATA56_W<'_, DHTMEM14rs> {
        DATA56_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 57
    #[inline(always)]
    pub fn data57(&mut self) -> DATA57_W<'_, DHTMEM14rs> {
        DATA57_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 58
    #[inline(always)]
    pub fn data58(&mut self) -> DATA58_W<'_, DHTMEM14rs> {
        DATA58_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 59
    #[inline(always)]
    pub fn data59(&mut self) -> DATA59_W<'_, DHTMEM14rs> {
        DATA59_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM14)*/
pub struct DHTMEM14rs;
impl crate::RegisterSpec for DHTMEM14rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem14::R`](R) reader structure
impl crate::Readable for DHTMEM14rs {}
///`write(|w| ..)` method takes [`dhtmem14::W`](W) writer structure
impl crate::Writable for DHTMEM14rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM14 to value 0
impl crate::Resettable for DHTMEM14rs {}
