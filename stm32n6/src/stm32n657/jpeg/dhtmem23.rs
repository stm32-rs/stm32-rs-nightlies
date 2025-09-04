///Register `DHTMEM23` reader
pub type R = crate::R<DHTMEM23rs>;
///Register `DHTMEM23` writer
pub type W = crate::W<DHTMEM23rs>;
///Field `DATA92` reader - Huffman table data 92
pub type DATA92_R = crate::FieldReader;
///Field `DATA92` writer - Huffman table data 92
pub type DATA92_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA93` reader - Huffman table data 93
pub type DATA93_R = crate::FieldReader;
///Field `DATA93` writer - Huffman table data 93
pub type DATA93_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA94` reader - Huffman table data 94
pub type DATA94_R = crate::FieldReader;
///Field `DATA94` writer - Huffman table data 94
pub type DATA94_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA95` reader - Huffman table data 95
pub type DATA95_R = crate::FieldReader;
///Field `DATA95` writer - Huffman table data 95
pub type DATA95_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 92
    #[inline(always)]
    pub fn data92(&self) -> DATA92_R {
        DATA92_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 93
    #[inline(always)]
    pub fn data93(&self) -> DATA93_R {
        DATA93_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 94
    #[inline(always)]
    pub fn data94(&self) -> DATA94_R {
        DATA94_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 95
    #[inline(always)]
    pub fn data95(&self) -> DATA95_R {
        DATA95_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM23")
            .field("data92", &self.data92())
            .field("data93", &self.data93())
            .field("data94", &self.data94())
            .field("data95", &self.data95())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 92
    #[inline(always)]
    pub fn data92(&mut self) -> DATA92_W<DHTMEM23rs> {
        DATA92_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 93
    #[inline(always)]
    pub fn data93(&mut self) -> DATA93_W<DHTMEM23rs> {
        DATA93_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 94
    #[inline(always)]
    pub fn data94(&mut self) -> DATA94_W<DHTMEM23rs> {
        DATA94_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 95
    #[inline(always)]
    pub fn data95(&mut self) -> DATA95_W<DHTMEM23rs> {
        DATA95_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM23)*/
pub struct DHTMEM23rs;
impl crate::RegisterSpec for DHTMEM23rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem23::R`](R) reader structure
impl crate::Readable for DHTMEM23rs {}
///`write(|w| ..)` method takes [`dhtmem23::W`](W) writer structure
impl crate::Writable for DHTMEM23rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM23 to value 0
impl crate::Resettable for DHTMEM23rs {}
