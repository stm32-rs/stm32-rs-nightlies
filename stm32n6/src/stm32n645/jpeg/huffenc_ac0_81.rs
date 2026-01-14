///Register `HUFFENC_AC0_81` reader
pub type R = crate::R<HUFFENC_AC0_81rs>;
///Register `HUFFENC_AC0_81` writer
pub type W = crate::W<HUFFENC_AC0_81rs>;
///Field `HCODE162` reader - Huffman code 162
pub type HCODE162_R = crate::FieldReader;
///Field `HCODE162` writer - Huffman code 162
pub type HCODE162_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN162` reader - Huffman length 162
pub type HLEN162_R = crate::FieldReader;
///Field `HLEN162` writer - Huffman length 162
pub type HLEN162_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE163` reader - Huffman code 163
pub type HCODE163_R = crate::FieldReader;
///Field `HCODE163` writer - Huffman code 163
pub type HCODE163_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN163` reader - Huffman length 163
pub type HLEN163_R = crate::FieldReader;
///Field `HLEN163` writer - Huffman length 163
pub type HLEN163_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 162
    #[inline(always)]
    pub fn hcode162(&self) -> HCODE162_R {
        HCODE162_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 162
    #[inline(always)]
    pub fn hlen162(&self) -> HLEN162_R {
        HLEN162_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 163
    #[inline(always)]
    pub fn hcode163(&self) -> HCODE163_R {
        HCODE163_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 163
    #[inline(always)]
    pub fn hlen163(&self) -> HLEN163_R {
        HLEN163_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_81")
            .field("hcode162", &self.hcode162())
            .field("hlen162", &self.hlen162())
            .field("hcode163", &self.hcode163())
            .field("hlen163", &self.hlen163())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 162
    #[inline(always)]
    pub fn hcode162(&mut self) -> HCODE162_W<'_, HUFFENC_AC0_81rs> {
        HCODE162_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 162
    #[inline(always)]
    pub fn hlen162(&mut self) -> HLEN162_W<'_, HUFFENC_AC0_81rs> {
        HLEN162_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 163
    #[inline(always)]
    pub fn hcode163(&mut self) -> HCODE163_W<'_, HUFFENC_AC0_81rs> {
        HCODE163_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 163
    #[inline(always)]
    pub fn hlen163(&mut self) -> HLEN163_W<'_, HUFFENC_AC0_81rs> {
        HLEN163_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_81::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_81::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC0_81)*/
pub struct HUFFENC_AC0_81rs;
impl crate::RegisterSpec for HUFFENC_AC0_81rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_81::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_81rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_81::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_81rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_81 to value 0
impl crate::Resettable for HUFFENC_AC0_81rs {}
