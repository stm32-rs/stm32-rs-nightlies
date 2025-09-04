///Register `DHTMEM81` reader
pub type R = crate::R<DHTMEM81rs>;
///Register `DHTMEM81` writer
pub type W = crate::W<DHTMEM81rs>;
///Field `DATA324` reader - Huffman table data 324
pub type DATA324_R = crate::FieldReader;
///Field `DATA324` writer - Huffman table data 324
pub type DATA324_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA325` reader - Huffman table data 325
pub type DATA325_R = crate::FieldReader;
///Field `DATA325` writer - Huffman table data 325
pub type DATA325_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA326` reader - Huffman table data 326
pub type DATA326_R = crate::FieldReader;
///Field `DATA326` writer - Huffman table data 326
pub type DATA326_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA327` reader - Huffman table data 327
pub type DATA327_R = crate::FieldReader;
///Field `DATA327` writer - Huffman table data 327
pub type DATA327_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 324
    #[inline(always)]
    pub fn data324(&self) -> DATA324_R {
        DATA324_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 325
    #[inline(always)]
    pub fn data325(&self) -> DATA325_R {
        DATA325_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 326
    #[inline(always)]
    pub fn data326(&self) -> DATA326_R {
        DATA326_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 327
    #[inline(always)]
    pub fn data327(&self) -> DATA327_R {
        DATA327_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM81")
            .field("data324", &self.data324())
            .field("data325", &self.data325())
            .field("data326", &self.data326())
            .field("data327", &self.data327())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 324
    #[inline(always)]
    pub fn data324(&mut self) -> DATA324_W<DHTMEM81rs> {
        DATA324_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 325
    #[inline(always)]
    pub fn data325(&mut self) -> DATA325_W<DHTMEM81rs> {
        DATA325_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 326
    #[inline(always)]
    pub fn data326(&mut self) -> DATA326_W<DHTMEM81rs> {
        DATA326_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 327
    #[inline(always)]
    pub fn data327(&mut self) -> DATA327_W<DHTMEM81rs> {
        DATA327_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem81::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem81::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM81)*/
pub struct DHTMEM81rs;
impl crate::RegisterSpec for DHTMEM81rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem81::R`](R) reader structure
impl crate::Readable for DHTMEM81rs {}
///`write(|w| ..)` method takes [`dhtmem81::W`](W) writer structure
impl crate::Writable for DHTMEM81rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM81 to value 0
impl crate::Resettable for DHTMEM81rs {}
