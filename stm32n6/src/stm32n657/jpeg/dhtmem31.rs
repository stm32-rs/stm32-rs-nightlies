///Register `DHTMEM31` reader
pub type R = crate::R<DHTMEM31rs>;
///Register `DHTMEM31` writer
pub type W = crate::W<DHTMEM31rs>;
///Field `DATA124` reader - Huffman table data 124
pub type DATA124_R = crate::FieldReader;
///Field `DATA124` writer - Huffman table data 124
pub type DATA124_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA125` reader - Huffman table data 125
pub type DATA125_R = crate::FieldReader;
///Field `DATA125` writer - Huffman table data 125
pub type DATA125_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA126` reader - Huffman table data 126
pub type DATA126_R = crate::FieldReader;
///Field `DATA126` writer - Huffman table data 126
pub type DATA126_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA127` reader - Huffman table data 127
pub type DATA127_R = crate::FieldReader;
///Field `DATA127` writer - Huffman table data 127
pub type DATA127_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 124
    #[inline(always)]
    pub fn data124(&self) -> DATA124_R {
        DATA124_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 125
    #[inline(always)]
    pub fn data125(&self) -> DATA125_R {
        DATA125_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 126
    #[inline(always)]
    pub fn data126(&self) -> DATA126_R {
        DATA126_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 127
    #[inline(always)]
    pub fn data127(&self) -> DATA127_R {
        DATA127_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM31")
            .field("data124", &self.data124())
            .field("data125", &self.data125())
            .field("data126", &self.data126())
            .field("data127", &self.data127())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 124
    #[inline(always)]
    pub fn data124(&mut self) -> DATA124_W<'_, DHTMEM31rs> {
        DATA124_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 125
    #[inline(always)]
    pub fn data125(&mut self) -> DATA125_W<'_, DHTMEM31rs> {
        DATA125_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 126
    #[inline(always)]
    pub fn data126(&mut self) -> DATA126_W<'_, DHTMEM31rs> {
        DATA126_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 127
    #[inline(always)]
    pub fn data127(&mut self) -> DATA127_W<'_, DHTMEM31rs> {
        DATA127_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM31)*/
pub struct DHTMEM31rs;
impl crate::RegisterSpec for DHTMEM31rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem31::R`](R) reader structure
impl crate::Readable for DHTMEM31rs {}
///`write(|w| ..)` method takes [`dhtmem31::W`](W) writer structure
impl crate::Writable for DHTMEM31rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM31 to value 0
impl crate::Resettable for DHTMEM31rs {}
