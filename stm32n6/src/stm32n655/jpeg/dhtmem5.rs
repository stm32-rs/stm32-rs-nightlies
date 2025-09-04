///Register `DHTMEM5` reader
pub type R = crate::R<DHTMEM5rs>;
///Register `DHTMEM5` writer
pub type W = crate::W<DHTMEM5rs>;
///Field `DATA20` reader - Huffman table data 20
pub type DATA20_R = crate::FieldReader;
///Field `DATA20` writer - Huffman table data 20
pub type DATA20_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA21` reader - Huffman table data 21
pub type DATA21_R = crate::FieldReader;
///Field `DATA21` writer - Huffman table data 21
pub type DATA21_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA22` reader - Huffman table data 22
pub type DATA22_R = crate::FieldReader;
///Field `DATA22` writer - Huffman table data 22
pub type DATA22_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA23` reader - Huffman table data 23
pub type DATA23_R = crate::FieldReader;
///Field `DATA23` writer - Huffman table data 23
pub type DATA23_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 20
    #[inline(always)]
    pub fn data20(&self) -> DATA20_R {
        DATA20_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 21
    #[inline(always)]
    pub fn data21(&self) -> DATA21_R {
        DATA21_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 22
    #[inline(always)]
    pub fn data22(&self) -> DATA22_R {
        DATA22_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 23
    #[inline(always)]
    pub fn data23(&self) -> DATA23_R {
        DATA23_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM5")
            .field("data20", &self.data20())
            .field("data21", &self.data21())
            .field("data22", &self.data22())
            .field("data23", &self.data23())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 20
    #[inline(always)]
    pub fn data20(&mut self) -> DATA20_W<DHTMEM5rs> {
        DATA20_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 21
    #[inline(always)]
    pub fn data21(&mut self) -> DATA21_W<DHTMEM5rs> {
        DATA21_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 22
    #[inline(always)]
    pub fn data22(&mut self) -> DATA22_W<DHTMEM5rs> {
        DATA22_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 23
    #[inline(always)]
    pub fn data23(&mut self) -> DATA23_W<DHTMEM5rs> {
        DATA23_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM5)*/
pub struct DHTMEM5rs;
impl crate::RegisterSpec for DHTMEM5rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem5::R`](R) reader structure
impl crate::Readable for DHTMEM5rs {}
///`write(|w| ..)` method takes [`dhtmem5::W`](W) writer structure
impl crate::Writable for DHTMEM5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM5 to value 0
impl crate::Resettable for DHTMEM5rs {}
