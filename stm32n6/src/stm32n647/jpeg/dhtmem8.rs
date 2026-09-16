///Register `DHTMEM8` reader
pub type R = crate::R<DHTMEM8rs>;
///Register `DHTMEM8` writer
pub type W = crate::W<DHTMEM8rs>;
///Field `DATA32` reader - Huffman table data 32
pub type DATA32_R = crate::FieldReader;
///Field `DATA32` writer - Huffman table data 32
pub type DATA32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA33` reader - Huffman table data 33
pub type DATA33_R = crate::FieldReader;
///Field `DATA33` writer - Huffman table data 33
pub type DATA33_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA34` reader - Huffman table data 34
pub type DATA34_R = crate::FieldReader;
///Field `DATA34` writer - Huffman table data 34
pub type DATA34_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA35` reader - Huffman table data 35
pub type DATA35_R = crate::FieldReader;
///Field `DATA35` writer - Huffman table data 35
pub type DATA35_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 32
    #[inline(always)]
    pub fn data32(&self) -> DATA32_R {
        DATA32_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 33
    #[inline(always)]
    pub fn data33(&self) -> DATA33_R {
        DATA33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 34
    #[inline(always)]
    pub fn data34(&self) -> DATA34_R {
        DATA34_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 35
    #[inline(always)]
    pub fn data35(&self) -> DATA35_R {
        DATA35_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM8")
            .field("data32", &self.data32())
            .field("data33", &self.data33())
            .field("data34", &self.data34())
            .field("data35", &self.data35())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 32
    #[inline(always)]
    pub fn data32(&mut self) -> DATA32_W<'_, DHTMEM8rs> {
        DATA32_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 33
    #[inline(always)]
    pub fn data33(&mut self) -> DATA33_W<'_, DHTMEM8rs> {
        DATA33_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 34
    #[inline(always)]
    pub fn data34(&mut self) -> DATA34_W<'_, DHTMEM8rs> {
        DATA34_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 35
    #[inline(always)]
    pub fn data35(&mut self) -> DATA35_W<'_, DHTMEM8rs> {
        DATA35_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM8)*/
pub struct DHTMEM8rs;
impl crate::RegisterSpec for DHTMEM8rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem8::R`](R) reader structure
impl crate::Readable for DHTMEM8rs {}
///`write(|w| ..)` method takes [`dhtmem8::W`](W) writer structure
impl crate::Writable for DHTMEM8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM8 to value 0
impl crate::Resettable for DHTMEM8rs {}
