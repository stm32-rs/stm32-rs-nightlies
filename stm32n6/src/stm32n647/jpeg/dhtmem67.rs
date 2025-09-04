///Register `DHTMEM67` reader
pub type R = crate::R<DHTMEM67rs>;
///Register `DHTMEM67` writer
pub type W = crate::W<DHTMEM67rs>;
///Field `DATA268` reader - Huffman table data 268
pub type DATA268_R = crate::FieldReader;
///Field `DATA268` writer - Huffman table data 268
pub type DATA268_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA269` reader - Huffman table data 269
pub type DATA269_R = crate::FieldReader;
///Field `DATA269` writer - Huffman table data 269
pub type DATA269_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA270` reader - Huffman table data 270
pub type DATA270_R = crate::FieldReader;
///Field `DATA270` writer - Huffman table data 270
pub type DATA270_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA271` reader - Huffman table data 271
pub type DATA271_R = crate::FieldReader;
///Field `DATA271` writer - Huffman table data 271
pub type DATA271_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 268
    #[inline(always)]
    pub fn data268(&self) -> DATA268_R {
        DATA268_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 269
    #[inline(always)]
    pub fn data269(&self) -> DATA269_R {
        DATA269_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 270
    #[inline(always)]
    pub fn data270(&self) -> DATA270_R {
        DATA270_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 271
    #[inline(always)]
    pub fn data271(&self) -> DATA271_R {
        DATA271_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM67")
            .field("data268", &self.data268())
            .field("data269", &self.data269())
            .field("data270", &self.data270())
            .field("data271", &self.data271())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 268
    #[inline(always)]
    pub fn data268(&mut self) -> DATA268_W<DHTMEM67rs> {
        DATA268_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 269
    #[inline(always)]
    pub fn data269(&mut self) -> DATA269_W<DHTMEM67rs> {
        DATA269_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 270
    #[inline(always)]
    pub fn data270(&mut self) -> DATA270_W<DHTMEM67rs> {
        DATA270_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 271
    #[inline(always)]
    pub fn data271(&mut self) -> DATA271_W<DHTMEM67rs> {
        DATA271_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem67::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem67::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM67)*/
pub struct DHTMEM67rs;
impl crate::RegisterSpec for DHTMEM67rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem67::R`](R) reader structure
impl crate::Readable for DHTMEM67rs {}
///`write(|w| ..)` method takes [`dhtmem67::W`](W) writer structure
impl crate::Writable for DHTMEM67rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM67 to value 0
impl crate::Resettable for DHTMEM67rs {}
