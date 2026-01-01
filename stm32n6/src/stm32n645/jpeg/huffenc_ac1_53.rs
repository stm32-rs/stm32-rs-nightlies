///Register `HUFFENC_AC1_53` reader
pub type R = crate::R<HUFFENC_AC1_53rs>;
///Register `HUFFENC_AC1_53` writer
pub type W = crate::W<HUFFENC_AC1_53rs>;
///Field `HCODE106` reader - Huffman code 106
pub type HCODE106_R = crate::FieldReader;
///Field `HCODE106` writer - Huffman code 106
pub type HCODE106_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN106` reader - Huffman length 106
pub type HLEN106_R = crate::FieldReader;
///Field `HLEN106` writer - Huffman length 106
pub type HLEN106_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE107` reader - Huffman code 107
pub type HCODE107_R = crate::FieldReader;
///Field `HCODE107` writer - Huffman code 107
pub type HCODE107_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN107` reader - Huffman length 107
pub type HLEN107_R = crate::FieldReader;
///Field `HLEN107` writer - Huffman length 107
pub type HLEN107_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 106
    #[inline(always)]
    pub fn hcode106(&self) -> HCODE106_R {
        HCODE106_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 106
    #[inline(always)]
    pub fn hlen106(&self) -> HLEN106_R {
        HLEN106_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 107
    #[inline(always)]
    pub fn hcode107(&self) -> HCODE107_R {
        HCODE107_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 107
    #[inline(always)]
    pub fn hlen107(&self) -> HLEN107_R {
        HLEN107_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_53")
            .field("hcode106", &self.hcode106())
            .field("hlen106", &self.hlen106())
            .field("hcode107", &self.hcode107())
            .field("hlen107", &self.hlen107())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 106
    #[inline(always)]
    pub fn hcode106(&mut self) -> HCODE106_W<'_, HUFFENC_AC1_53rs> {
        HCODE106_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 106
    #[inline(always)]
    pub fn hlen106(&mut self) -> HLEN106_W<'_, HUFFENC_AC1_53rs> {
        HLEN106_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 107
    #[inline(always)]
    pub fn hcode107(&mut self) -> HCODE107_W<'_, HUFFENC_AC1_53rs> {
        HCODE107_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 107
    #[inline(always)]
    pub fn hlen107(&mut self) -> HLEN107_W<'_, HUFFENC_AC1_53rs> {
        HLEN107_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_53::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_53::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC1_53)*/
pub struct HUFFENC_AC1_53rs;
impl crate::RegisterSpec for HUFFENC_AC1_53rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_53::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_53rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_53::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_53rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_53 to value 0
impl crate::Resettable for HUFFENC_AC1_53rs {}
