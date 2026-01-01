///Register `DHTMEM101` reader
pub type R = crate::R<DHTMEM101rs>;
///Register `DHTMEM101` writer
pub type W = crate::W<DHTMEM101rs>;
///Field `DATA404` reader - Huffman table data 404
pub type DATA404_R = crate::FieldReader;
///Field `DATA404` writer - Huffman table data 404
pub type DATA404_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA405` reader - Huffman table data 405
pub type DATA405_R = crate::FieldReader;
///Field `DATA405` writer - Huffman table data 405
pub type DATA405_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA406` reader - Huffman table data 406
pub type DATA406_R = crate::FieldReader;
///Field `DATA406` writer - Huffman table data 406
pub type DATA406_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA407` reader - Huffman table data 407
pub type DATA407_R = crate::FieldReader;
///Field `DATA407` writer - Huffman table data 407
pub type DATA407_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 404
    #[inline(always)]
    pub fn data404(&self) -> DATA404_R {
        DATA404_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 405
    #[inline(always)]
    pub fn data405(&self) -> DATA405_R {
        DATA405_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 406
    #[inline(always)]
    pub fn data406(&self) -> DATA406_R {
        DATA406_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 407
    #[inline(always)]
    pub fn data407(&self) -> DATA407_R {
        DATA407_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM101")
            .field("data404", &self.data404())
            .field("data405", &self.data405())
            .field("data406", &self.data406())
            .field("data407", &self.data407())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 404
    #[inline(always)]
    pub fn data404(&mut self) -> DATA404_W<'_, DHTMEM101rs> {
        DATA404_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 405
    #[inline(always)]
    pub fn data405(&mut self) -> DATA405_W<'_, DHTMEM101rs> {
        DATA405_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 406
    #[inline(always)]
    pub fn data406(&mut self) -> DATA406_W<'_, DHTMEM101rs> {
        DATA406_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 407
    #[inline(always)]
    pub fn data407(&mut self) -> DATA407_W<'_, DHTMEM101rs> {
        DATA407_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem101::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem101::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM101)*/
pub struct DHTMEM101rs;
impl crate::RegisterSpec for DHTMEM101rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem101::R`](R) reader structure
impl crate::Readable for DHTMEM101rs {}
///`write(|w| ..)` method takes [`dhtmem101::W`](W) writer structure
impl crate::Writable for DHTMEM101rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM101 to value 0
impl crate::Resettable for DHTMEM101rs {}
