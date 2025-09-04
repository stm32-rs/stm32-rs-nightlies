///Register `DHTMEM21` reader
pub type R = crate::R<DHTMEM21rs>;
///Register `DHTMEM21` writer
pub type W = crate::W<DHTMEM21rs>;
///Field `DATA84` reader - Huffman table data 84
pub type DATA84_R = crate::FieldReader;
///Field `DATA84` writer - Huffman table data 84
pub type DATA84_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA85` reader - Huffman table data 85
pub type DATA85_R = crate::FieldReader;
///Field `DATA85` writer - Huffman table data 85
pub type DATA85_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA86` reader - Huffman table data 86
pub type DATA86_R = crate::FieldReader;
///Field `DATA86` writer - Huffman table data 86
pub type DATA86_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA87` reader - Huffman table data 87
pub type DATA87_R = crate::FieldReader;
///Field `DATA87` writer - Huffman table data 87
pub type DATA87_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 84
    #[inline(always)]
    pub fn data84(&self) -> DATA84_R {
        DATA84_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 85
    #[inline(always)]
    pub fn data85(&self) -> DATA85_R {
        DATA85_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 86
    #[inline(always)]
    pub fn data86(&self) -> DATA86_R {
        DATA86_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 87
    #[inline(always)]
    pub fn data87(&self) -> DATA87_R {
        DATA87_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM21")
            .field("data84", &self.data84())
            .field("data85", &self.data85())
            .field("data86", &self.data86())
            .field("data87", &self.data87())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 84
    #[inline(always)]
    pub fn data84(&mut self) -> DATA84_W<DHTMEM21rs> {
        DATA84_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 85
    #[inline(always)]
    pub fn data85(&mut self) -> DATA85_W<DHTMEM21rs> {
        DATA85_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 86
    #[inline(always)]
    pub fn data86(&mut self) -> DATA86_W<DHTMEM21rs> {
        DATA86_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 87
    #[inline(always)]
    pub fn data87(&mut self) -> DATA87_W<DHTMEM21rs> {
        DATA87_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM21)*/
pub struct DHTMEM21rs;
impl crate::RegisterSpec for DHTMEM21rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem21::R`](R) reader structure
impl crate::Readable for DHTMEM21rs {}
///`write(|w| ..)` method takes [`dhtmem21::W`](W) writer structure
impl crate::Writable for DHTMEM21rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM21 to value 0
impl crate::Resettable for DHTMEM21rs {}
