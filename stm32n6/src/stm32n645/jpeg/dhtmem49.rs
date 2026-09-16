///Register `DHTMEM49` reader
pub type R = crate::R<DHTMEM49rs>;
///Register `DHTMEM49` writer
pub type W = crate::W<DHTMEM49rs>;
///Field `DATA196` reader - Huffman table data 196
pub type DATA196_R = crate::FieldReader;
///Field `DATA196` writer - Huffman table data 196
pub type DATA196_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA197` reader - Huffman table data 197
pub type DATA197_R = crate::FieldReader;
///Field `DATA197` writer - Huffman table data 197
pub type DATA197_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA198` reader - Huffman table data 198
pub type DATA198_R = crate::FieldReader;
///Field `DATA198` writer - Huffman table data 198
pub type DATA198_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA199` reader - Huffman table data 199
pub type DATA199_R = crate::FieldReader;
///Field `DATA199` writer - Huffman table data 199
pub type DATA199_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 196
    #[inline(always)]
    pub fn data196(&self) -> DATA196_R {
        DATA196_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 197
    #[inline(always)]
    pub fn data197(&self) -> DATA197_R {
        DATA197_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 198
    #[inline(always)]
    pub fn data198(&self) -> DATA198_R {
        DATA198_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 199
    #[inline(always)]
    pub fn data199(&self) -> DATA199_R {
        DATA199_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM49")
            .field("data196", &self.data196())
            .field("data197", &self.data197())
            .field("data198", &self.data198())
            .field("data199", &self.data199())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 196
    #[inline(always)]
    pub fn data196(&mut self) -> DATA196_W<'_, DHTMEM49rs> {
        DATA196_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 197
    #[inline(always)]
    pub fn data197(&mut self) -> DATA197_W<'_, DHTMEM49rs> {
        DATA197_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 198
    #[inline(always)]
    pub fn data198(&mut self) -> DATA198_W<'_, DHTMEM49rs> {
        DATA198_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 199
    #[inline(always)]
    pub fn data199(&mut self) -> DATA199_W<'_, DHTMEM49rs> {
        DATA199_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem49::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem49::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM49)*/
pub struct DHTMEM49rs;
impl crate::RegisterSpec for DHTMEM49rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem49::R`](R) reader structure
impl crate::Readable for DHTMEM49rs {}
///`write(|w| ..)` method takes [`dhtmem49::W`](W) writer structure
impl crate::Writable for DHTMEM49rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM49 to value 0
impl crate::Resettable for DHTMEM49rs {}
