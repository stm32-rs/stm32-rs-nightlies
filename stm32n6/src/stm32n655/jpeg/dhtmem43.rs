///Register `DHTMEM43` reader
pub type R = crate::R<DHTMEM43rs>;
///Register `DHTMEM43` writer
pub type W = crate::W<DHTMEM43rs>;
///Field `DATA172` reader - Huffman table data 172
pub type DATA172_R = crate::FieldReader;
///Field `DATA172` writer - Huffman table data 172
pub type DATA172_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA173` reader - Huffman table data 173
pub type DATA173_R = crate::FieldReader;
///Field `DATA173` writer - Huffman table data 173
pub type DATA173_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA174` reader - Huffman table data 174
pub type DATA174_R = crate::FieldReader;
///Field `DATA174` writer - Huffman table data 174
pub type DATA174_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA175` reader - Huffman table data 175
pub type DATA175_R = crate::FieldReader;
///Field `DATA175` writer - Huffman table data 175
pub type DATA175_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 172
    #[inline(always)]
    pub fn data172(&self) -> DATA172_R {
        DATA172_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 173
    #[inline(always)]
    pub fn data173(&self) -> DATA173_R {
        DATA173_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 174
    #[inline(always)]
    pub fn data174(&self) -> DATA174_R {
        DATA174_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 175
    #[inline(always)]
    pub fn data175(&self) -> DATA175_R {
        DATA175_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM43")
            .field("data172", &self.data172())
            .field("data173", &self.data173())
            .field("data174", &self.data174())
            .field("data175", &self.data175())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 172
    #[inline(always)]
    pub fn data172(&mut self) -> DATA172_W<'_, DHTMEM43rs> {
        DATA172_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 173
    #[inline(always)]
    pub fn data173(&mut self) -> DATA173_W<'_, DHTMEM43rs> {
        DATA173_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 174
    #[inline(always)]
    pub fn data174(&mut self) -> DATA174_W<'_, DHTMEM43rs> {
        DATA174_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 175
    #[inline(always)]
    pub fn data175(&mut self) -> DATA175_W<'_, DHTMEM43rs> {
        DATA175_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem43::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem43::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM43)*/
pub struct DHTMEM43rs;
impl crate::RegisterSpec for DHTMEM43rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem43::R`](R) reader structure
impl crate::Readable for DHTMEM43rs {}
///`write(|w| ..)` method takes [`dhtmem43::W`](W) writer structure
impl crate::Writable for DHTMEM43rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM43 to value 0
impl crate::Resettable for DHTMEM43rs {}
