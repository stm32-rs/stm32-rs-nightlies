///Register `DHTMEM17` reader
pub type R = crate::R<DHTMEM17rs>;
///Register `DHTMEM17` writer
pub type W = crate::W<DHTMEM17rs>;
///Field `DATA68` reader - Huffman table data 68
pub type DATA68_R = crate::FieldReader;
///Field `DATA68` writer - Huffman table data 68
pub type DATA68_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA69` reader - Huffman table data 69
pub type DATA69_R = crate::FieldReader;
///Field `DATA69` writer - Huffman table data 69
pub type DATA69_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA70` reader - Huffman table data 70
pub type DATA70_R = crate::FieldReader;
///Field `DATA70` writer - Huffman table data 70
pub type DATA70_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA71` reader - Huffman table data 71
pub type DATA71_R = crate::FieldReader;
///Field `DATA71` writer - Huffman table data 71
pub type DATA71_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 68
    #[inline(always)]
    pub fn data68(&self) -> DATA68_R {
        DATA68_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 69
    #[inline(always)]
    pub fn data69(&self) -> DATA69_R {
        DATA69_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 70
    #[inline(always)]
    pub fn data70(&self) -> DATA70_R {
        DATA70_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 71
    #[inline(always)]
    pub fn data71(&self) -> DATA71_R {
        DATA71_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM17")
            .field("data68", &self.data68())
            .field("data69", &self.data69())
            .field("data70", &self.data70())
            .field("data71", &self.data71())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 68
    #[inline(always)]
    pub fn data68(&mut self) -> DATA68_W<'_, DHTMEM17rs> {
        DATA68_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 69
    #[inline(always)]
    pub fn data69(&mut self) -> DATA69_W<'_, DHTMEM17rs> {
        DATA69_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 70
    #[inline(always)]
    pub fn data70(&mut self) -> DATA70_W<'_, DHTMEM17rs> {
        DATA70_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 71
    #[inline(always)]
    pub fn data71(&mut self) -> DATA71_W<'_, DHTMEM17rs> {
        DATA71_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM17)*/
pub struct DHTMEM17rs;
impl crate::RegisterSpec for DHTMEM17rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem17::R`](R) reader structure
impl crate::Readable for DHTMEM17rs {}
///`write(|w| ..)` method takes [`dhtmem17::W`](W) writer structure
impl crate::Writable for DHTMEM17rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM17 to value 0
impl crate::Resettable for DHTMEM17rs {}
