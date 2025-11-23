///Register `DHTMEM102` reader
pub type R = crate::R<DHTMEM102rs>;
///Register `DHTMEM102` writer
pub type W = crate::W<DHTMEM102rs>;
///Field `DATA408` reader - Huffman table data 408
pub type DATA408_R = crate::FieldReader;
///Field `DATA408` writer - Huffman table data 408
pub type DATA408_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA409` reader - Huffman table data 409
pub type DATA409_R = crate::FieldReader;
///Field `DATA409` writer - Huffman table data 409
pub type DATA409_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA410` reader - Huffman table data 410
pub type DATA410_R = crate::FieldReader;
///Field `DATA410` writer - Huffman table data 410
pub type DATA410_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA411` reader - Huffman table data 411
pub type DATA411_R = crate::FieldReader;
///Field `DATA411` writer - Huffman table data 411
pub type DATA411_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 408
    #[inline(always)]
    pub fn data408(&self) -> DATA408_R {
        DATA408_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 409
    #[inline(always)]
    pub fn data409(&self) -> DATA409_R {
        DATA409_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 410
    #[inline(always)]
    pub fn data410(&self) -> DATA410_R {
        DATA410_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 411
    #[inline(always)]
    pub fn data411(&self) -> DATA411_R {
        DATA411_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM102")
            .field("data408", &self.data408())
            .field("data409", &self.data409())
            .field("data410", &self.data410())
            .field("data411", &self.data411())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 408
    #[inline(always)]
    pub fn data408(&mut self) -> DATA408_W<'_, DHTMEM102rs> {
        DATA408_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 409
    #[inline(always)]
    pub fn data409(&mut self) -> DATA409_W<'_, DHTMEM102rs> {
        DATA409_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 410
    #[inline(always)]
    pub fn data410(&mut self) -> DATA410_W<'_, DHTMEM102rs> {
        DATA410_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 411
    #[inline(always)]
    pub fn data411(&mut self) -> DATA411_W<'_, DHTMEM102rs> {
        DATA411_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem102::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem102::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM102)*/
pub struct DHTMEM102rs;
impl crate::RegisterSpec for DHTMEM102rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem102::R`](R) reader structure
impl crate::Readable for DHTMEM102rs {}
///`write(|w| ..)` method takes [`dhtmem102::W`](W) writer structure
impl crate::Writable for DHTMEM102rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM102 to value 0
impl crate::Resettable for DHTMEM102rs {}
