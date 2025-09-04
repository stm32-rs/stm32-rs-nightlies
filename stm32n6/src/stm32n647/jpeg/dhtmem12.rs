///Register `DHTMEM12` reader
pub type R = crate::R<DHTMEM12rs>;
///Register `DHTMEM12` writer
pub type W = crate::W<DHTMEM12rs>;
///Field `DATA48` reader - Huffman table data 48
pub type DATA48_R = crate::FieldReader;
///Field `DATA48` writer - Huffman table data 48
pub type DATA48_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA49` reader - Huffman table data 49
pub type DATA49_R = crate::FieldReader;
///Field `DATA49` writer - Huffman table data 49
pub type DATA49_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA50` reader - Huffman table data 50
pub type DATA50_R = crate::FieldReader;
///Field `DATA50` writer - Huffman table data 50
pub type DATA50_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA51` reader - Huffman table data 51
pub type DATA51_R = crate::FieldReader;
///Field `DATA51` writer - Huffman table data 51
pub type DATA51_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 48
    #[inline(always)]
    pub fn data48(&self) -> DATA48_R {
        DATA48_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 49
    #[inline(always)]
    pub fn data49(&self) -> DATA49_R {
        DATA49_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 50
    #[inline(always)]
    pub fn data50(&self) -> DATA50_R {
        DATA50_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 51
    #[inline(always)]
    pub fn data51(&self) -> DATA51_R {
        DATA51_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM12")
            .field("data48", &self.data48())
            .field("data49", &self.data49())
            .field("data50", &self.data50())
            .field("data51", &self.data51())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 48
    #[inline(always)]
    pub fn data48(&mut self) -> DATA48_W<DHTMEM12rs> {
        DATA48_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 49
    #[inline(always)]
    pub fn data49(&mut self) -> DATA49_W<DHTMEM12rs> {
        DATA49_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 50
    #[inline(always)]
    pub fn data50(&mut self) -> DATA50_W<DHTMEM12rs> {
        DATA50_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 51
    #[inline(always)]
    pub fn data51(&mut self) -> DATA51_W<DHTMEM12rs> {
        DATA51_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM12)*/
pub struct DHTMEM12rs;
impl crate::RegisterSpec for DHTMEM12rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem12::R`](R) reader structure
impl crate::Readable for DHTMEM12rs {}
///`write(|w| ..)` method takes [`dhtmem12::W`](W) writer structure
impl crate::Writable for DHTMEM12rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM12 to value 0
impl crate::Resettable for DHTMEM12rs {}
