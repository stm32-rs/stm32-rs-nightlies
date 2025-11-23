///Register `DHTMEM44` reader
pub type R = crate::R<DHTMEM44rs>;
///Register `DHTMEM44` writer
pub type W = crate::W<DHTMEM44rs>;
///Field `DATA176` reader - Huffman table data 176
pub type DATA176_R = crate::FieldReader;
///Field `DATA176` writer - Huffman table data 176
pub type DATA176_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA177` reader - Huffman table data 177
pub type DATA177_R = crate::FieldReader;
///Field `DATA177` writer - Huffman table data 177
pub type DATA177_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA178` reader - Huffman table data 178
pub type DATA178_R = crate::FieldReader;
///Field `DATA178` writer - Huffman table data 178
pub type DATA178_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA179` reader - Huffman table data 179
pub type DATA179_R = crate::FieldReader;
///Field `DATA179` writer - Huffman table data 179
pub type DATA179_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 176
    #[inline(always)]
    pub fn data176(&self) -> DATA176_R {
        DATA176_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 177
    #[inline(always)]
    pub fn data177(&self) -> DATA177_R {
        DATA177_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 178
    #[inline(always)]
    pub fn data178(&self) -> DATA178_R {
        DATA178_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 179
    #[inline(always)]
    pub fn data179(&self) -> DATA179_R {
        DATA179_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM44")
            .field("data176", &self.data176())
            .field("data177", &self.data177())
            .field("data178", &self.data178())
            .field("data179", &self.data179())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 176
    #[inline(always)]
    pub fn data176(&mut self) -> DATA176_W<'_, DHTMEM44rs> {
        DATA176_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 177
    #[inline(always)]
    pub fn data177(&mut self) -> DATA177_W<'_, DHTMEM44rs> {
        DATA177_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 178
    #[inline(always)]
    pub fn data178(&mut self) -> DATA178_W<'_, DHTMEM44rs> {
        DATA178_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 179
    #[inline(always)]
    pub fn data179(&mut self) -> DATA179_W<'_, DHTMEM44rs> {
        DATA179_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM44)*/
pub struct DHTMEM44rs;
impl crate::RegisterSpec for DHTMEM44rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem44::R`](R) reader structure
impl crate::Readable for DHTMEM44rs {}
///`write(|w| ..)` method takes [`dhtmem44::W`](W) writer structure
impl crate::Writable for DHTMEM44rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM44 to value 0
impl crate::Resettable for DHTMEM44rs {}
