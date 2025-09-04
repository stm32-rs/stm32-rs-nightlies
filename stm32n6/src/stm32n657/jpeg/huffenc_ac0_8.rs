///Register `HUFFENC_AC0_8` reader
pub type R = crate::R<HUFFENC_AC0_8rs>;
///Register `HUFFENC_AC0_8` writer
pub type W = crate::W<HUFFENC_AC0_8rs>;
///Field `HCODE16` reader - Huffman code 16
pub type HCODE16_R = crate::FieldReader;
///Field `HCODE16` writer - Huffman code 16
pub type HCODE16_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN16` reader - Huffman length 16
pub type HLEN16_R = crate::FieldReader;
///Field `HLEN16` writer - Huffman length 16
pub type HLEN16_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE17` reader - Huffman code 17
pub type HCODE17_R = crate::FieldReader;
///Field `HCODE17` writer - Huffman code 17
pub type HCODE17_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN17` reader - Huffman length 17
pub type HLEN17_R = crate::FieldReader;
///Field `HLEN17` writer - Huffman length 17
pub type HLEN17_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 16
    #[inline(always)]
    pub fn hcode16(&self) -> HCODE16_R {
        HCODE16_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 16
    #[inline(always)]
    pub fn hlen16(&self) -> HLEN16_R {
        HLEN16_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 17
    #[inline(always)]
    pub fn hcode17(&self) -> HCODE17_R {
        HCODE17_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 17
    #[inline(always)]
    pub fn hlen17(&self) -> HLEN17_R {
        HLEN17_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_8")
            .field("hcode16", &self.hcode16())
            .field("hlen16", &self.hlen16())
            .field("hcode17", &self.hcode17())
            .field("hlen17", &self.hlen17())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 16
    #[inline(always)]
    pub fn hcode16(&mut self) -> HCODE16_W<HUFFENC_AC0_8rs> {
        HCODE16_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 16
    #[inline(always)]
    pub fn hlen16(&mut self) -> HLEN16_W<HUFFENC_AC0_8rs> {
        HLEN16_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 17
    #[inline(always)]
    pub fn hcode17(&mut self) -> HCODE17_W<HUFFENC_AC0_8rs> {
        HCODE17_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 17
    #[inline(always)]
    pub fn hlen17(&mut self) -> HLEN17_W<HUFFENC_AC0_8rs> {
        HLEN17_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC0_8)*/
pub struct HUFFENC_AC0_8rs;
impl crate::RegisterSpec for HUFFENC_AC0_8rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_8::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_8rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_8::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_8 to value 0
impl crate::Resettable for HUFFENC_AC0_8rs {}
