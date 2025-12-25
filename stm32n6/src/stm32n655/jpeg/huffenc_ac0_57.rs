///Register `HUFFENC_AC0_57` reader
pub type R = crate::R<HUFFENC_AC0_57rs>;
///Register `HUFFENC_AC0_57` writer
pub type W = crate::W<HUFFENC_AC0_57rs>;
///Field `HCODE114` reader - Huffman code 114
pub type HCODE114_R = crate::FieldReader;
///Field `HCODE114` writer - Huffman code 114
pub type HCODE114_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN114` reader - Huffman length 114
pub type HLEN114_R = crate::FieldReader;
///Field `HLEN114` writer - Huffman length 114
pub type HLEN114_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE115` reader - Huffman code 115
pub type HCODE115_R = crate::FieldReader;
///Field `HCODE115` writer - Huffman code 115
pub type HCODE115_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN115` reader - Huffman length 115
pub type HLEN115_R = crate::FieldReader;
///Field `HLEN115` writer - Huffman length 115
pub type HLEN115_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 114
    #[inline(always)]
    pub fn hcode114(&self) -> HCODE114_R {
        HCODE114_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 114
    #[inline(always)]
    pub fn hlen114(&self) -> HLEN114_R {
        HLEN114_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 115
    #[inline(always)]
    pub fn hcode115(&self) -> HCODE115_R {
        HCODE115_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 115
    #[inline(always)]
    pub fn hlen115(&self) -> HLEN115_R {
        HLEN115_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_57")
            .field("hcode114", &self.hcode114())
            .field("hlen114", &self.hlen114())
            .field("hcode115", &self.hcode115())
            .field("hlen115", &self.hlen115())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 114
    #[inline(always)]
    pub fn hcode114(&mut self) -> HCODE114_W<'_, HUFFENC_AC0_57rs> {
        HCODE114_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 114
    #[inline(always)]
    pub fn hlen114(&mut self) -> HLEN114_W<'_, HUFFENC_AC0_57rs> {
        HLEN114_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 115
    #[inline(always)]
    pub fn hcode115(&mut self) -> HCODE115_W<'_, HUFFENC_AC0_57rs> {
        HCODE115_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 115
    #[inline(always)]
    pub fn hlen115(&mut self) -> HLEN115_W<'_, HUFFENC_AC0_57rs> {
        HLEN115_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_57::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_57::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_57)*/
pub struct HUFFENC_AC0_57rs;
impl crate::RegisterSpec for HUFFENC_AC0_57rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_57::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_57rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_57::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_57rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_57 to value 0
impl crate::Resettable for HUFFENC_AC0_57rs {}
