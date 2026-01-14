///Register `HUFFENC_AC1_54` reader
pub type R = crate::R<HUFFENC_AC1_54rs>;
///Register `HUFFENC_AC1_54` writer
pub type W = crate::W<HUFFENC_AC1_54rs>;
///Field `HCODE108` reader - Huffman code 108
pub type HCODE108_R = crate::FieldReader;
///Field `HCODE108` writer - Huffman code 108
pub type HCODE108_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN108` reader - Huffman length 108
pub type HLEN108_R = crate::FieldReader;
///Field `HLEN108` writer - Huffman length 108
pub type HLEN108_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE109` reader - Huffman code 109
pub type HCODE109_R = crate::FieldReader;
///Field `HCODE109` writer - Huffman code 109
pub type HCODE109_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN109` reader - Huffman length 109
pub type HLEN109_R = crate::FieldReader;
///Field `HLEN109` writer - Huffman length 109
pub type HLEN109_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 108
    #[inline(always)]
    pub fn hcode108(&self) -> HCODE108_R {
        HCODE108_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 108
    #[inline(always)]
    pub fn hlen108(&self) -> HLEN108_R {
        HLEN108_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 109
    #[inline(always)]
    pub fn hcode109(&self) -> HCODE109_R {
        HCODE109_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 109
    #[inline(always)]
    pub fn hlen109(&self) -> HLEN109_R {
        HLEN109_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_54")
            .field("hcode108", &self.hcode108())
            .field("hlen108", &self.hlen108())
            .field("hcode109", &self.hcode109())
            .field("hlen109", &self.hlen109())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 108
    #[inline(always)]
    pub fn hcode108(&mut self) -> HCODE108_W<'_, HUFFENC_AC1_54rs> {
        HCODE108_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 108
    #[inline(always)]
    pub fn hlen108(&mut self) -> HLEN108_W<'_, HUFFENC_AC1_54rs> {
        HLEN108_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 109
    #[inline(always)]
    pub fn hcode109(&mut self) -> HCODE109_W<'_, HUFFENC_AC1_54rs> {
        HCODE109_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 109
    #[inline(always)]
    pub fn hlen109(&mut self) -> HLEN109_W<'_, HUFFENC_AC1_54rs> {
        HLEN109_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_54)*/
pub struct HUFFENC_AC1_54rs;
impl crate::RegisterSpec for HUFFENC_AC1_54rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_54::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_54rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_54::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_54rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_54 to value 0
impl crate::Resettable for HUFFENC_AC1_54rs {}
