///Register `DHTMEM29` reader
pub type R = crate::R<DHTMEM29rs>;
///Register `DHTMEM29` writer
pub type W = crate::W<DHTMEM29rs>;
///Field `DATA116` reader - Huffman table data 116
pub type DATA116_R = crate::FieldReader;
///Field `DATA116` writer - Huffman table data 116
pub type DATA116_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA117` reader - Huffman table data 117
pub type DATA117_R = crate::FieldReader;
///Field `DATA117` writer - Huffman table data 117
pub type DATA117_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA118` reader - Huffman table data 118
pub type DATA118_R = crate::FieldReader;
///Field `DATA118` writer - Huffman table data 118
pub type DATA118_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA119` reader - Huffman table data 119
pub type DATA119_R = crate::FieldReader;
///Field `DATA119` writer - Huffman table data 119
pub type DATA119_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 116
    #[inline(always)]
    pub fn data116(&self) -> DATA116_R {
        DATA116_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 117
    #[inline(always)]
    pub fn data117(&self) -> DATA117_R {
        DATA117_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 118
    #[inline(always)]
    pub fn data118(&self) -> DATA118_R {
        DATA118_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 119
    #[inline(always)]
    pub fn data119(&self) -> DATA119_R {
        DATA119_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM29")
            .field("data116", &self.data116())
            .field("data117", &self.data117())
            .field("data118", &self.data118())
            .field("data119", &self.data119())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 116
    #[inline(always)]
    pub fn data116(&mut self) -> DATA116_W<'_, DHTMEM29rs> {
        DATA116_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 117
    #[inline(always)]
    pub fn data117(&mut self) -> DATA117_W<'_, DHTMEM29rs> {
        DATA117_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 118
    #[inline(always)]
    pub fn data118(&mut self) -> DATA118_W<'_, DHTMEM29rs> {
        DATA118_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 119
    #[inline(always)]
    pub fn data119(&mut self) -> DATA119_W<'_, DHTMEM29rs> {
        DATA119_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM29)*/
pub struct DHTMEM29rs;
impl crate::RegisterSpec for DHTMEM29rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem29::R`](R) reader structure
impl crate::Readable for DHTMEM29rs {}
///`write(|w| ..)` method takes [`dhtmem29::W`](W) writer structure
impl crate::Writable for DHTMEM29rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM29 to value 0
impl crate::Resettable for DHTMEM29rs {}
