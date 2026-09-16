///Register `DHTMEM93` reader
pub type R = crate::R<DHTMEM93rs>;
///Register `DHTMEM93` writer
pub type W = crate::W<DHTMEM93rs>;
///Field `DATA372` reader - Huffman table data 372
pub type DATA372_R = crate::FieldReader;
///Field `DATA372` writer - Huffman table data 372
pub type DATA372_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA373` reader - Huffman table data 373
pub type DATA373_R = crate::FieldReader;
///Field `DATA373` writer - Huffman table data 373
pub type DATA373_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA374` reader - Huffman table data 374
pub type DATA374_R = crate::FieldReader;
///Field `DATA374` writer - Huffman table data 374
pub type DATA374_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA375` reader - Huffman table data 375
pub type DATA375_R = crate::FieldReader;
///Field `DATA375` writer - Huffman table data 375
pub type DATA375_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 372
    #[inline(always)]
    pub fn data372(&self) -> DATA372_R {
        DATA372_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 373
    #[inline(always)]
    pub fn data373(&self) -> DATA373_R {
        DATA373_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 374
    #[inline(always)]
    pub fn data374(&self) -> DATA374_R {
        DATA374_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 375
    #[inline(always)]
    pub fn data375(&self) -> DATA375_R {
        DATA375_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM93")
            .field("data372", &self.data372())
            .field("data373", &self.data373())
            .field("data374", &self.data374())
            .field("data375", &self.data375())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 372
    #[inline(always)]
    pub fn data372(&mut self) -> DATA372_W<'_, DHTMEM93rs> {
        DATA372_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 373
    #[inline(always)]
    pub fn data373(&mut self) -> DATA373_W<'_, DHTMEM93rs> {
        DATA373_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 374
    #[inline(always)]
    pub fn data374(&mut self) -> DATA374_W<'_, DHTMEM93rs> {
        DATA374_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 375
    #[inline(always)]
    pub fn data375(&mut self) -> DATA375_W<'_, DHTMEM93rs> {
        DATA375_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem93::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem93::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM93)*/
pub struct DHTMEM93rs;
impl crate::RegisterSpec for DHTMEM93rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem93::R`](R) reader structure
impl crate::Readable for DHTMEM93rs {}
///`write(|w| ..)` method takes [`dhtmem93::W`](W) writer structure
impl crate::Writable for DHTMEM93rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM93 to value 0
impl crate::Resettable for DHTMEM93rs {}
