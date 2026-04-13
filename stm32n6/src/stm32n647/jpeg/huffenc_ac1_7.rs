///Register `HUFFENC_AC1_7` reader
pub type R = crate::R<HUFFENC_AC1_7rs>;
///Register `HUFFENC_AC1_7` writer
pub type W = crate::W<HUFFENC_AC1_7rs>;
///Field `HCODE14` reader - Huffman code 14
pub type HCODE14_R = crate::FieldReader;
///Field `HCODE14` writer - Huffman code 14
pub type HCODE14_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN14` reader - Huffman length 14
pub type HLEN14_R = crate::FieldReader;
///Field `HLEN14` writer - Huffman length 14
pub type HLEN14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE15` reader - Huffman code 15
pub type HCODE15_R = crate::FieldReader;
///Field `HCODE15` writer - Huffman code 15
pub type HCODE15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN15` reader - Huffman length 15
pub type HLEN15_R = crate::FieldReader;
///Field `HLEN15` writer - Huffman length 15
pub type HLEN15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 14
    #[inline(always)]
    pub fn hcode14(&self) -> HCODE14_R {
        HCODE14_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 14
    #[inline(always)]
    pub fn hlen14(&self) -> HLEN14_R {
        HLEN14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 15
    #[inline(always)]
    pub fn hcode15(&self) -> HCODE15_R {
        HCODE15_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 15
    #[inline(always)]
    pub fn hlen15(&self) -> HLEN15_R {
        HLEN15_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_7")
            .field("hcode14", &self.hcode14())
            .field("hlen14", &self.hlen14())
            .field("hcode15", &self.hcode15())
            .field("hlen15", &self.hlen15())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 14
    #[inline(always)]
    pub fn hcode14(&mut self) -> HCODE14_W<'_, HUFFENC_AC1_7rs> {
        HCODE14_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 14
    #[inline(always)]
    pub fn hlen14(&mut self) -> HLEN14_W<'_, HUFFENC_AC1_7rs> {
        HLEN14_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 15
    #[inline(always)]
    pub fn hcode15(&mut self) -> HCODE15_W<'_, HUFFENC_AC1_7rs> {
        HCODE15_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 15
    #[inline(always)]
    pub fn hlen15(&mut self) -> HLEN15_W<'_, HUFFENC_AC1_7rs> {
        HLEN15_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC1_7)*/
pub struct HUFFENC_AC1_7rs;
impl crate::RegisterSpec for HUFFENC_AC1_7rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_7::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_7rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_7::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_7 to value 0
impl crate::Resettable for HUFFENC_AC1_7rs {}
