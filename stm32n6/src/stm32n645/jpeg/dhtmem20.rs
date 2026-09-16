///Register `DHTMEM20` reader
pub type R = crate::R<DHTMEM20rs>;
///Register `DHTMEM20` writer
pub type W = crate::W<DHTMEM20rs>;
///Field `DATA80` reader - Huffman table data 80
pub type DATA80_R = crate::FieldReader;
///Field `DATA80` writer - Huffman table data 80
pub type DATA80_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA81` reader - Huffman table data 81
pub type DATA81_R = crate::FieldReader;
///Field `DATA81` writer - Huffman table data 81
pub type DATA81_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA82` reader - Huffman table data 82
pub type DATA82_R = crate::FieldReader;
///Field `DATA82` writer - Huffman table data 82
pub type DATA82_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA83` reader - Huffman table data 83
pub type DATA83_R = crate::FieldReader;
///Field `DATA83` writer - Huffman table data 83
pub type DATA83_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 80
    #[inline(always)]
    pub fn data80(&self) -> DATA80_R {
        DATA80_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 81
    #[inline(always)]
    pub fn data81(&self) -> DATA81_R {
        DATA81_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 82
    #[inline(always)]
    pub fn data82(&self) -> DATA82_R {
        DATA82_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 83
    #[inline(always)]
    pub fn data83(&self) -> DATA83_R {
        DATA83_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM20")
            .field("data80", &self.data80())
            .field("data81", &self.data81())
            .field("data82", &self.data82())
            .field("data83", &self.data83())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 80
    #[inline(always)]
    pub fn data80(&mut self) -> DATA80_W<'_, DHTMEM20rs> {
        DATA80_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 81
    #[inline(always)]
    pub fn data81(&mut self) -> DATA81_W<'_, DHTMEM20rs> {
        DATA81_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 82
    #[inline(always)]
    pub fn data82(&mut self) -> DATA82_W<'_, DHTMEM20rs> {
        DATA82_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 83
    #[inline(always)]
    pub fn data83(&mut self) -> DATA83_W<'_, DHTMEM20rs> {
        DATA83_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM20)*/
pub struct DHTMEM20rs;
impl crate::RegisterSpec for DHTMEM20rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem20::R`](R) reader structure
impl crate::Readable for DHTMEM20rs {}
///`write(|w| ..)` method takes [`dhtmem20::W`](W) writer structure
impl crate::Writable for DHTMEM20rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM20 to value 0
impl crate::Resettable for DHTMEM20rs {}
