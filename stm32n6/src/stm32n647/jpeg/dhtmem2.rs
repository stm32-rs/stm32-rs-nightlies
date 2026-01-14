///Register `DHTMEM2` reader
pub type R = crate::R<DHTMEM2rs>;
///Register `DHTMEM2` writer
pub type W = crate::W<DHTMEM2rs>;
///Field `DATA8` reader - Huffman table data 8
pub type DATA8_R = crate::FieldReader;
///Field `DATA8` writer - Huffman table data 8
pub type DATA8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA9` reader - Huffman table data 9
pub type DATA9_R = crate::FieldReader;
///Field `DATA9` writer - Huffman table data 9
pub type DATA9_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA10` reader - Huffman table data 10
pub type DATA10_R = crate::FieldReader;
///Field `DATA10` writer - Huffman table data 10
pub type DATA10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA11` reader - Huffman table data 11
pub type DATA11_R = crate::FieldReader;
///Field `DATA11` writer - Huffman table data 11
pub type DATA11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 8
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 9
    #[inline(always)]
    pub fn data9(&self) -> DATA9_R {
        DATA9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 10
    #[inline(always)]
    pub fn data10(&self) -> DATA10_R {
        DATA10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 11
    #[inline(always)]
    pub fn data11(&self) -> DATA11_R {
        DATA11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM2")
            .field("data8", &self.data8())
            .field("data9", &self.data9())
            .field("data10", &self.data10())
            .field("data11", &self.data11())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 8
    #[inline(always)]
    pub fn data8(&mut self) -> DATA8_W<'_, DHTMEM2rs> {
        DATA8_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 9
    #[inline(always)]
    pub fn data9(&mut self) -> DATA9_W<'_, DHTMEM2rs> {
        DATA9_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 10
    #[inline(always)]
    pub fn data10(&mut self) -> DATA10_W<'_, DHTMEM2rs> {
        DATA10_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 11
    #[inline(always)]
    pub fn data11(&mut self) -> DATA11_W<'_, DHTMEM2rs> {
        DATA11_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM2)*/
pub struct DHTMEM2rs;
impl crate::RegisterSpec for DHTMEM2rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem2::R`](R) reader structure
impl crate::Readable for DHTMEM2rs {}
///`write(|w| ..)` method takes [`dhtmem2::W`](W) writer structure
impl crate::Writable for DHTMEM2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM2 to value 0
impl crate::Resettable for DHTMEM2rs {}
