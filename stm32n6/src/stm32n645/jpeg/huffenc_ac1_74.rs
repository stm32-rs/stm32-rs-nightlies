///Register `HUFFENC_AC1_74` reader
pub type R = crate::R<HUFFENC_AC1_74rs>;
///Register `HUFFENC_AC1_74` writer
pub type W = crate::W<HUFFENC_AC1_74rs>;
///Field `HCODE148` reader - Huffman code 148
pub type HCODE148_R = crate::FieldReader;
///Field `HCODE148` writer - Huffman code 148
pub type HCODE148_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN148` reader - Huffman length 148
pub type HLEN148_R = crate::FieldReader;
///Field `HLEN148` writer - Huffman length 148
pub type HLEN148_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE149` reader - Huffman code 149
pub type HCODE149_R = crate::FieldReader;
///Field `HCODE149` writer - Huffman code 149
pub type HCODE149_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN149` reader - Huffman length 149
pub type HLEN149_R = crate::FieldReader;
///Field `HLEN149` writer - Huffman length 149
pub type HLEN149_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 148
    #[inline(always)]
    pub fn hcode148(&self) -> HCODE148_R {
        HCODE148_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 148
    #[inline(always)]
    pub fn hlen148(&self) -> HLEN148_R {
        HLEN148_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 149
    #[inline(always)]
    pub fn hcode149(&self) -> HCODE149_R {
        HCODE149_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 149
    #[inline(always)]
    pub fn hlen149(&self) -> HLEN149_R {
        HLEN149_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_74")
            .field("hcode148", &self.hcode148())
            .field("hlen148", &self.hlen148())
            .field("hcode149", &self.hcode149())
            .field("hlen149", &self.hlen149())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 148
    #[inline(always)]
    pub fn hcode148(&mut self) -> HCODE148_W<HUFFENC_AC1_74rs> {
        HCODE148_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 148
    #[inline(always)]
    pub fn hlen148(&mut self) -> HLEN148_W<HUFFENC_AC1_74rs> {
        HLEN148_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 149
    #[inline(always)]
    pub fn hcode149(&mut self) -> HCODE149_W<HUFFENC_AC1_74rs> {
        HCODE149_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 149
    #[inline(always)]
    pub fn hlen149(&mut self) -> HLEN149_W<HUFFENC_AC1_74rs> {
        HLEN149_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_74::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_74::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC1_74)*/
pub struct HUFFENC_AC1_74rs;
impl crate::RegisterSpec for HUFFENC_AC1_74rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_74::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_74rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_74::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_74rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_74 to value 0
impl crate::Resettable for HUFFENC_AC1_74rs {}
