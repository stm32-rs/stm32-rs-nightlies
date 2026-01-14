///Register `HUFFENC_AC1_28` reader
pub type R = crate::R<HUFFENC_AC1_28rs>;
///Register `HUFFENC_AC1_28` writer
pub type W = crate::W<HUFFENC_AC1_28rs>;
///Field `HCODE56` reader - Huffman code 56
pub type HCODE56_R = crate::FieldReader;
///Field `HCODE56` writer - Huffman code 56
pub type HCODE56_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN56` reader - Huffman length 56
pub type HLEN56_R = crate::FieldReader;
///Field `HLEN56` writer - Huffman length 56
pub type HLEN56_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE57` reader - Huffman code 57
pub type HCODE57_R = crate::FieldReader;
///Field `HCODE57` writer - Huffman code 57
pub type HCODE57_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN57` reader - Huffman length 57
pub type HLEN57_R = crate::FieldReader;
///Field `HLEN57` writer - Huffman length 57
pub type HLEN57_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 56
    #[inline(always)]
    pub fn hcode56(&self) -> HCODE56_R {
        HCODE56_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 56
    #[inline(always)]
    pub fn hlen56(&self) -> HLEN56_R {
        HLEN56_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 57
    #[inline(always)]
    pub fn hcode57(&self) -> HCODE57_R {
        HCODE57_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 57
    #[inline(always)]
    pub fn hlen57(&self) -> HLEN57_R {
        HLEN57_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_28")
            .field("hcode56", &self.hcode56())
            .field("hlen56", &self.hlen56())
            .field("hcode57", &self.hcode57())
            .field("hlen57", &self.hlen57())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 56
    #[inline(always)]
    pub fn hcode56(&mut self) -> HCODE56_W<'_, HUFFENC_AC1_28rs> {
        HCODE56_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 56
    #[inline(always)]
    pub fn hlen56(&mut self) -> HLEN56_W<'_, HUFFENC_AC1_28rs> {
        HLEN56_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 57
    #[inline(always)]
    pub fn hcode57(&mut self) -> HCODE57_W<'_, HUFFENC_AC1_28rs> {
        HCODE57_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 57
    #[inline(always)]
    pub fn hlen57(&mut self) -> HLEN57_W<'_, HUFFENC_AC1_28rs> {
        HLEN57_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC1_28)*/
pub struct HUFFENC_AC1_28rs;
impl crate::RegisterSpec for HUFFENC_AC1_28rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_28::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_28rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_28::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_28rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_28 to value 0
impl crate::Resettable for HUFFENC_AC1_28rs {}
