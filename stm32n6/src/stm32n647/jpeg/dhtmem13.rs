///Register `DHTMEM13` reader
pub type R = crate::R<DHTMEM13rs>;
///Register `DHTMEM13` writer
pub type W = crate::W<DHTMEM13rs>;
///Field `DATA52` reader - Huffman table data 52
pub type DATA52_R = crate::FieldReader;
///Field `DATA52` writer - Huffman table data 52
pub type DATA52_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA53` reader - Huffman table data 53
pub type DATA53_R = crate::FieldReader;
///Field `DATA53` writer - Huffman table data 53
pub type DATA53_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA54` reader - Huffman table data 54
pub type DATA54_R = crate::FieldReader;
///Field `DATA54` writer - Huffman table data 54
pub type DATA54_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA55` reader - Huffman table data 55
pub type DATA55_R = crate::FieldReader;
///Field `DATA55` writer - Huffman table data 55
pub type DATA55_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 52
    #[inline(always)]
    pub fn data52(&self) -> DATA52_R {
        DATA52_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 53
    #[inline(always)]
    pub fn data53(&self) -> DATA53_R {
        DATA53_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 54
    #[inline(always)]
    pub fn data54(&self) -> DATA54_R {
        DATA54_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 55
    #[inline(always)]
    pub fn data55(&self) -> DATA55_R {
        DATA55_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM13")
            .field("data52", &self.data52())
            .field("data53", &self.data53())
            .field("data54", &self.data54())
            .field("data55", &self.data55())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 52
    #[inline(always)]
    pub fn data52(&mut self) -> DATA52_W<'_, DHTMEM13rs> {
        DATA52_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 53
    #[inline(always)]
    pub fn data53(&mut self) -> DATA53_W<'_, DHTMEM13rs> {
        DATA53_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 54
    #[inline(always)]
    pub fn data54(&mut self) -> DATA54_W<'_, DHTMEM13rs> {
        DATA54_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 55
    #[inline(always)]
    pub fn data55(&mut self) -> DATA55_W<'_, DHTMEM13rs> {
        DATA55_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM13)*/
pub struct DHTMEM13rs;
impl crate::RegisterSpec for DHTMEM13rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem13::R`](R) reader structure
impl crate::Readable for DHTMEM13rs {}
///`write(|w| ..)` method takes [`dhtmem13::W`](W) writer structure
impl crate::Writable for DHTMEM13rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM13 to value 0
impl crate::Resettable for DHTMEM13rs {}
