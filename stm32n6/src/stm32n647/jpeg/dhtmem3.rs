///Register `DHTMEM3` reader
pub type R = crate::R<DHTMEM3rs>;
///Register `DHTMEM3` writer
pub type W = crate::W<DHTMEM3rs>;
///Field `DATA12` reader - Huffman table data 12
pub type DATA12_R = crate::FieldReader;
///Field `DATA12` writer - Huffman table data 12
pub type DATA12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA13` reader - Huffman table data 13
pub type DATA13_R = crate::FieldReader;
///Field `DATA13` writer - Huffman table data 13
pub type DATA13_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA14` reader - Huffman table data 14
pub type DATA14_R = crate::FieldReader;
///Field `DATA14` writer - Huffman table data 14
pub type DATA14_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA15` reader - Huffman table data 15
pub type DATA15_R = crate::FieldReader;
///Field `DATA15` writer - Huffman table data 15
pub type DATA15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 12
    #[inline(always)]
    pub fn data12(&self) -> DATA12_R {
        DATA12_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 13
    #[inline(always)]
    pub fn data13(&self) -> DATA13_R {
        DATA13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 14
    #[inline(always)]
    pub fn data14(&self) -> DATA14_R {
        DATA14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 15
    #[inline(always)]
    pub fn data15(&self) -> DATA15_R {
        DATA15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM3")
            .field("data12", &self.data12())
            .field("data13", &self.data13())
            .field("data14", &self.data14())
            .field("data15", &self.data15())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 12
    #[inline(always)]
    pub fn data12(&mut self) -> DATA12_W<'_, DHTMEM3rs> {
        DATA12_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 13
    #[inline(always)]
    pub fn data13(&mut self) -> DATA13_W<'_, DHTMEM3rs> {
        DATA13_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 14
    #[inline(always)]
    pub fn data14(&mut self) -> DATA14_W<'_, DHTMEM3rs> {
        DATA14_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 15
    #[inline(always)]
    pub fn data15(&mut self) -> DATA15_W<'_, DHTMEM3rs> {
        DATA15_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM3)*/
pub struct DHTMEM3rs;
impl crate::RegisterSpec for DHTMEM3rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem3::R`](R) reader structure
impl crate::Readable for DHTMEM3rs {}
///`write(|w| ..)` method takes [`dhtmem3::W`](W) writer structure
impl crate::Writable for DHTMEM3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM3 to value 0
impl crate::Resettable for DHTMEM3rs {}
