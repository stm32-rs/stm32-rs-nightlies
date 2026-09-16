///Register `HUFFENC_AC0_52` reader
pub type R = crate::R<HUFFENC_AC0_52rs>;
///Register `HUFFENC_AC0_52` writer
pub type W = crate::W<HUFFENC_AC0_52rs>;
///Field `HCODE104` reader - Huffman code 104
pub type HCODE104_R = crate::FieldReader;
///Field `HCODE104` writer - Huffman code 104
pub type HCODE104_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN104` reader - Huffman length 104
pub type HLEN104_R = crate::FieldReader;
///Field `HLEN104` writer - Huffman length 104
pub type HLEN104_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE105` reader - Huffman code 105
pub type HCODE105_R = crate::FieldReader;
///Field `HCODE105` writer - Huffman code 105
pub type HCODE105_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN105` reader - Huffman length 105
pub type HLEN105_R = crate::FieldReader;
///Field `HLEN105` writer - Huffman length 105
pub type HLEN105_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 104
    #[inline(always)]
    pub fn hcode104(&self) -> HCODE104_R {
        HCODE104_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 104
    #[inline(always)]
    pub fn hlen104(&self) -> HLEN104_R {
        HLEN104_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 105
    #[inline(always)]
    pub fn hcode105(&self) -> HCODE105_R {
        HCODE105_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 105
    #[inline(always)]
    pub fn hlen105(&self) -> HLEN105_R {
        HLEN105_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_52")
            .field("hcode104", &self.hcode104())
            .field("hlen104", &self.hlen104())
            .field("hcode105", &self.hcode105())
            .field("hlen105", &self.hlen105())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 104
    #[inline(always)]
    pub fn hcode104(&mut self) -> HCODE104_W<'_, HUFFENC_AC0_52rs> {
        HCODE104_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 104
    #[inline(always)]
    pub fn hlen104(&mut self) -> HLEN104_W<'_, HUFFENC_AC0_52rs> {
        HLEN104_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 105
    #[inline(always)]
    pub fn hcode105(&mut self) -> HCODE105_W<'_, HUFFENC_AC0_52rs> {
        HCODE105_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 105
    #[inline(always)]
    pub fn hlen105(&mut self) -> HLEN105_W<'_, HUFFENC_AC0_52rs> {
        HLEN105_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_52::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_52::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC0_52)*/
pub struct HUFFENC_AC0_52rs;
impl crate::RegisterSpec for HUFFENC_AC0_52rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_52::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_52rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_52::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_52rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_52 to value 0
impl crate::Resettable for HUFFENC_AC0_52rs {}
