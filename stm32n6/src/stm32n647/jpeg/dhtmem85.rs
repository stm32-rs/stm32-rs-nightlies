///Register `DHTMEM85` reader
pub type R = crate::R<DHTMEM85rs>;
///Register `DHTMEM85` writer
pub type W = crate::W<DHTMEM85rs>;
///Field `DATA340` reader - Huffman table data 340
pub type DATA340_R = crate::FieldReader;
///Field `DATA340` writer - Huffman table data 340
pub type DATA340_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA341` reader - Huffman table data 341
pub type DATA341_R = crate::FieldReader;
///Field `DATA341` writer - Huffman table data 341
pub type DATA341_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA342` reader - Huffman table data 342
pub type DATA342_R = crate::FieldReader;
///Field `DATA342` writer - Huffman table data 342
pub type DATA342_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA343` reader - Huffman table data 343
pub type DATA343_R = crate::FieldReader;
///Field `DATA343` writer - Huffman table data 343
pub type DATA343_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 340
    #[inline(always)]
    pub fn data340(&self) -> DATA340_R {
        DATA340_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 341
    #[inline(always)]
    pub fn data341(&self) -> DATA341_R {
        DATA341_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 342
    #[inline(always)]
    pub fn data342(&self) -> DATA342_R {
        DATA342_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 343
    #[inline(always)]
    pub fn data343(&self) -> DATA343_R {
        DATA343_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM85")
            .field("data340", &self.data340())
            .field("data341", &self.data341())
            .field("data342", &self.data342())
            .field("data343", &self.data343())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 340
    #[inline(always)]
    pub fn data340(&mut self) -> DATA340_W<'_, DHTMEM85rs> {
        DATA340_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 341
    #[inline(always)]
    pub fn data341(&mut self) -> DATA341_W<'_, DHTMEM85rs> {
        DATA341_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 342
    #[inline(always)]
    pub fn data342(&mut self) -> DATA342_W<'_, DHTMEM85rs> {
        DATA342_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 343
    #[inline(always)]
    pub fn data343(&mut self) -> DATA343_W<'_, DHTMEM85rs> {
        DATA343_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem85::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem85::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM85)*/
pub struct DHTMEM85rs;
impl crate::RegisterSpec for DHTMEM85rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem85::R`](R) reader structure
impl crate::Readable for DHTMEM85rs {}
///`write(|w| ..)` method takes [`dhtmem85::W`](W) writer structure
impl crate::Writable for DHTMEM85rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM85 to value 0
impl crate::Resettable for DHTMEM85rs {}
