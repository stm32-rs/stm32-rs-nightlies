///Register `DHTMEM9` reader
pub type R = crate::R<DHTMEM9rs>;
///Register `DHTMEM9` writer
pub type W = crate::W<DHTMEM9rs>;
///Field `DATA36` reader - Huffman table data 36
pub type DATA36_R = crate::FieldReader;
///Field `DATA36` writer - Huffman table data 36
pub type DATA36_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA37` reader - Huffman table data 37
pub type DATA37_R = crate::FieldReader;
///Field `DATA37` writer - Huffman table data 37
pub type DATA37_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA38` reader - Huffman table data 38
pub type DATA38_R = crate::FieldReader;
///Field `DATA38` writer - Huffman table data 38
pub type DATA38_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA39` reader - Huffman table data 39
pub type DATA39_R = crate::FieldReader;
///Field `DATA39` writer - Huffman table data 39
pub type DATA39_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 36
    #[inline(always)]
    pub fn data36(&self) -> DATA36_R {
        DATA36_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 37
    #[inline(always)]
    pub fn data37(&self) -> DATA37_R {
        DATA37_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 38
    #[inline(always)]
    pub fn data38(&self) -> DATA38_R {
        DATA38_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 39
    #[inline(always)]
    pub fn data39(&self) -> DATA39_R {
        DATA39_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM9")
            .field("data36", &self.data36())
            .field("data37", &self.data37())
            .field("data38", &self.data38())
            .field("data39", &self.data39())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 36
    #[inline(always)]
    pub fn data36(&mut self) -> DATA36_W<'_, DHTMEM9rs> {
        DATA36_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 37
    #[inline(always)]
    pub fn data37(&mut self) -> DATA37_W<'_, DHTMEM9rs> {
        DATA37_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 38
    #[inline(always)]
    pub fn data38(&mut self) -> DATA38_W<'_, DHTMEM9rs> {
        DATA38_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 39
    #[inline(always)]
    pub fn data39(&mut self) -> DATA39_W<'_, DHTMEM9rs> {
        DATA39_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM9)*/
pub struct DHTMEM9rs;
impl crate::RegisterSpec for DHTMEM9rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem9::R`](R) reader structure
impl crate::Readable for DHTMEM9rs {}
///`write(|w| ..)` method takes [`dhtmem9::W`](W) writer structure
impl crate::Writable for DHTMEM9rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM9 to value 0
impl crate::Resettable for DHTMEM9rs {}
