///Register `HUFFENC_AC1_22` reader
pub type R = crate::R<HUFFENC_AC1_22rs>;
///Register `HUFFENC_AC1_22` writer
pub type W = crate::W<HUFFENC_AC1_22rs>;
///Field `HCODE44` reader - Huffman code 44
pub type HCODE44_R = crate::FieldReader;
///Field `HCODE44` writer - Huffman code 44
pub type HCODE44_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN44` reader - Huffman length 44
pub type HLEN44_R = crate::FieldReader;
///Field `HLEN44` writer - Huffman length 44
pub type HLEN44_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE45` reader - Huffman code 45
pub type HCODE45_R = crate::FieldReader;
///Field `HCODE45` writer - Huffman code 45
pub type HCODE45_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN45` reader - Huffman length 45
pub type HLEN45_R = crate::FieldReader;
///Field `HLEN45` writer - Huffman length 45
pub type HLEN45_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 44
    #[inline(always)]
    pub fn hcode44(&self) -> HCODE44_R {
        HCODE44_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 44
    #[inline(always)]
    pub fn hlen44(&self) -> HLEN44_R {
        HLEN44_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 45
    #[inline(always)]
    pub fn hcode45(&self) -> HCODE45_R {
        HCODE45_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 45
    #[inline(always)]
    pub fn hlen45(&self) -> HLEN45_R {
        HLEN45_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_22")
            .field("hcode44", &self.hcode44())
            .field("hlen44", &self.hlen44())
            .field("hcode45", &self.hcode45())
            .field("hlen45", &self.hlen45())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 44
    #[inline(always)]
    pub fn hcode44(&mut self) -> HCODE44_W<'_, HUFFENC_AC1_22rs> {
        HCODE44_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 44
    #[inline(always)]
    pub fn hlen44(&mut self) -> HLEN44_W<'_, HUFFENC_AC1_22rs> {
        HLEN44_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 45
    #[inline(always)]
    pub fn hcode45(&mut self) -> HCODE45_W<'_, HUFFENC_AC1_22rs> {
        HCODE45_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 45
    #[inline(always)]
    pub fn hlen45(&mut self) -> HLEN45_W<'_, HUFFENC_AC1_22rs> {
        HLEN45_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC1_22)*/
pub struct HUFFENC_AC1_22rs;
impl crate::RegisterSpec for HUFFENC_AC1_22rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_22::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_22rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_22::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_22rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_22 to value 0
impl crate::Resettable for HUFFENC_AC1_22rs {}
