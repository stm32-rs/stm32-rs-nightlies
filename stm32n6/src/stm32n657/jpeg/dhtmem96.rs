///Register `DHTMEM96` reader
pub type R = crate::R<DHTMEM96rs>;
///Register `DHTMEM96` writer
pub type W = crate::W<DHTMEM96rs>;
///Field `DATA384` reader - Huffman table data 384
pub type DATA384_R = crate::FieldReader;
///Field `DATA384` writer - Huffman table data 384
pub type DATA384_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA385` reader - Huffman table data 385
pub type DATA385_R = crate::FieldReader;
///Field `DATA385` writer - Huffman table data 385
pub type DATA385_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA386` reader - Huffman table data 386
pub type DATA386_R = crate::FieldReader;
///Field `DATA386` writer - Huffman table data 386
pub type DATA386_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA387` reader - Huffman table data 387
pub type DATA387_R = crate::FieldReader;
///Field `DATA387` writer - Huffman table data 387
pub type DATA387_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 384
    #[inline(always)]
    pub fn data384(&self) -> DATA384_R {
        DATA384_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 385
    #[inline(always)]
    pub fn data385(&self) -> DATA385_R {
        DATA385_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 386
    #[inline(always)]
    pub fn data386(&self) -> DATA386_R {
        DATA386_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 387
    #[inline(always)]
    pub fn data387(&self) -> DATA387_R {
        DATA387_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM96")
            .field("data384", &self.data384())
            .field("data385", &self.data385())
            .field("data386", &self.data386())
            .field("data387", &self.data387())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 384
    #[inline(always)]
    pub fn data384(&mut self) -> DATA384_W<'_, DHTMEM96rs> {
        DATA384_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 385
    #[inline(always)]
    pub fn data385(&mut self) -> DATA385_W<'_, DHTMEM96rs> {
        DATA385_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 386
    #[inline(always)]
    pub fn data386(&mut self) -> DATA386_W<'_, DHTMEM96rs> {
        DATA386_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 387
    #[inline(always)]
    pub fn data387(&mut self) -> DATA387_W<'_, DHTMEM96rs> {
        DATA387_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem96::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem96::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM96)*/
pub struct DHTMEM96rs;
impl crate::RegisterSpec for DHTMEM96rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem96::R`](R) reader structure
impl crate::Readable for DHTMEM96rs {}
///`write(|w| ..)` method takes [`dhtmem96::W`](W) writer structure
impl crate::Writable for DHTMEM96rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM96 to value 0
impl crate::Resettable for DHTMEM96rs {}
