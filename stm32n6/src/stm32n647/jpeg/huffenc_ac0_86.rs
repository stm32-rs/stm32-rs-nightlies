///Register `HUFFENC_AC0_86` reader
pub type R = crate::R<HUFFENC_AC0_86rs>;
///Register `HUFFENC_AC0_86` writer
pub type W = crate::W<HUFFENC_AC0_86rs>;
///Field `HCODE172` reader - Huffman code 172
pub type HCODE172_R = crate::FieldReader;
///Field `HCODE172` writer - Huffman code 172
pub type HCODE172_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN172` reader - Huffman length 172
pub type HLEN172_R = crate::FieldReader;
///Field `HLEN172` writer - Huffman length 172
pub type HLEN172_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE173` reader - Huffman code 173
pub type HCODE173_R = crate::FieldReader;
///Field `HCODE173` writer - Huffman code 173
pub type HCODE173_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN173` reader - Huffman length 173
pub type HLEN173_R = crate::FieldReader;
///Field `HLEN173` writer - Huffman length 173
pub type HLEN173_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 172
    #[inline(always)]
    pub fn hcode172(&self) -> HCODE172_R {
        HCODE172_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 172
    #[inline(always)]
    pub fn hlen172(&self) -> HLEN172_R {
        HLEN172_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 173
    #[inline(always)]
    pub fn hcode173(&self) -> HCODE173_R {
        HCODE173_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 173
    #[inline(always)]
    pub fn hlen173(&self) -> HLEN173_R {
        HLEN173_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_86")
            .field("hcode172", &self.hcode172())
            .field("hlen172", &self.hlen172())
            .field("hcode173", &self.hcode173())
            .field("hlen173", &self.hlen173())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 172
    #[inline(always)]
    pub fn hcode172(&mut self) -> HCODE172_W<'_, HUFFENC_AC0_86rs> {
        HCODE172_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 172
    #[inline(always)]
    pub fn hlen172(&mut self) -> HLEN172_W<'_, HUFFENC_AC0_86rs> {
        HLEN172_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 173
    #[inline(always)]
    pub fn hcode173(&mut self) -> HCODE173_W<'_, HUFFENC_AC0_86rs> {
        HCODE173_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 173
    #[inline(always)]
    pub fn hlen173(&mut self) -> HLEN173_W<'_, HUFFENC_AC0_86rs> {
        HLEN173_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_86::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_86::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC0_86)*/
pub struct HUFFENC_AC0_86rs;
impl crate::RegisterSpec for HUFFENC_AC0_86rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_86::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_86rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_86::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_86rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_86 to value 0
impl crate::Resettable for HUFFENC_AC0_86rs {}
