///Register `DHTMEM86` reader
pub type R = crate::R<DHTMEM86rs>;
///Register `DHTMEM86` writer
pub type W = crate::W<DHTMEM86rs>;
///Field `DATA344` reader - Huffman table data 344
pub type DATA344_R = crate::FieldReader;
///Field `DATA344` writer - Huffman table data 344
pub type DATA344_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA345` reader - Huffman table data 345
pub type DATA345_R = crate::FieldReader;
///Field `DATA345` writer - Huffman table data 345
pub type DATA345_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA346` reader - Huffman table data 346
pub type DATA346_R = crate::FieldReader;
///Field `DATA346` writer - Huffman table data 346
pub type DATA346_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA347` reader - Huffman table data 347
pub type DATA347_R = crate::FieldReader;
///Field `DATA347` writer - Huffman table data 347
pub type DATA347_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 344
    #[inline(always)]
    pub fn data344(&self) -> DATA344_R {
        DATA344_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 345
    #[inline(always)]
    pub fn data345(&self) -> DATA345_R {
        DATA345_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 346
    #[inline(always)]
    pub fn data346(&self) -> DATA346_R {
        DATA346_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 347
    #[inline(always)]
    pub fn data347(&self) -> DATA347_R {
        DATA347_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM86")
            .field("data344", &self.data344())
            .field("data345", &self.data345())
            .field("data346", &self.data346())
            .field("data347", &self.data347())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 344
    #[inline(always)]
    pub fn data344(&mut self) -> DATA344_W<'_, DHTMEM86rs> {
        DATA344_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 345
    #[inline(always)]
    pub fn data345(&mut self) -> DATA345_W<'_, DHTMEM86rs> {
        DATA345_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 346
    #[inline(always)]
    pub fn data346(&mut self) -> DATA346_W<'_, DHTMEM86rs> {
        DATA346_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 347
    #[inline(always)]
    pub fn data347(&mut self) -> DATA347_W<'_, DHTMEM86rs> {
        DATA347_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem86::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem86::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM86)*/
pub struct DHTMEM86rs;
impl crate::RegisterSpec for DHTMEM86rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem86::R`](R) reader structure
impl crate::Readable for DHTMEM86rs {}
///`write(|w| ..)` method takes [`dhtmem86::W`](W) writer structure
impl crate::Writable for DHTMEM86rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM86 to value 0
impl crate::Resettable for DHTMEM86rs {}
