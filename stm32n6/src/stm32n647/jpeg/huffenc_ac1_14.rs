///Register `HUFFENC_AC1_14` reader
pub type R = crate::R<HUFFENC_AC1_14rs>;
///Register `HUFFENC_AC1_14` writer
pub type W = crate::W<HUFFENC_AC1_14rs>;
///Field `HCODE28` reader - Huffman code 28
pub type HCODE28_R = crate::FieldReader;
///Field `HCODE28` writer - Huffman code 28
pub type HCODE28_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN28` reader - Huffman length 28
pub type HLEN28_R = crate::FieldReader;
///Field `HLEN28` writer - Huffman length 28
pub type HLEN28_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE29` reader - Huffman code 29
pub type HCODE29_R = crate::FieldReader;
///Field `HCODE29` writer - Huffman code 29
pub type HCODE29_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN29` reader - Huffman length 29
pub type HLEN29_R = crate::FieldReader;
///Field `HLEN29` writer - Huffman length 29
pub type HLEN29_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 28
    #[inline(always)]
    pub fn hcode28(&self) -> HCODE28_R {
        HCODE28_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 28
    #[inline(always)]
    pub fn hlen28(&self) -> HLEN28_R {
        HLEN28_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 29
    #[inline(always)]
    pub fn hcode29(&self) -> HCODE29_R {
        HCODE29_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 29
    #[inline(always)]
    pub fn hlen29(&self) -> HLEN29_R {
        HLEN29_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_14")
            .field("hcode28", &self.hcode28())
            .field("hlen28", &self.hlen28())
            .field("hcode29", &self.hcode29())
            .field("hlen29", &self.hlen29())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 28
    #[inline(always)]
    pub fn hcode28(&mut self) -> HCODE28_W<'_, HUFFENC_AC1_14rs> {
        HCODE28_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 28
    #[inline(always)]
    pub fn hlen28(&mut self) -> HLEN28_W<'_, HUFFENC_AC1_14rs> {
        HLEN28_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 29
    #[inline(always)]
    pub fn hcode29(&mut self) -> HCODE29_W<'_, HUFFENC_AC1_14rs> {
        HCODE29_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 29
    #[inline(always)]
    pub fn hlen29(&mut self) -> HLEN29_W<'_, HUFFENC_AC1_14rs> {
        HLEN29_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC1_14)*/
pub struct HUFFENC_AC1_14rs;
impl crate::RegisterSpec for HUFFENC_AC1_14rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_14::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_14rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_14::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_14rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_14 to value 0
impl crate::Resettable for HUFFENC_AC1_14rs {}
