///Register `HUFFENC_AC0_29` reader
pub type R = crate::R<HUFFENC_AC0_29rs>;
///Register `HUFFENC_AC0_29` writer
pub type W = crate::W<HUFFENC_AC0_29rs>;
///Field `HCODE58` reader - Huffman code 58
pub type HCODE58_R = crate::FieldReader;
///Field `HCODE58` writer - Huffman code 58
pub type HCODE58_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN58` reader - Huffman length 58
pub type HLEN58_R = crate::FieldReader;
///Field `HLEN58` writer - Huffman length 58
pub type HLEN58_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE59` reader - Huffman code 59
pub type HCODE59_R = crate::FieldReader;
///Field `HCODE59` writer - Huffman code 59
pub type HCODE59_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN59` reader - Huffman length 59
pub type HLEN59_R = crate::FieldReader;
///Field `HLEN59` writer - Huffman length 59
pub type HLEN59_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 58
    #[inline(always)]
    pub fn hcode58(&self) -> HCODE58_R {
        HCODE58_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 58
    #[inline(always)]
    pub fn hlen58(&self) -> HLEN58_R {
        HLEN58_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 59
    #[inline(always)]
    pub fn hcode59(&self) -> HCODE59_R {
        HCODE59_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 59
    #[inline(always)]
    pub fn hlen59(&self) -> HLEN59_R {
        HLEN59_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_29")
            .field("hcode58", &self.hcode58())
            .field("hlen58", &self.hlen58())
            .field("hcode59", &self.hcode59())
            .field("hlen59", &self.hlen59())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 58
    #[inline(always)]
    pub fn hcode58(&mut self) -> HCODE58_W<'_, HUFFENC_AC0_29rs> {
        HCODE58_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 58
    #[inline(always)]
    pub fn hlen58(&mut self) -> HLEN58_W<'_, HUFFENC_AC0_29rs> {
        HLEN58_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 59
    #[inline(always)]
    pub fn hcode59(&mut self) -> HCODE59_W<'_, HUFFENC_AC0_29rs> {
        HCODE59_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 59
    #[inline(always)]
    pub fn hlen59(&mut self) -> HLEN59_W<'_, HUFFENC_AC0_29rs> {
        HLEN59_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC0_29)*/
pub struct HUFFENC_AC0_29rs;
impl crate::RegisterSpec for HUFFENC_AC0_29rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_29::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_29rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_29::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_29rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_29 to value 0
impl crate::Resettable for HUFFENC_AC0_29rs {}
