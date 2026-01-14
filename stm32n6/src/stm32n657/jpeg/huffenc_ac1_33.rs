///Register `HUFFENC_AC1_33` reader
pub type R = crate::R<HUFFENC_AC1_33rs>;
///Register `HUFFENC_AC1_33` writer
pub type W = crate::W<HUFFENC_AC1_33rs>;
///Field `HCODE66` reader - Huffman code 66
pub type HCODE66_R = crate::FieldReader;
///Field `HCODE66` writer - Huffman code 66
pub type HCODE66_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN66` reader - Huffman length 66
pub type HLEN66_R = crate::FieldReader;
///Field `HLEN66` writer - Huffman length 66
pub type HLEN66_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE67` reader - Huffman code 67
pub type HCODE67_R = crate::FieldReader;
///Field `HCODE67` writer - Huffman code 67
pub type HCODE67_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN67` reader - Huffman length 67
pub type HLEN67_R = crate::FieldReader;
///Field `HLEN67` writer - Huffman length 67
pub type HLEN67_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 66
    #[inline(always)]
    pub fn hcode66(&self) -> HCODE66_R {
        HCODE66_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 66
    #[inline(always)]
    pub fn hlen66(&self) -> HLEN66_R {
        HLEN66_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 67
    #[inline(always)]
    pub fn hcode67(&self) -> HCODE67_R {
        HCODE67_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 67
    #[inline(always)]
    pub fn hlen67(&self) -> HLEN67_R {
        HLEN67_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_33")
            .field("hcode66", &self.hcode66())
            .field("hlen66", &self.hlen66())
            .field("hcode67", &self.hcode67())
            .field("hlen67", &self.hlen67())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 66
    #[inline(always)]
    pub fn hcode66(&mut self) -> HCODE66_W<'_, HUFFENC_AC1_33rs> {
        HCODE66_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 66
    #[inline(always)]
    pub fn hlen66(&mut self) -> HLEN66_W<'_, HUFFENC_AC1_33rs> {
        HLEN66_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 67
    #[inline(always)]
    pub fn hcode67(&mut self) -> HCODE67_W<'_, HUFFENC_AC1_33rs> {
        HCODE67_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 67
    #[inline(always)]
    pub fn hlen67(&mut self) -> HLEN67_W<'_, HUFFENC_AC1_33rs> {
        HLEN67_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC1_33)*/
pub struct HUFFENC_AC1_33rs;
impl crate::RegisterSpec for HUFFENC_AC1_33rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_33::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_33rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_33::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_33rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_33 to value 0
impl crate::Resettable for HUFFENC_AC1_33rs {}
