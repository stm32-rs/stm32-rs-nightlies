///Register `HUFFENC_AC1_44` reader
pub type R = crate::R<HUFFENC_AC1_44rs>;
///Register `HUFFENC_AC1_44` writer
pub type W = crate::W<HUFFENC_AC1_44rs>;
///Field `HCODE88` reader - Huffman code 88
pub type HCODE88_R = crate::FieldReader;
///Field `HCODE88` writer - Huffman code 88
pub type HCODE88_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN88` reader - Huffman length 88
pub type HLEN88_R = crate::FieldReader;
///Field `HLEN88` writer - Huffman length 88
pub type HLEN88_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE89` reader - Huffman code 89
pub type HCODE89_R = crate::FieldReader;
///Field `HCODE89` writer - Huffman code 89
pub type HCODE89_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN89` reader - Huffman length 89
pub type HLEN89_R = crate::FieldReader;
///Field `HLEN89` writer - Huffman length 89
pub type HLEN89_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 88
    #[inline(always)]
    pub fn hcode88(&self) -> HCODE88_R {
        HCODE88_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 88
    #[inline(always)]
    pub fn hlen88(&self) -> HLEN88_R {
        HLEN88_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 89
    #[inline(always)]
    pub fn hcode89(&self) -> HCODE89_R {
        HCODE89_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 89
    #[inline(always)]
    pub fn hlen89(&self) -> HLEN89_R {
        HLEN89_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_44")
            .field("hcode88", &self.hcode88())
            .field("hlen88", &self.hlen88())
            .field("hcode89", &self.hcode89())
            .field("hlen89", &self.hlen89())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 88
    #[inline(always)]
    pub fn hcode88(&mut self) -> HCODE88_W<'_, HUFFENC_AC1_44rs> {
        HCODE88_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 88
    #[inline(always)]
    pub fn hlen88(&mut self) -> HLEN88_W<'_, HUFFENC_AC1_44rs> {
        HLEN88_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 89
    #[inline(always)]
    pub fn hcode89(&mut self) -> HCODE89_W<'_, HUFFENC_AC1_44rs> {
        HCODE89_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 89
    #[inline(always)]
    pub fn hlen89(&mut self) -> HLEN89_W<'_, HUFFENC_AC1_44rs> {
        HLEN89_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_44)*/
pub struct HUFFENC_AC1_44rs;
impl crate::RegisterSpec for HUFFENC_AC1_44rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_44::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_44rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_44::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_44rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_44 to value 0
impl crate::Resettable for HUFFENC_AC1_44rs {}
