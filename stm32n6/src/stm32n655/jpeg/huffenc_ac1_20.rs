///Register `HUFFENC_AC1_20` reader
pub type R = crate::R<HUFFENC_AC1_20rs>;
///Register `HUFFENC_AC1_20` writer
pub type W = crate::W<HUFFENC_AC1_20rs>;
///Field `HCODE40` reader - Huffman code 40
pub type HCODE40_R = crate::FieldReader;
///Field `HCODE40` writer - Huffman code 40
pub type HCODE40_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN40` reader - Huffman length 40
pub type HLEN40_R = crate::FieldReader;
///Field `HLEN40` writer - Huffman length 40
pub type HLEN40_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE41` reader - Huffman code 41
pub type HCODE41_R = crate::FieldReader;
///Field `HCODE41` writer - Huffman code 41
pub type HCODE41_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN41` reader - Huffman length 41
pub type HLEN41_R = crate::FieldReader;
///Field `HLEN41` writer - Huffman length 41
pub type HLEN41_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 40
    #[inline(always)]
    pub fn hcode40(&self) -> HCODE40_R {
        HCODE40_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 40
    #[inline(always)]
    pub fn hlen40(&self) -> HLEN40_R {
        HLEN40_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 41
    #[inline(always)]
    pub fn hcode41(&self) -> HCODE41_R {
        HCODE41_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 41
    #[inline(always)]
    pub fn hlen41(&self) -> HLEN41_R {
        HLEN41_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_20")
            .field("hcode40", &self.hcode40())
            .field("hlen40", &self.hlen40())
            .field("hcode41", &self.hcode41())
            .field("hlen41", &self.hlen41())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 40
    #[inline(always)]
    pub fn hcode40(&mut self) -> HCODE40_W<'_, HUFFENC_AC1_20rs> {
        HCODE40_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 40
    #[inline(always)]
    pub fn hlen40(&mut self) -> HLEN40_W<'_, HUFFENC_AC1_20rs> {
        HLEN40_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 41
    #[inline(always)]
    pub fn hcode41(&mut self) -> HCODE41_W<'_, HUFFENC_AC1_20rs> {
        HCODE41_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 41
    #[inline(always)]
    pub fn hlen41(&mut self) -> HLEN41_W<'_, HUFFENC_AC1_20rs> {
        HLEN41_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_20)*/
pub struct HUFFENC_AC1_20rs;
impl crate::RegisterSpec for HUFFENC_AC1_20rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_20::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_20rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_20::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_20rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_20 to value 0
impl crate::Resettable for HUFFENC_AC1_20rs {}
