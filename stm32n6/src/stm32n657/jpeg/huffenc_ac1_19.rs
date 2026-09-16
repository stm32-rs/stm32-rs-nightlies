///Register `HUFFENC_AC1_19` reader
pub type R = crate::R<HUFFENC_AC1_19rs>;
///Register `HUFFENC_AC1_19` writer
pub type W = crate::W<HUFFENC_AC1_19rs>;
///Field `HCODE38` reader - Huffman code 38
pub type HCODE38_R = crate::FieldReader;
///Field `HCODE38` writer - Huffman code 38
pub type HCODE38_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN38` reader - Huffman length 38
pub type HLEN38_R = crate::FieldReader;
///Field `HLEN38` writer - Huffman length 38
pub type HLEN38_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE39` reader - Huffman code 39
pub type HCODE39_R = crate::FieldReader;
///Field `HCODE39` writer - Huffman code 39
pub type HCODE39_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN39` reader - Huffman length 39
pub type HLEN39_R = crate::FieldReader;
///Field `HLEN39` writer - Huffman length 39
pub type HLEN39_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 38
    #[inline(always)]
    pub fn hcode38(&self) -> HCODE38_R {
        HCODE38_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 38
    #[inline(always)]
    pub fn hlen38(&self) -> HLEN38_R {
        HLEN38_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 39
    #[inline(always)]
    pub fn hcode39(&self) -> HCODE39_R {
        HCODE39_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 39
    #[inline(always)]
    pub fn hlen39(&self) -> HLEN39_R {
        HLEN39_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_19")
            .field("hcode38", &self.hcode38())
            .field("hlen38", &self.hlen38())
            .field("hcode39", &self.hcode39())
            .field("hlen39", &self.hlen39())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 38
    #[inline(always)]
    pub fn hcode38(&mut self) -> HCODE38_W<'_, HUFFENC_AC1_19rs> {
        HCODE38_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 38
    #[inline(always)]
    pub fn hlen38(&mut self) -> HLEN38_W<'_, HUFFENC_AC1_19rs> {
        HLEN38_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 39
    #[inline(always)]
    pub fn hcode39(&mut self) -> HCODE39_W<'_, HUFFENC_AC1_19rs> {
        HCODE39_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 39
    #[inline(always)]
    pub fn hlen39(&mut self) -> HLEN39_W<'_, HUFFENC_AC1_19rs> {
        HLEN39_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC1_19)*/
pub struct HUFFENC_AC1_19rs;
impl crate::RegisterSpec for HUFFENC_AC1_19rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_19::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_19rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_19::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_19rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_19 to value 0
impl crate::Resettable for HUFFENC_AC1_19rs {}
