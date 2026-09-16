///Register `HUFFENC_AC1_85` reader
pub type R = crate::R<HUFFENC_AC1_85rs>;
///Register `HUFFENC_AC1_85` writer
pub type W = crate::W<HUFFENC_AC1_85rs>;
///Field `HCODE170` reader - Huffman code 170
pub type HCODE170_R = crate::FieldReader;
///Field `HCODE170` writer - Huffman code 170
pub type HCODE170_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN170` reader - Huffman length 170
pub type HLEN170_R = crate::FieldReader;
///Field `HLEN170` writer - Huffman length 170
pub type HLEN170_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE171` reader - Huffman code 171
pub type HCODE171_R = crate::FieldReader;
///Field `HCODE171` writer - Huffman code 171
pub type HCODE171_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN171` reader - Huffman length 171
pub type HLEN171_R = crate::FieldReader;
///Field `HLEN171` writer - Huffman length 171
pub type HLEN171_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 170
    #[inline(always)]
    pub fn hcode170(&self) -> HCODE170_R {
        HCODE170_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 170
    #[inline(always)]
    pub fn hlen170(&self) -> HLEN170_R {
        HLEN170_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 171
    #[inline(always)]
    pub fn hcode171(&self) -> HCODE171_R {
        HCODE171_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 171
    #[inline(always)]
    pub fn hlen171(&self) -> HLEN171_R {
        HLEN171_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_85")
            .field("hcode170", &self.hcode170())
            .field("hlen170", &self.hlen170())
            .field("hcode171", &self.hcode171())
            .field("hlen171", &self.hlen171())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 170
    #[inline(always)]
    pub fn hcode170(&mut self) -> HCODE170_W<'_, HUFFENC_AC1_85rs> {
        HCODE170_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 170
    #[inline(always)]
    pub fn hlen170(&mut self) -> HLEN170_W<'_, HUFFENC_AC1_85rs> {
        HLEN170_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 171
    #[inline(always)]
    pub fn hcode171(&mut self) -> HCODE171_W<'_, HUFFENC_AC1_85rs> {
        HCODE171_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 171
    #[inline(always)]
    pub fn hlen171(&mut self) -> HLEN171_W<'_, HUFFENC_AC1_85rs> {
        HLEN171_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_85::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_85::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC1_85)*/
pub struct HUFFENC_AC1_85rs;
impl crate::RegisterSpec for HUFFENC_AC1_85rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_85::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_85rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_85::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_85rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_85 to value 0
impl crate::Resettable for HUFFENC_AC1_85rs {}
