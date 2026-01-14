///Register `DHTMEM18` reader
pub type R = crate::R<DHTMEM18rs>;
///Register `DHTMEM18` writer
pub type W = crate::W<DHTMEM18rs>;
///Field `DATA72` reader - Huffman table data 72
pub type DATA72_R = crate::FieldReader;
///Field `DATA72` writer - Huffman table data 72
pub type DATA72_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA73` reader - Huffman table data 73
pub type DATA73_R = crate::FieldReader;
///Field `DATA73` writer - Huffman table data 73
pub type DATA73_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA74` reader - Huffman table data 74
pub type DATA74_R = crate::FieldReader;
///Field `DATA74` writer - Huffman table data 74
pub type DATA74_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA75` reader - Huffman table data 75
pub type DATA75_R = crate::FieldReader;
///Field `DATA75` writer - Huffman table data 75
pub type DATA75_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 72
    #[inline(always)]
    pub fn data72(&self) -> DATA72_R {
        DATA72_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 73
    #[inline(always)]
    pub fn data73(&self) -> DATA73_R {
        DATA73_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 74
    #[inline(always)]
    pub fn data74(&self) -> DATA74_R {
        DATA74_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 75
    #[inline(always)]
    pub fn data75(&self) -> DATA75_R {
        DATA75_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM18")
            .field("data72", &self.data72())
            .field("data73", &self.data73())
            .field("data74", &self.data74())
            .field("data75", &self.data75())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 72
    #[inline(always)]
    pub fn data72(&mut self) -> DATA72_W<'_, DHTMEM18rs> {
        DATA72_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 73
    #[inline(always)]
    pub fn data73(&mut self) -> DATA73_W<'_, DHTMEM18rs> {
        DATA73_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 74
    #[inline(always)]
    pub fn data74(&mut self) -> DATA74_W<'_, DHTMEM18rs> {
        DATA74_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 75
    #[inline(always)]
    pub fn data75(&mut self) -> DATA75_W<'_, DHTMEM18rs> {
        DATA75_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM18)*/
pub struct DHTMEM18rs;
impl crate::RegisterSpec for DHTMEM18rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem18::R`](R) reader structure
impl crate::Readable for DHTMEM18rs {}
///`write(|w| ..)` method takes [`dhtmem18::W`](W) writer structure
impl crate::Writable for DHTMEM18rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM18 to value 0
impl crate::Resettable for DHTMEM18rs {}
