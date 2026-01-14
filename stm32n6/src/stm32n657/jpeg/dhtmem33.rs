///Register `DHTMEM33` reader
pub type R = crate::R<DHTMEM33rs>;
///Register `DHTMEM33` writer
pub type W = crate::W<DHTMEM33rs>;
///Field `DATA132` reader - Huffman table data 132
pub type DATA132_R = crate::FieldReader;
///Field `DATA132` writer - Huffman table data 132
pub type DATA132_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA133` reader - Huffman table data 133
pub type DATA133_R = crate::FieldReader;
///Field `DATA133` writer - Huffman table data 133
pub type DATA133_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA134` reader - Huffman table data 134
pub type DATA134_R = crate::FieldReader;
///Field `DATA134` writer - Huffman table data 134
pub type DATA134_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA135` reader - Huffman table data 135
pub type DATA135_R = crate::FieldReader;
///Field `DATA135` writer - Huffman table data 135
pub type DATA135_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 132
    #[inline(always)]
    pub fn data132(&self) -> DATA132_R {
        DATA132_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 133
    #[inline(always)]
    pub fn data133(&self) -> DATA133_R {
        DATA133_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 134
    #[inline(always)]
    pub fn data134(&self) -> DATA134_R {
        DATA134_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 135
    #[inline(always)]
    pub fn data135(&self) -> DATA135_R {
        DATA135_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM33")
            .field("data132", &self.data132())
            .field("data133", &self.data133())
            .field("data134", &self.data134())
            .field("data135", &self.data135())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 132
    #[inline(always)]
    pub fn data132(&mut self) -> DATA132_W<'_, DHTMEM33rs> {
        DATA132_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 133
    #[inline(always)]
    pub fn data133(&mut self) -> DATA133_W<'_, DHTMEM33rs> {
        DATA133_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 134
    #[inline(always)]
    pub fn data134(&mut self) -> DATA134_W<'_, DHTMEM33rs> {
        DATA134_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 135
    #[inline(always)]
    pub fn data135(&mut self) -> DATA135_W<'_, DHTMEM33rs> {
        DATA135_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM33)*/
pub struct DHTMEM33rs;
impl crate::RegisterSpec for DHTMEM33rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem33::R`](R) reader structure
impl crate::Readable for DHTMEM33rs {}
///`write(|w| ..)` method takes [`dhtmem33::W`](W) writer structure
impl crate::Writable for DHTMEM33rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM33 to value 0
impl crate::Resettable for DHTMEM33rs {}
