///Register `DHTMEM15` reader
pub type R = crate::R<DHTMEM15rs>;
///Register `DHTMEM15` writer
pub type W = crate::W<DHTMEM15rs>;
///Field `DATA60` reader - Huffman table data 60
pub type DATA60_R = crate::FieldReader;
///Field `DATA60` writer - Huffman table data 60
pub type DATA60_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA61` reader - Huffman table data 61
pub type DATA61_R = crate::FieldReader;
///Field `DATA61` writer - Huffman table data 61
pub type DATA61_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA62` reader - Huffman table data 62
pub type DATA62_R = crate::FieldReader;
///Field `DATA62` writer - Huffman table data 62
pub type DATA62_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA63` reader - Huffman table data 63
pub type DATA63_R = crate::FieldReader;
///Field `DATA63` writer - Huffman table data 63
pub type DATA63_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 60
    #[inline(always)]
    pub fn data60(&self) -> DATA60_R {
        DATA60_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 61
    #[inline(always)]
    pub fn data61(&self) -> DATA61_R {
        DATA61_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 62
    #[inline(always)]
    pub fn data62(&self) -> DATA62_R {
        DATA62_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 63
    #[inline(always)]
    pub fn data63(&self) -> DATA63_R {
        DATA63_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM15")
            .field("data60", &self.data60())
            .field("data61", &self.data61())
            .field("data62", &self.data62())
            .field("data63", &self.data63())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 60
    #[inline(always)]
    pub fn data60(&mut self) -> DATA60_W<'_, DHTMEM15rs> {
        DATA60_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 61
    #[inline(always)]
    pub fn data61(&mut self) -> DATA61_W<'_, DHTMEM15rs> {
        DATA61_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 62
    #[inline(always)]
    pub fn data62(&mut self) -> DATA62_W<'_, DHTMEM15rs> {
        DATA62_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 63
    #[inline(always)]
    pub fn data63(&mut self) -> DATA63_W<'_, DHTMEM15rs> {
        DATA63_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM15)*/
pub struct DHTMEM15rs;
impl crate::RegisterSpec for DHTMEM15rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem15::R`](R) reader structure
impl crate::Readable for DHTMEM15rs {}
///`write(|w| ..)` method takes [`dhtmem15::W`](W) writer structure
impl crate::Writable for DHTMEM15rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM15 to value 0
impl crate::Resettable for DHTMEM15rs {}
