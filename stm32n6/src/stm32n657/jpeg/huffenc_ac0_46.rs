///Register `HUFFENC_AC0_46` reader
pub type R = crate::R<HUFFENC_AC0_46rs>;
///Register `HUFFENC_AC0_46` writer
pub type W = crate::W<HUFFENC_AC0_46rs>;
///Field `HCODE92` reader - Huffman code 92
pub type HCODE92_R = crate::FieldReader;
///Field `HCODE92` writer - Huffman code 92
pub type HCODE92_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN92` reader - Huffman length 92
pub type HLEN92_R = crate::FieldReader;
///Field `HLEN92` writer - Huffman length 92
pub type HLEN92_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE93` reader - Huffman code 93
pub type HCODE93_R = crate::FieldReader;
///Field `HCODE93` writer - Huffman code 93
pub type HCODE93_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN93` reader - Huffman length 93
pub type HLEN93_R = crate::FieldReader;
///Field `HLEN93` writer - Huffman length 93
pub type HLEN93_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 92
    #[inline(always)]
    pub fn hcode92(&self) -> HCODE92_R {
        HCODE92_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 92
    #[inline(always)]
    pub fn hlen92(&self) -> HLEN92_R {
        HLEN92_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 93
    #[inline(always)]
    pub fn hcode93(&self) -> HCODE93_R {
        HCODE93_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 93
    #[inline(always)]
    pub fn hlen93(&self) -> HLEN93_R {
        HLEN93_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_46")
            .field("hcode92", &self.hcode92())
            .field("hlen92", &self.hlen92())
            .field("hcode93", &self.hcode93())
            .field("hlen93", &self.hlen93())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 92
    #[inline(always)]
    pub fn hcode92(&mut self) -> HCODE92_W<'_, HUFFENC_AC0_46rs> {
        HCODE92_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 92
    #[inline(always)]
    pub fn hlen92(&mut self) -> HLEN92_W<'_, HUFFENC_AC0_46rs> {
        HLEN92_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 93
    #[inline(always)]
    pub fn hcode93(&mut self) -> HCODE93_W<'_, HUFFENC_AC0_46rs> {
        HCODE93_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 93
    #[inline(always)]
    pub fn hlen93(&mut self) -> HLEN93_W<'_, HUFFENC_AC0_46rs> {
        HLEN93_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_46::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_46::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC0_46)*/
pub struct HUFFENC_AC0_46rs;
impl crate::RegisterSpec for HUFFENC_AC0_46rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_46::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_46rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_46::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_46rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_46 to value 0
impl crate::Resettable for HUFFENC_AC0_46rs {}
