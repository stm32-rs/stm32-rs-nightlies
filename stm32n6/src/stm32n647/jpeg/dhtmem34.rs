///Register `DHTMEM34` reader
pub type R = crate::R<DHTMEM34rs>;
///Register `DHTMEM34` writer
pub type W = crate::W<DHTMEM34rs>;
///Field `DATA136` reader - Huffman table data 136
pub type DATA136_R = crate::FieldReader;
///Field `DATA136` writer - Huffman table data 136
pub type DATA136_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA137` reader - Huffman table data 137
pub type DATA137_R = crate::FieldReader;
///Field `DATA137` writer - Huffman table data 137
pub type DATA137_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA138` reader - Huffman table data 138
pub type DATA138_R = crate::FieldReader;
///Field `DATA138` writer - Huffman table data 138
pub type DATA138_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA139` reader - Huffman table data 139
pub type DATA139_R = crate::FieldReader;
///Field `DATA139` writer - Huffman table data 139
pub type DATA139_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 136
    #[inline(always)]
    pub fn data136(&self) -> DATA136_R {
        DATA136_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 137
    #[inline(always)]
    pub fn data137(&self) -> DATA137_R {
        DATA137_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 138
    #[inline(always)]
    pub fn data138(&self) -> DATA138_R {
        DATA138_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 139
    #[inline(always)]
    pub fn data139(&self) -> DATA139_R {
        DATA139_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM34")
            .field("data136", &self.data136())
            .field("data137", &self.data137())
            .field("data138", &self.data138())
            .field("data139", &self.data139())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 136
    #[inline(always)]
    pub fn data136(&mut self) -> DATA136_W<'_, DHTMEM34rs> {
        DATA136_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 137
    #[inline(always)]
    pub fn data137(&mut self) -> DATA137_W<'_, DHTMEM34rs> {
        DATA137_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 138
    #[inline(always)]
    pub fn data138(&mut self) -> DATA138_W<'_, DHTMEM34rs> {
        DATA138_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 139
    #[inline(always)]
    pub fn data139(&mut self) -> DATA139_W<'_, DHTMEM34rs> {
        DATA139_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM34)*/
pub struct DHTMEM34rs;
impl crate::RegisterSpec for DHTMEM34rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem34::R`](R) reader structure
impl crate::Readable for DHTMEM34rs {}
///`write(|w| ..)` method takes [`dhtmem34::W`](W) writer structure
impl crate::Writable for DHTMEM34rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM34 to value 0
impl crate::Resettable for DHTMEM34rs {}
