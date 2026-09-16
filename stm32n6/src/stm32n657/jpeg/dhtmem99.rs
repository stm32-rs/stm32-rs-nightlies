///Register `DHTMEM99` reader
pub type R = crate::R<DHTMEM99rs>;
///Register `DHTMEM99` writer
pub type W = crate::W<DHTMEM99rs>;
///Field `DATA396` reader - Huffman table data 396
pub type DATA396_R = crate::FieldReader;
///Field `DATA396` writer - Huffman table data 396
pub type DATA396_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA397` reader - Huffman table data 397
pub type DATA397_R = crate::FieldReader;
///Field `DATA397` writer - Huffman table data 397
pub type DATA397_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA398` reader - Huffman table data 398
pub type DATA398_R = crate::FieldReader;
///Field `DATA398` writer - Huffman table data 398
pub type DATA398_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA399` reader - Huffman table data 399
pub type DATA399_R = crate::FieldReader;
///Field `DATA399` writer - Huffman table data 399
pub type DATA399_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 396
    #[inline(always)]
    pub fn data396(&self) -> DATA396_R {
        DATA396_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 397
    #[inline(always)]
    pub fn data397(&self) -> DATA397_R {
        DATA397_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 398
    #[inline(always)]
    pub fn data398(&self) -> DATA398_R {
        DATA398_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 399
    #[inline(always)]
    pub fn data399(&self) -> DATA399_R {
        DATA399_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM99")
            .field("data396", &self.data396())
            .field("data397", &self.data397())
            .field("data398", &self.data398())
            .field("data399", &self.data399())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 396
    #[inline(always)]
    pub fn data396(&mut self) -> DATA396_W<'_, DHTMEM99rs> {
        DATA396_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 397
    #[inline(always)]
    pub fn data397(&mut self) -> DATA397_W<'_, DHTMEM99rs> {
        DATA397_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 398
    #[inline(always)]
    pub fn data398(&mut self) -> DATA398_W<'_, DHTMEM99rs> {
        DATA398_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 399
    #[inline(always)]
    pub fn data399(&mut self) -> DATA399_W<'_, DHTMEM99rs> {
        DATA399_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem99::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem99::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM99)*/
pub struct DHTMEM99rs;
impl crate::RegisterSpec for DHTMEM99rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem99::R`](R) reader structure
impl crate::Readable for DHTMEM99rs {}
///`write(|w| ..)` method takes [`dhtmem99::W`](W) writer structure
impl crate::Writable for DHTMEM99rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM99 to value 0
impl crate::Resettable for DHTMEM99rs {}
