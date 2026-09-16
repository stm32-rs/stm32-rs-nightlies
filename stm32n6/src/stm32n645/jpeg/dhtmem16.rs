///Register `DHTMEM16` reader
pub type R = crate::R<DHTMEM16rs>;
///Register `DHTMEM16` writer
pub type W = crate::W<DHTMEM16rs>;
///Field `DATA64` reader - Huffman table data 64
pub type DATA64_R = crate::FieldReader;
///Field `DATA64` writer - Huffman table data 64
pub type DATA64_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA65` reader - Huffman table data 65
pub type DATA65_R = crate::FieldReader;
///Field `DATA65` writer - Huffman table data 65
pub type DATA65_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA66` reader - Huffman table data 66
pub type DATA66_R = crate::FieldReader;
///Field `DATA66` writer - Huffman table data 66
pub type DATA66_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA67` reader - Huffman table data 67
pub type DATA67_R = crate::FieldReader;
///Field `DATA67` writer - Huffman table data 67
pub type DATA67_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 64
    #[inline(always)]
    pub fn data64(&self) -> DATA64_R {
        DATA64_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 65
    #[inline(always)]
    pub fn data65(&self) -> DATA65_R {
        DATA65_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 66
    #[inline(always)]
    pub fn data66(&self) -> DATA66_R {
        DATA66_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 67
    #[inline(always)]
    pub fn data67(&self) -> DATA67_R {
        DATA67_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM16")
            .field("data64", &self.data64())
            .field("data65", &self.data65())
            .field("data66", &self.data66())
            .field("data67", &self.data67())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 64
    #[inline(always)]
    pub fn data64(&mut self) -> DATA64_W<'_, DHTMEM16rs> {
        DATA64_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 65
    #[inline(always)]
    pub fn data65(&mut self) -> DATA65_W<'_, DHTMEM16rs> {
        DATA65_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 66
    #[inline(always)]
    pub fn data66(&mut self) -> DATA66_W<'_, DHTMEM16rs> {
        DATA66_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 67
    #[inline(always)]
    pub fn data67(&mut self) -> DATA67_W<'_, DHTMEM16rs> {
        DATA67_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM16)*/
pub struct DHTMEM16rs;
impl crate::RegisterSpec for DHTMEM16rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem16::R`](R) reader structure
impl crate::Readable for DHTMEM16rs {}
///`write(|w| ..)` method takes [`dhtmem16::W`](W) writer structure
impl crate::Writable for DHTMEM16rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM16 to value 0
impl crate::Resettable for DHTMEM16rs {}
