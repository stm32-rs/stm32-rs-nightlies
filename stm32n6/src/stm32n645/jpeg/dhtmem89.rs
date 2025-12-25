///Register `DHTMEM89` reader
pub type R = crate::R<DHTMEM89rs>;
///Register `DHTMEM89` writer
pub type W = crate::W<DHTMEM89rs>;
///Field `DATA356` reader - Huffman table data 356
pub type DATA356_R = crate::FieldReader;
///Field `DATA356` writer - Huffman table data 356
pub type DATA356_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA357` reader - Huffman table data 357
pub type DATA357_R = crate::FieldReader;
///Field `DATA357` writer - Huffman table data 357
pub type DATA357_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA358` reader - Huffman table data 358
pub type DATA358_R = crate::FieldReader;
///Field `DATA358` writer - Huffman table data 358
pub type DATA358_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA359` reader - Huffman table data 359
pub type DATA359_R = crate::FieldReader;
///Field `DATA359` writer - Huffman table data 359
pub type DATA359_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 356
    #[inline(always)]
    pub fn data356(&self) -> DATA356_R {
        DATA356_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 357
    #[inline(always)]
    pub fn data357(&self) -> DATA357_R {
        DATA357_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 358
    #[inline(always)]
    pub fn data358(&self) -> DATA358_R {
        DATA358_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 359
    #[inline(always)]
    pub fn data359(&self) -> DATA359_R {
        DATA359_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM89")
            .field("data356", &self.data356())
            .field("data357", &self.data357())
            .field("data358", &self.data358())
            .field("data359", &self.data359())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 356
    #[inline(always)]
    pub fn data356(&mut self) -> DATA356_W<'_, DHTMEM89rs> {
        DATA356_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 357
    #[inline(always)]
    pub fn data357(&mut self) -> DATA357_W<'_, DHTMEM89rs> {
        DATA357_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 358
    #[inline(always)]
    pub fn data358(&mut self) -> DATA358_W<'_, DHTMEM89rs> {
        DATA358_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 359
    #[inline(always)]
    pub fn data359(&mut self) -> DATA359_W<'_, DHTMEM89rs> {
        DATA359_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem89::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem89::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM89)*/
pub struct DHTMEM89rs;
impl crate::RegisterSpec for DHTMEM89rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem89::R`](R) reader structure
impl crate::Readable for DHTMEM89rs {}
///`write(|w| ..)` method takes [`dhtmem89::W`](W) writer structure
impl crate::Writable for DHTMEM89rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM89 to value 0
impl crate::Resettable for DHTMEM89rs {}
