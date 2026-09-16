///Register `DHTMEM32` reader
pub type R = crate::R<DHTMEM32rs>;
///Register `DHTMEM32` writer
pub type W = crate::W<DHTMEM32rs>;
///Field `DATA128` reader - Huffman table data 128
pub type DATA128_R = crate::FieldReader;
///Field `DATA128` writer - Huffman table data 128
pub type DATA128_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA129` reader - Huffman table data 129
pub type DATA129_R = crate::FieldReader;
///Field `DATA129` writer - Huffman table data 129
pub type DATA129_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA130` reader - Huffman table data 130
pub type DATA130_R = crate::FieldReader;
///Field `DATA130` writer - Huffman table data 130
pub type DATA130_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA131` reader - Huffman table data 131
pub type DATA131_R = crate::FieldReader;
///Field `DATA131` writer - Huffman table data 131
pub type DATA131_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 128
    #[inline(always)]
    pub fn data128(&self) -> DATA128_R {
        DATA128_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 129
    #[inline(always)]
    pub fn data129(&self) -> DATA129_R {
        DATA129_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 130
    #[inline(always)]
    pub fn data130(&self) -> DATA130_R {
        DATA130_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 131
    #[inline(always)]
    pub fn data131(&self) -> DATA131_R {
        DATA131_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM32")
            .field("data128", &self.data128())
            .field("data129", &self.data129())
            .field("data130", &self.data130())
            .field("data131", &self.data131())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 128
    #[inline(always)]
    pub fn data128(&mut self) -> DATA128_W<'_, DHTMEM32rs> {
        DATA128_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 129
    #[inline(always)]
    pub fn data129(&mut self) -> DATA129_W<'_, DHTMEM32rs> {
        DATA129_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 130
    #[inline(always)]
    pub fn data130(&mut self) -> DATA130_W<'_, DHTMEM32rs> {
        DATA130_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 131
    #[inline(always)]
    pub fn data131(&mut self) -> DATA131_W<'_, DHTMEM32rs> {
        DATA131_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM32)*/
pub struct DHTMEM32rs;
impl crate::RegisterSpec for DHTMEM32rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem32::R`](R) reader structure
impl crate::Readable for DHTMEM32rs {}
///`write(|w| ..)` method takes [`dhtmem32::W`](W) writer structure
impl crate::Writable for DHTMEM32rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM32 to value 0
impl crate::Resettable for DHTMEM32rs {}
