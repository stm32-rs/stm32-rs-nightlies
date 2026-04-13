///Register `DHTMEM38` reader
pub type R = crate::R<DHTMEM38rs>;
///Register `DHTMEM38` writer
pub type W = crate::W<DHTMEM38rs>;
///Field `DATA152` reader - Huffman table data 152
pub type DATA152_R = crate::FieldReader;
///Field `DATA152` writer - Huffman table data 152
pub type DATA152_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA153` reader - Huffman table data 153
pub type DATA153_R = crate::FieldReader;
///Field `DATA153` writer - Huffman table data 153
pub type DATA153_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA154` reader - Huffman table data 154
pub type DATA154_R = crate::FieldReader;
///Field `DATA154` writer - Huffman table data 154
pub type DATA154_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA155` reader - Huffman table data 155
pub type DATA155_R = crate::FieldReader;
///Field `DATA155` writer - Huffman table data 155
pub type DATA155_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 152
    #[inline(always)]
    pub fn data152(&self) -> DATA152_R {
        DATA152_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 153
    #[inline(always)]
    pub fn data153(&self) -> DATA153_R {
        DATA153_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 154
    #[inline(always)]
    pub fn data154(&self) -> DATA154_R {
        DATA154_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 155
    #[inline(always)]
    pub fn data155(&self) -> DATA155_R {
        DATA155_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM38")
            .field("data152", &self.data152())
            .field("data153", &self.data153())
            .field("data154", &self.data154())
            .field("data155", &self.data155())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 152
    #[inline(always)]
    pub fn data152(&mut self) -> DATA152_W<'_, DHTMEM38rs> {
        DATA152_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 153
    #[inline(always)]
    pub fn data153(&mut self) -> DATA153_W<'_, DHTMEM38rs> {
        DATA153_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 154
    #[inline(always)]
    pub fn data154(&mut self) -> DATA154_W<'_, DHTMEM38rs> {
        DATA154_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 155
    #[inline(always)]
    pub fn data155(&mut self) -> DATA155_W<'_, DHTMEM38rs> {
        DATA155_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM38)*/
pub struct DHTMEM38rs;
impl crate::RegisterSpec for DHTMEM38rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem38::R`](R) reader structure
impl crate::Readable for DHTMEM38rs {}
///`write(|w| ..)` method takes [`dhtmem38::W`](W) writer structure
impl crate::Writable for DHTMEM38rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM38 to value 0
impl crate::Resettable for DHTMEM38rs {}
