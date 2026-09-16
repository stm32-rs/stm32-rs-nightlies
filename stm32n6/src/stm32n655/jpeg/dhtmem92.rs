///Register `DHTMEM92` reader
pub type R = crate::R<DHTMEM92rs>;
///Register `DHTMEM92` writer
pub type W = crate::W<DHTMEM92rs>;
///Field `DATA368` reader - Huffman table data 368
pub type DATA368_R = crate::FieldReader;
///Field `DATA368` writer - Huffman table data 368
pub type DATA368_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA369` reader - Huffman table data 369
pub type DATA369_R = crate::FieldReader;
///Field `DATA369` writer - Huffman table data 369
pub type DATA369_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA370` reader - Huffman table data 370
pub type DATA370_R = crate::FieldReader;
///Field `DATA370` writer - Huffman table data 370
pub type DATA370_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA371` reader - Huffman table data 371
pub type DATA371_R = crate::FieldReader;
///Field `DATA371` writer - Huffman table data 371
pub type DATA371_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 368
    #[inline(always)]
    pub fn data368(&self) -> DATA368_R {
        DATA368_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 369
    #[inline(always)]
    pub fn data369(&self) -> DATA369_R {
        DATA369_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 370
    #[inline(always)]
    pub fn data370(&self) -> DATA370_R {
        DATA370_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 371
    #[inline(always)]
    pub fn data371(&self) -> DATA371_R {
        DATA371_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM92")
            .field("data368", &self.data368())
            .field("data369", &self.data369())
            .field("data370", &self.data370())
            .field("data371", &self.data371())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 368
    #[inline(always)]
    pub fn data368(&mut self) -> DATA368_W<'_, DHTMEM92rs> {
        DATA368_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 369
    #[inline(always)]
    pub fn data369(&mut self) -> DATA369_W<'_, DHTMEM92rs> {
        DATA369_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 370
    #[inline(always)]
    pub fn data370(&mut self) -> DATA370_W<'_, DHTMEM92rs> {
        DATA370_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 371
    #[inline(always)]
    pub fn data371(&mut self) -> DATA371_W<'_, DHTMEM92rs> {
        DATA371_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem92::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem92::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM92)*/
pub struct DHTMEM92rs;
impl crate::RegisterSpec for DHTMEM92rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem92::R`](R) reader structure
impl crate::Readable for DHTMEM92rs {}
///`write(|w| ..)` method takes [`dhtmem92::W`](W) writer structure
impl crate::Writable for DHTMEM92rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM92 to value 0
impl crate::Resettable for DHTMEM92rs {}
