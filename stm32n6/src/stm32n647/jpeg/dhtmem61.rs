///Register `DHTMEM61` reader
pub type R = crate::R<DHTMEM61rs>;
///Register `DHTMEM61` writer
pub type W = crate::W<DHTMEM61rs>;
///Field `DATA244` reader - Huffman table data 244
pub type DATA244_R = crate::FieldReader;
///Field `DATA244` writer - Huffman table data 244
pub type DATA244_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA245` reader - Huffman table data 245
pub type DATA245_R = crate::FieldReader;
///Field `DATA245` writer - Huffman table data 245
pub type DATA245_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA246` reader - Huffman table data 246
pub type DATA246_R = crate::FieldReader;
///Field `DATA246` writer - Huffman table data 246
pub type DATA246_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA247` reader - Huffman table data 247
pub type DATA247_R = crate::FieldReader;
///Field `DATA247` writer - Huffman table data 247
pub type DATA247_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 244
    #[inline(always)]
    pub fn data244(&self) -> DATA244_R {
        DATA244_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 245
    #[inline(always)]
    pub fn data245(&self) -> DATA245_R {
        DATA245_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 246
    #[inline(always)]
    pub fn data246(&self) -> DATA246_R {
        DATA246_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 247
    #[inline(always)]
    pub fn data247(&self) -> DATA247_R {
        DATA247_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM61")
            .field("data244", &self.data244())
            .field("data245", &self.data245())
            .field("data246", &self.data246())
            .field("data247", &self.data247())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 244
    #[inline(always)]
    pub fn data244(&mut self) -> DATA244_W<'_, DHTMEM61rs> {
        DATA244_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 245
    #[inline(always)]
    pub fn data245(&mut self) -> DATA245_W<'_, DHTMEM61rs> {
        DATA245_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 246
    #[inline(always)]
    pub fn data246(&mut self) -> DATA246_W<'_, DHTMEM61rs> {
        DATA246_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 247
    #[inline(always)]
    pub fn data247(&mut self) -> DATA247_W<'_, DHTMEM61rs> {
        DATA247_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem61::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem61::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM61)*/
pub struct DHTMEM61rs;
impl crate::RegisterSpec for DHTMEM61rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem61::R`](R) reader structure
impl crate::Readable for DHTMEM61rs {}
///`write(|w| ..)` method takes [`dhtmem61::W`](W) writer structure
impl crate::Writable for DHTMEM61rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM61 to value 0
impl crate::Resettable for DHTMEM61rs {}
