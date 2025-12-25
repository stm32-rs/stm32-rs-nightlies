///Register `DHTMEM98` reader
pub type R = crate::R<DHTMEM98rs>;
///Register `DHTMEM98` writer
pub type W = crate::W<DHTMEM98rs>;
///Field `DATA392` reader - Huffman table data 392
pub type DATA392_R = crate::FieldReader;
///Field `DATA392` writer - Huffman table data 392
pub type DATA392_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA393` reader - Huffman table data 393
pub type DATA393_R = crate::FieldReader;
///Field `DATA393` writer - Huffman table data 393
pub type DATA393_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA394` reader - Huffman table data 394
pub type DATA394_R = crate::FieldReader;
///Field `DATA394` writer - Huffman table data 394
pub type DATA394_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA395` reader - Huffman table data 395
pub type DATA395_R = crate::FieldReader;
///Field `DATA395` writer - Huffman table data 395
pub type DATA395_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 392
    #[inline(always)]
    pub fn data392(&self) -> DATA392_R {
        DATA392_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 393
    #[inline(always)]
    pub fn data393(&self) -> DATA393_R {
        DATA393_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 394
    #[inline(always)]
    pub fn data394(&self) -> DATA394_R {
        DATA394_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 395
    #[inline(always)]
    pub fn data395(&self) -> DATA395_R {
        DATA395_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM98")
            .field("data392", &self.data392())
            .field("data393", &self.data393())
            .field("data394", &self.data394())
            .field("data395", &self.data395())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 392
    #[inline(always)]
    pub fn data392(&mut self) -> DATA392_W<'_, DHTMEM98rs> {
        DATA392_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 393
    #[inline(always)]
    pub fn data393(&mut self) -> DATA393_W<'_, DHTMEM98rs> {
        DATA393_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 394
    #[inline(always)]
    pub fn data394(&mut self) -> DATA394_W<'_, DHTMEM98rs> {
        DATA394_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 395
    #[inline(always)]
    pub fn data395(&mut self) -> DATA395_W<'_, DHTMEM98rs> {
        DATA395_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem98::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem98::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM98)*/
pub struct DHTMEM98rs;
impl crate::RegisterSpec for DHTMEM98rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem98::R`](R) reader structure
impl crate::Readable for DHTMEM98rs {}
///`write(|w| ..)` method takes [`dhtmem98::W`](W) writer structure
impl crate::Writable for DHTMEM98rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM98 to value 0
impl crate::Resettable for DHTMEM98rs {}
