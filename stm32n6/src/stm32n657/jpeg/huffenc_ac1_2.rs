///Register `HUFFENC_AC1_2` reader
pub type R = crate::R<HUFFENC_AC1_2rs>;
///Register `HUFFENC_AC1_2` writer
pub type W = crate::W<HUFFENC_AC1_2rs>;
///Field `HCODE4` reader - Huffman code 4
pub type HCODE4_R = crate::FieldReader;
///Field `HCODE4` writer - Huffman code 4
pub type HCODE4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN4` reader - Huffman length 4
pub type HLEN4_R = crate::FieldReader;
///Field `HLEN4` writer - Huffman length 4
pub type HLEN4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE5` reader - Huffman code 5
pub type HCODE5_R = crate::FieldReader;
///Field `HCODE5` writer - Huffman code 5
pub type HCODE5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN5` reader - Huffman length 5
pub type HLEN5_R = crate::FieldReader;
///Field `HLEN5` writer - Huffman length 5
pub type HLEN5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 4
    #[inline(always)]
    pub fn hcode4(&self) -> HCODE4_R {
        HCODE4_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 4
    #[inline(always)]
    pub fn hlen4(&self) -> HLEN4_R {
        HLEN4_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 5
    #[inline(always)]
    pub fn hcode5(&self) -> HCODE5_R {
        HCODE5_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 5
    #[inline(always)]
    pub fn hlen5(&self) -> HLEN5_R {
        HLEN5_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_2")
            .field("hcode4", &self.hcode4())
            .field("hlen4", &self.hlen4())
            .field("hcode5", &self.hcode5())
            .field("hlen5", &self.hlen5())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 4
    #[inline(always)]
    pub fn hcode4(&mut self) -> HCODE4_W<'_, HUFFENC_AC1_2rs> {
        HCODE4_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 4
    #[inline(always)]
    pub fn hlen4(&mut self) -> HLEN4_W<'_, HUFFENC_AC1_2rs> {
        HLEN4_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 5
    #[inline(always)]
    pub fn hcode5(&mut self) -> HCODE5_W<'_, HUFFENC_AC1_2rs> {
        HCODE5_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 5
    #[inline(always)]
    pub fn hlen5(&mut self) -> HLEN5_W<'_, HUFFENC_AC1_2rs> {
        HLEN5_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC1_2)*/
pub struct HUFFENC_AC1_2rs;
impl crate::RegisterSpec for HUFFENC_AC1_2rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_2::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_2rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_2::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_2 to value 0
impl crate::Resettable for HUFFENC_AC1_2rs {}
