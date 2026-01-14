///Register `DHTMEM42` reader
pub type R = crate::R<DHTMEM42rs>;
///Register `DHTMEM42` writer
pub type W = crate::W<DHTMEM42rs>;
///Field `DATA168` reader - Huffman table data 168
pub type DATA168_R = crate::FieldReader;
///Field `DATA168` writer - Huffman table data 168
pub type DATA168_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA169` reader - Huffman table data 169
pub type DATA169_R = crate::FieldReader;
///Field `DATA169` writer - Huffman table data 169
pub type DATA169_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA170` reader - Huffman table data 170
pub type DATA170_R = crate::FieldReader;
///Field `DATA170` writer - Huffman table data 170
pub type DATA170_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA171` reader - Huffman table data 171
pub type DATA171_R = crate::FieldReader;
///Field `DATA171` writer - Huffman table data 171
pub type DATA171_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 168
    #[inline(always)]
    pub fn data168(&self) -> DATA168_R {
        DATA168_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 169
    #[inline(always)]
    pub fn data169(&self) -> DATA169_R {
        DATA169_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 170
    #[inline(always)]
    pub fn data170(&self) -> DATA170_R {
        DATA170_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 171
    #[inline(always)]
    pub fn data171(&self) -> DATA171_R {
        DATA171_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM42")
            .field("data168", &self.data168())
            .field("data169", &self.data169())
            .field("data170", &self.data170())
            .field("data171", &self.data171())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 168
    #[inline(always)]
    pub fn data168(&mut self) -> DATA168_W<'_, DHTMEM42rs> {
        DATA168_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 169
    #[inline(always)]
    pub fn data169(&mut self) -> DATA169_W<'_, DHTMEM42rs> {
        DATA169_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 170
    #[inline(always)]
    pub fn data170(&mut self) -> DATA170_W<'_, DHTMEM42rs> {
        DATA170_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 171
    #[inline(always)]
    pub fn data171(&mut self) -> DATA171_W<'_, DHTMEM42rs> {
        DATA171_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem42::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem42::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM42)*/
pub struct DHTMEM42rs;
impl crate::RegisterSpec for DHTMEM42rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem42::R`](R) reader structure
impl crate::Readable for DHTMEM42rs {}
///`write(|w| ..)` method takes [`dhtmem42::W`](W) writer structure
impl crate::Writable for DHTMEM42rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM42 to value 0
impl crate::Resettable for DHTMEM42rs {}
