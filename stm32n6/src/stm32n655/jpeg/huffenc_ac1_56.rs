///Register `HUFFENC_AC1_56` reader
pub type R = crate::R<HUFFENC_AC1_56rs>;
///Register `HUFFENC_AC1_56` writer
pub type W = crate::W<HUFFENC_AC1_56rs>;
///Field `HCODE112` reader - Huffman code 112
pub type HCODE112_R = crate::FieldReader;
///Field `HCODE112` writer - Huffman code 112
pub type HCODE112_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN112` reader - Huffman length 112
pub type HLEN112_R = crate::FieldReader;
///Field `HLEN112` writer - Huffman length 112
pub type HLEN112_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE113` reader - Huffman code 113
pub type HCODE113_R = crate::FieldReader;
///Field `HCODE113` writer - Huffman code 113
pub type HCODE113_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN113` reader - Huffman length 113
pub type HLEN113_R = crate::FieldReader;
///Field `HLEN113` writer - Huffman length 113
pub type HLEN113_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 112
    #[inline(always)]
    pub fn hcode112(&self) -> HCODE112_R {
        HCODE112_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 112
    #[inline(always)]
    pub fn hlen112(&self) -> HLEN112_R {
        HLEN112_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 113
    #[inline(always)]
    pub fn hcode113(&self) -> HCODE113_R {
        HCODE113_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 113
    #[inline(always)]
    pub fn hlen113(&self) -> HLEN113_R {
        HLEN113_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_56")
            .field("hcode112", &self.hcode112())
            .field("hlen112", &self.hlen112())
            .field("hcode113", &self.hcode113())
            .field("hlen113", &self.hlen113())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 112
    #[inline(always)]
    pub fn hcode112(&mut self) -> HCODE112_W<'_, HUFFENC_AC1_56rs> {
        HCODE112_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 112
    #[inline(always)]
    pub fn hlen112(&mut self) -> HLEN112_W<'_, HUFFENC_AC1_56rs> {
        HLEN112_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 113
    #[inline(always)]
    pub fn hcode113(&mut self) -> HCODE113_W<'_, HUFFENC_AC1_56rs> {
        HCODE113_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 113
    #[inline(always)]
    pub fn hlen113(&mut self) -> HLEN113_W<'_, HUFFENC_AC1_56rs> {
        HLEN113_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_56::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_56::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_56)*/
pub struct HUFFENC_AC1_56rs;
impl crate::RegisterSpec for HUFFENC_AC1_56rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_56::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_56rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_56::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_56rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_56 to value 0
impl crate::Resettable for HUFFENC_AC1_56rs {}
