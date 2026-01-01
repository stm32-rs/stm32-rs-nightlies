///Register `DHTMEM75` reader
pub type R = crate::R<DHTMEM75rs>;
///Register `DHTMEM75` writer
pub type W = crate::W<DHTMEM75rs>;
///Field `DATA300` reader - Huffman table data 300
pub type DATA300_R = crate::FieldReader;
///Field `DATA300` writer - Huffman table data 300
pub type DATA300_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA301` reader - Huffman table data 301
pub type DATA301_R = crate::FieldReader;
///Field `DATA301` writer - Huffman table data 301
pub type DATA301_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA302` reader - Huffman table data 302
pub type DATA302_R = crate::FieldReader;
///Field `DATA302` writer - Huffman table data 302
pub type DATA302_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA303` reader - Huffman table data 303
pub type DATA303_R = crate::FieldReader;
///Field `DATA303` writer - Huffman table data 303
pub type DATA303_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 300
    #[inline(always)]
    pub fn data300(&self) -> DATA300_R {
        DATA300_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 301
    #[inline(always)]
    pub fn data301(&self) -> DATA301_R {
        DATA301_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 302
    #[inline(always)]
    pub fn data302(&self) -> DATA302_R {
        DATA302_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 303
    #[inline(always)]
    pub fn data303(&self) -> DATA303_R {
        DATA303_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM75")
            .field("data300", &self.data300())
            .field("data301", &self.data301())
            .field("data302", &self.data302())
            .field("data303", &self.data303())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 300
    #[inline(always)]
    pub fn data300(&mut self) -> DATA300_W<'_, DHTMEM75rs> {
        DATA300_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 301
    #[inline(always)]
    pub fn data301(&mut self) -> DATA301_W<'_, DHTMEM75rs> {
        DATA301_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 302
    #[inline(always)]
    pub fn data302(&mut self) -> DATA302_W<'_, DHTMEM75rs> {
        DATA302_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 303
    #[inline(always)]
    pub fn data303(&mut self) -> DATA303_W<'_, DHTMEM75rs> {
        DATA303_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem75::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem75::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM75)*/
pub struct DHTMEM75rs;
impl crate::RegisterSpec for DHTMEM75rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem75::R`](R) reader structure
impl crate::Readable for DHTMEM75rs {}
///`write(|w| ..)` method takes [`dhtmem75::W`](W) writer structure
impl crate::Writable for DHTMEM75rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM75 to value 0
impl crate::Resettable for DHTMEM75rs {}
