///Register `DHTMEM40` reader
pub type R = crate::R<DHTMEM40rs>;
///Register `DHTMEM40` writer
pub type W = crate::W<DHTMEM40rs>;
///Field `DATA160` reader - Huffman table data 160
pub type DATA160_R = crate::FieldReader;
///Field `DATA160` writer - Huffman table data 160
pub type DATA160_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA161` reader - Huffman table data 161
pub type DATA161_R = crate::FieldReader;
///Field `DATA161` writer - Huffman table data 161
pub type DATA161_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA162` reader - Huffman table data 162
pub type DATA162_R = crate::FieldReader;
///Field `DATA162` writer - Huffman table data 162
pub type DATA162_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA163` reader - Huffman table data 163
pub type DATA163_R = crate::FieldReader;
///Field `DATA163` writer - Huffman table data 163
pub type DATA163_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 160
    #[inline(always)]
    pub fn data160(&self) -> DATA160_R {
        DATA160_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 161
    #[inline(always)]
    pub fn data161(&self) -> DATA161_R {
        DATA161_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 162
    #[inline(always)]
    pub fn data162(&self) -> DATA162_R {
        DATA162_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 163
    #[inline(always)]
    pub fn data163(&self) -> DATA163_R {
        DATA163_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM40")
            .field("data160", &self.data160())
            .field("data161", &self.data161())
            .field("data162", &self.data162())
            .field("data163", &self.data163())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 160
    #[inline(always)]
    pub fn data160(&mut self) -> DATA160_W<DHTMEM40rs> {
        DATA160_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 161
    #[inline(always)]
    pub fn data161(&mut self) -> DATA161_W<DHTMEM40rs> {
        DATA161_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 162
    #[inline(always)]
    pub fn data162(&mut self) -> DATA162_W<DHTMEM40rs> {
        DATA162_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 163
    #[inline(always)]
    pub fn data163(&mut self) -> DATA163_W<DHTMEM40rs> {
        DATA163_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM40)*/
pub struct DHTMEM40rs;
impl crate::RegisterSpec for DHTMEM40rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem40::R`](R) reader structure
impl crate::Readable for DHTMEM40rs {}
///`write(|w| ..)` method takes [`dhtmem40::W`](W) writer structure
impl crate::Writable for DHTMEM40rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM40 to value 0
impl crate::Resettable for DHTMEM40rs {}
