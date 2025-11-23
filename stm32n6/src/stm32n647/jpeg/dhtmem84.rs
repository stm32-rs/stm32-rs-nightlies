///Register `DHTMEM84` reader
pub type R = crate::R<DHTMEM84rs>;
///Register `DHTMEM84` writer
pub type W = crate::W<DHTMEM84rs>;
///Field `DATA336` reader - Huffman table data 336
pub type DATA336_R = crate::FieldReader;
///Field `DATA336` writer - Huffman table data 336
pub type DATA336_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA337` reader - Huffman table data 337
pub type DATA337_R = crate::FieldReader;
///Field `DATA337` writer - Huffman table data 337
pub type DATA337_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA338` reader - Huffman table data 338
pub type DATA338_R = crate::FieldReader;
///Field `DATA338` writer - Huffman table data 338
pub type DATA338_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA339` reader - Huffman table data 339
pub type DATA339_R = crate::FieldReader;
///Field `DATA339` writer - Huffman table data 339
pub type DATA339_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 336
    #[inline(always)]
    pub fn data336(&self) -> DATA336_R {
        DATA336_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 337
    #[inline(always)]
    pub fn data337(&self) -> DATA337_R {
        DATA337_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 338
    #[inline(always)]
    pub fn data338(&self) -> DATA338_R {
        DATA338_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 339
    #[inline(always)]
    pub fn data339(&self) -> DATA339_R {
        DATA339_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM84")
            .field("data336", &self.data336())
            .field("data337", &self.data337())
            .field("data338", &self.data338())
            .field("data339", &self.data339())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 336
    #[inline(always)]
    pub fn data336(&mut self) -> DATA336_W<'_, DHTMEM84rs> {
        DATA336_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 337
    #[inline(always)]
    pub fn data337(&mut self) -> DATA337_W<'_, DHTMEM84rs> {
        DATA337_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 338
    #[inline(always)]
    pub fn data338(&mut self) -> DATA338_W<'_, DHTMEM84rs> {
        DATA338_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 339
    #[inline(always)]
    pub fn data339(&mut self) -> DATA339_W<'_, DHTMEM84rs> {
        DATA339_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem84::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem84::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM84)*/
pub struct DHTMEM84rs;
impl crate::RegisterSpec for DHTMEM84rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem84::R`](R) reader structure
impl crate::Readable for DHTMEM84rs {}
///`write(|w| ..)` method takes [`dhtmem84::W`](W) writer structure
impl crate::Writable for DHTMEM84rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM84 to value 0
impl crate::Resettable for DHTMEM84rs {}
