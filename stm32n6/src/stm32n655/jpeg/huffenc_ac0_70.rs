///Register `HUFFENC_AC0_70` reader
pub type R = crate::R<HUFFENC_AC0_70rs>;
///Register `HUFFENC_AC0_70` writer
pub type W = crate::W<HUFFENC_AC0_70rs>;
///Field `HCODE140` reader - Huffman code 140
pub type HCODE140_R = crate::FieldReader;
///Field `HCODE140` writer - Huffman code 140
pub type HCODE140_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN140` reader - Huffman length 140
pub type HLEN140_R = crate::FieldReader;
///Field `HLEN140` writer - Huffman length 140
pub type HLEN140_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE141` reader - Huffman code 141
pub type HCODE141_R = crate::FieldReader;
///Field `HCODE141` writer - Huffman code 141
pub type HCODE141_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN141` reader - Huffman length 141
pub type HLEN141_R = crate::FieldReader;
///Field `HLEN141` writer - Huffman length 141
pub type HLEN141_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 140
    #[inline(always)]
    pub fn hcode140(&self) -> HCODE140_R {
        HCODE140_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 140
    #[inline(always)]
    pub fn hlen140(&self) -> HLEN140_R {
        HLEN140_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 141
    #[inline(always)]
    pub fn hcode141(&self) -> HCODE141_R {
        HCODE141_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 141
    #[inline(always)]
    pub fn hlen141(&self) -> HLEN141_R {
        HLEN141_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_70")
            .field("hcode140", &self.hcode140())
            .field("hlen140", &self.hlen140())
            .field("hcode141", &self.hcode141())
            .field("hlen141", &self.hlen141())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 140
    #[inline(always)]
    pub fn hcode140(&mut self) -> HCODE140_W<'_, HUFFENC_AC0_70rs> {
        HCODE140_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 140
    #[inline(always)]
    pub fn hlen140(&mut self) -> HLEN140_W<'_, HUFFENC_AC0_70rs> {
        HLEN140_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 141
    #[inline(always)]
    pub fn hcode141(&mut self) -> HCODE141_W<'_, HUFFENC_AC0_70rs> {
        HCODE141_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 141
    #[inline(always)]
    pub fn hlen141(&mut self) -> HLEN141_W<'_, HUFFENC_AC0_70rs> {
        HLEN141_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_70::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_70::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_70)*/
pub struct HUFFENC_AC0_70rs;
impl crate::RegisterSpec for HUFFENC_AC0_70rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_70::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_70rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_70::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_70rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_70 to value 0
impl crate::Resettable for HUFFENC_AC0_70rs {}
