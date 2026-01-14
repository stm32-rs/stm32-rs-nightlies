///Register `HUFFENC_AC0_43` reader
pub type R = crate::R<HUFFENC_AC0_43rs>;
///Register `HUFFENC_AC0_43` writer
pub type W = crate::W<HUFFENC_AC0_43rs>;
///Field `HCODE86` reader - Huffman code 86
pub type HCODE86_R = crate::FieldReader;
///Field `HCODE86` writer - Huffman code 86
pub type HCODE86_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN86` reader - Huffman length 86
pub type HLEN86_R = crate::FieldReader;
///Field `HLEN86` writer - Huffman length 86
pub type HLEN86_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE87` reader - Huffman code 87
pub type HCODE87_R = crate::FieldReader;
///Field `HCODE87` writer - Huffman code 87
pub type HCODE87_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN87` reader - Huffman length 87
pub type HLEN87_R = crate::FieldReader;
///Field `HLEN87` writer - Huffman length 87
pub type HLEN87_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 86
    #[inline(always)]
    pub fn hcode86(&self) -> HCODE86_R {
        HCODE86_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 86
    #[inline(always)]
    pub fn hlen86(&self) -> HLEN86_R {
        HLEN86_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 87
    #[inline(always)]
    pub fn hcode87(&self) -> HCODE87_R {
        HCODE87_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 87
    #[inline(always)]
    pub fn hlen87(&self) -> HLEN87_R {
        HLEN87_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_43")
            .field("hcode86", &self.hcode86())
            .field("hlen86", &self.hlen86())
            .field("hcode87", &self.hcode87())
            .field("hlen87", &self.hlen87())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 86
    #[inline(always)]
    pub fn hcode86(&mut self) -> HCODE86_W<'_, HUFFENC_AC0_43rs> {
        HCODE86_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 86
    #[inline(always)]
    pub fn hlen86(&mut self) -> HLEN86_W<'_, HUFFENC_AC0_43rs> {
        HLEN86_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 87
    #[inline(always)]
    pub fn hcode87(&mut self) -> HCODE87_W<'_, HUFFENC_AC0_43rs> {
        HCODE87_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 87
    #[inline(always)]
    pub fn hlen87(&mut self) -> HLEN87_W<'_, HUFFENC_AC0_43rs> {
        HLEN87_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_43::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_43::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC0_43)*/
pub struct HUFFENC_AC0_43rs;
impl crate::RegisterSpec for HUFFENC_AC0_43rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_43::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_43rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_43::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_43rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_43 to value 0
impl crate::Resettable for HUFFENC_AC0_43rs {}
