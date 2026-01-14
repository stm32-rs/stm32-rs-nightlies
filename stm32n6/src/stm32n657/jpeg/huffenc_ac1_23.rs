///Register `HUFFENC_AC1_23` reader
pub type R = crate::R<HUFFENC_AC1_23rs>;
///Register `HUFFENC_AC1_23` writer
pub type W = crate::W<HUFFENC_AC1_23rs>;
///Field `HCODE46` reader - Huffman code 46
pub type HCODE46_R = crate::FieldReader;
///Field `HCODE46` writer - Huffman code 46
pub type HCODE46_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN46` reader - Huffman length 46
pub type HLEN46_R = crate::FieldReader;
///Field `HLEN46` writer - Huffman length 46
pub type HLEN46_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE47` reader - Huffman code 47
pub type HCODE47_R = crate::FieldReader;
///Field `HCODE47` writer - Huffman code 47
pub type HCODE47_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN47` reader - Huffman length 47
pub type HLEN47_R = crate::FieldReader;
///Field `HLEN47` writer - Huffman length 47
pub type HLEN47_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 46
    #[inline(always)]
    pub fn hcode46(&self) -> HCODE46_R {
        HCODE46_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 46
    #[inline(always)]
    pub fn hlen46(&self) -> HLEN46_R {
        HLEN46_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 47
    #[inline(always)]
    pub fn hcode47(&self) -> HCODE47_R {
        HCODE47_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 47
    #[inline(always)]
    pub fn hlen47(&self) -> HLEN47_R {
        HLEN47_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_23")
            .field("hcode46", &self.hcode46())
            .field("hlen46", &self.hlen46())
            .field("hcode47", &self.hcode47())
            .field("hlen47", &self.hlen47())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 46
    #[inline(always)]
    pub fn hcode46(&mut self) -> HCODE46_W<'_, HUFFENC_AC1_23rs> {
        HCODE46_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 46
    #[inline(always)]
    pub fn hlen46(&mut self) -> HLEN46_W<'_, HUFFENC_AC1_23rs> {
        HLEN46_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 47
    #[inline(always)]
    pub fn hcode47(&mut self) -> HCODE47_W<'_, HUFFENC_AC1_23rs> {
        HCODE47_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 47
    #[inline(always)]
    pub fn hlen47(&mut self) -> HLEN47_W<'_, HUFFENC_AC1_23rs> {
        HLEN47_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC1_23)*/
pub struct HUFFENC_AC1_23rs;
impl crate::RegisterSpec for HUFFENC_AC1_23rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_23::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_23rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_23::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_23rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_23 to value 0
impl crate::Resettable for HUFFENC_AC1_23rs {}
