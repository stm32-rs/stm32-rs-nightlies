///Register `HUFFENC_AC1_60` reader
pub type R = crate::R<HUFFENC_AC1_60rs>;
///Register `HUFFENC_AC1_60` writer
pub type W = crate::W<HUFFENC_AC1_60rs>;
///Field `HCODE120` reader - Huffman code 120
pub type HCODE120_R = crate::FieldReader;
///Field `HCODE120` writer - Huffman code 120
pub type HCODE120_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN120` reader - Huffman length 120
pub type HLEN120_R = crate::FieldReader;
///Field `HLEN120` writer - Huffman length 120
pub type HLEN120_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE121` reader - Huffman code 121
pub type HCODE121_R = crate::FieldReader;
///Field `HCODE121` writer - Huffman code 121
pub type HCODE121_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN121` reader - Huffman length 121
pub type HLEN121_R = crate::FieldReader;
///Field `HLEN121` writer - Huffman length 121
pub type HLEN121_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 120
    #[inline(always)]
    pub fn hcode120(&self) -> HCODE120_R {
        HCODE120_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 120
    #[inline(always)]
    pub fn hlen120(&self) -> HLEN120_R {
        HLEN120_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 121
    #[inline(always)]
    pub fn hcode121(&self) -> HCODE121_R {
        HCODE121_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 121
    #[inline(always)]
    pub fn hlen121(&self) -> HLEN121_R {
        HLEN121_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_60")
            .field("hcode120", &self.hcode120())
            .field("hlen120", &self.hlen120())
            .field("hcode121", &self.hcode121())
            .field("hlen121", &self.hlen121())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 120
    #[inline(always)]
    pub fn hcode120(&mut self) -> HCODE120_W<'_, HUFFENC_AC1_60rs> {
        HCODE120_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 120
    #[inline(always)]
    pub fn hlen120(&mut self) -> HLEN120_W<'_, HUFFENC_AC1_60rs> {
        HLEN120_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 121
    #[inline(always)]
    pub fn hcode121(&mut self) -> HCODE121_W<'_, HUFFENC_AC1_60rs> {
        HCODE121_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 121
    #[inline(always)]
    pub fn hlen121(&mut self) -> HLEN121_W<'_, HUFFENC_AC1_60rs> {
        HLEN121_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC1_60)*/
pub struct HUFFENC_AC1_60rs;
impl crate::RegisterSpec for HUFFENC_AC1_60rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_60::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_60rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_60::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_60rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_60 to value 0
impl crate::Resettable for HUFFENC_AC1_60rs {}
