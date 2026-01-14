///Register `DHTMEM94` reader
pub type R = crate::R<DHTMEM94rs>;
///Register `DHTMEM94` writer
pub type W = crate::W<DHTMEM94rs>;
///Field `DATA376` reader - Huffman table data 376
pub type DATA376_R = crate::FieldReader;
///Field `DATA376` writer - Huffman table data 376
pub type DATA376_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA377` reader - Huffman table data 377
pub type DATA377_R = crate::FieldReader;
///Field `DATA377` writer - Huffman table data 377
pub type DATA377_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA378` reader - Huffman table data 378
pub type DATA378_R = crate::FieldReader;
///Field `DATA378` writer - Huffman table data 378
pub type DATA378_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA379` reader - Huffman table data 379
pub type DATA379_R = crate::FieldReader;
///Field `DATA379` writer - Huffman table data 379
pub type DATA379_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 376
    #[inline(always)]
    pub fn data376(&self) -> DATA376_R {
        DATA376_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 377
    #[inline(always)]
    pub fn data377(&self) -> DATA377_R {
        DATA377_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 378
    #[inline(always)]
    pub fn data378(&self) -> DATA378_R {
        DATA378_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 379
    #[inline(always)]
    pub fn data379(&self) -> DATA379_R {
        DATA379_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM94")
            .field("data376", &self.data376())
            .field("data377", &self.data377())
            .field("data378", &self.data378())
            .field("data379", &self.data379())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 376
    #[inline(always)]
    pub fn data376(&mut self) -> DATA376_W<'_, DHTMEM94rs> {
        DATA376_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 377
    #[inline(always)]
    pub fn data377(&mut self) -> DATA377_W<'_, DHTMEM94rs> {
        DATA377_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 378
    #[inline(always)]
    pub fn data378(&mut self) -> DATA378_W<'_, DHTMEM94rs> {
        DATA378_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 379
    #[inline(always)]
    pub fn data379(&mut self) -> DATA379_W<'_, DHTMEM94rs> {
        DATA379_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem94::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem94::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM94)*/
pub struct DHTMEM94rs;
impl crate::RegisterSpec for DHTMEM94rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem94::R`](R) reader structure
impl crate::Readable for DHTMEM94rs {}
///`write(|w| ..)` method takes [`dhtmem94::W`](W) writer structure
impl crate::Writable for DHTMEM94rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM94 to value 0
impl crate::Resettable for DHTMEM94rs {}
