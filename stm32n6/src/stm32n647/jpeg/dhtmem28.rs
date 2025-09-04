///Register `DHTMEM28` reader
pub type R = crate::R<DHTMEM28rs>;
///Register `DHTMEM28` writer
pub type W = crate::W<DHTMEM28rs>;
///Field `DATA112` reader - Huffman table data 112
pub type DATA112_R = crate::FieldReader;
///Field `DATA112` writer - Huffman table data 112
pub type DATA112_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA113` reader - Huffman table data 113
pub type DATA113_R = crate::FieldReader;
///Field `DATA113` writer - Huffman table data 113
pub type DATA113_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA114` reader - Huffman table data 114
pub type DATA114_R = crate::FieldReader;
///Field `DATA114` writer - Huffman table data 114
pub type DATA114_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA115` reader - Huffman table data 115
pub type DATA115_R = crate::FieldReader;
///Field `DATA115` writer - Huffman table data 115
pub type DATA115_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 112
    #[inline(always)]
    pub fn data112(&self) -> DATA112_R {
        DATA112_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 113
    #[inline(always)]
    pub fn data113(&self) -> DATA113_R {
        DATA113_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 114
    #[inline(always)]
    pub fn data114(&self) -> DATA114_R {
        DATA114_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 115
    #[inline(always)]
    pub fn data115(&self) -> DATA115_R {
        DATA115_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM28")
            .field("data112", &self.data112())
            .field("data113", &self.data113())
            .field("data114", &self.data114())
            .field("data115", &self.data115())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 112
    #[inline(always)]
    pub fn data112(&mut self) -> DATA112_W<DHTMEM28rs> {
        DATA112_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 113
    #[inline(always)]
    pub fn data113(&mut self) -> DATA113_W<DHTMEM28rs> {
        DATA113_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 114
    #[inline(always)]
    pub fn data114(&mut self) -> DATA114_W<DHTMEM28rs> {
        DATA114_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 115
    #[inline(always)]
    pub fn data115(&mut self) -> DATA115_W<DHTMEM28rs> {
        DATA115_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM28)*/
pub struct DHTMEM28rs;
impl crate::RegisterSpec for DHTMEM28rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem28::R`](R) reader structure
impl crate::Readable for DHTMEM28rs {}
///`write(|w| ..)` method takes [`dhtmem28::W`](W) writer structure
impl crate::Writable for DHTMEM28rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM28 to value 0
impl crate::Resettable for DHTMEM28rs {}
