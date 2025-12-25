///Register `DHTMEM37` reader
pub type R = crate::R<DHTMEM37rs>;
///Register `DHTMEM37` writer
pub type W = crate::W<DHTMEM37rs>;
///Field `DATA148` reader - Huffman table data 148
pub type DATA148_R = crate::FieldReader;
///Field `DATA148` writer - Huffman table data 148
pub type DATA148_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA149` reader - Huffman table data 149
pub type DATA149_R = crate::FieldReader;
///Field `DATA149` writer - Huffman table data 149
pub type DATA149_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA150` reader - Huffman table data 150
pub type DATA150_R = crate::FieldReader;
///Field `DATA150` writer - Huffman table data 150
pub type DATA150_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA151` reader - Huffman table data 151
pub type DATA151_R = crate::FieldReader;
///Field `DATA151` writer - Huffman table data 151
pub type DATA151_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 148
    #[inline(always)]
    pub fn data148(&self) -> DATA148_R {
        DATA148_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 149
    #[inline(always)]
    pub fn data149(&self) -> DATA149_R {
        DATA149_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 150
    #[inline(always)]
    pub fn data150(&self) -> DATA150_R {
        DATA150_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 151
    #[inline(always)]
    pub fn data151(&self) -> DATA151_R {
        DATA151_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM37")
            .field("data148", &self.data148())
            .field("data149", &self.data149())
            .field("data150", &self.data150())
            .field("data151", &self.data151())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 148
    #[inline(always)]
    pub fn data148(&mut self) -> DATA148_W<'_, DHTMEM37rs> {
        DATA148_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 149
    #[inline(always)]
    pub fn data149(&mut self) -> DATA149_W<'_, DHTMEM37rs> {
        DATA149_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 150
    #[inline(always)]
    pub fn data150(&mut self) -> DATA150_W<'_, DHTMEM37rs> {
        DATA150_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 151
    #[inline(always)]
    pub fn data151(&mut self) -> DATA151_W<'_, DHTMEM37rs> {
        DATA151_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem37::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem37::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM37)*/
pub struct DHTMEM37rs;
impl crate::RegisterSpec for DHTMEM37rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem37::R`](R) reader structure
impl crate::Readable for DHTMEM37rs {}
///`write(|w| ..)` method takes [`dhtmem37::W`](W) writer structure
impl crate::Writable for DHTMEM37rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM37 to value 0
impl crate::Resettable for DHTMEM37rs {}
