///Register `DHTMEM97` reader
pub type R = crate::R<DHTMEM97rs>;
///Register `DHTMEM97` writer
pub type W = crate::W<DHTMEM97rs>;
///Field `DATA388` reader - Huffman table data 388
pub type DATA388_R = crate::FieldReader;
///Field `DATA388` writer - Huffman table data 388
pub type DATA388_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA389` reader - Huffman table data 389
pub type DATA389_R = crate::FieldReader;
///Field `DATA389` writer - Huffman table data 389
pub type DATA389_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA390` reader - Huffman table data 390
pub type DATA390_R = crate::FieldReader;
///Field `DATA390` writer - Huffman table data 390
pub type DATA390_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA391` reader - Huffman table data 391
pub type DATA391_R = crate::FieldReader;
///Field `DATA391` writer - Huffman table data 391
pub type DATA391_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 388
    #[inline(always)]
    pub fn data388(&self) -> DATA388_R {
        DATA388_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 389
    #[inline(always)]
    pub fn data389(&self) -> DATA389_R {
        DATA389_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 390
    #[inline(always)]
    pub fn data390(&self) -> DATA390_R {
        DATA390_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 391
    #[inline(always)]
    pub fn data391(&self) -> DATA391_R {
        DATA391_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM97")
            .field("data388", &self.data388())
            .field("data389", &self.data389())
            .field("data390", &self.data390())
            .field("data391", &self.data391())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 388
    #[inline(always)]
    pub fn data388(&mut self) -> DATA388_W<'_, DHTMEM97rs> {
        DATA388_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 389
    #[inline(always)]
    pub fn data389(&mut self) -> DATA389_W<'_, DHTMEM97rs> {
        DATA389_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 390
    #[inline(always)]
    pub fn data390(&mut self) -> DATA390_W<'_, DHTMEM97rs> {
        DATA390_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 391
    #[inline(always)]
    pub fn data391(&mut self) -> DATA391_W<'_, DHTMEM97rs> {
        DATA391_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem97::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem97::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM97)*/
pub struct DHTMEM97rs;
impl crate::RegisterSpec for DHTMEM97rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem97::R`](R) reader structure
impl crate::Readable for DHTMEM97rs {}
///`write(|w| ..)` method takes [`dhtmem97::W`](W) writer structure
impl crate::Writable for DHTMEM97rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM97 to value 0
impl crate::Resettable for DHTMEM97rs {}
