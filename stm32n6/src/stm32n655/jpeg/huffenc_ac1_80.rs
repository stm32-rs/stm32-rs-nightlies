///Register `HUFFENC_AC1_80` reader
pub type R = crate::R<HUFFENC_AC1_80rs>;
///Register `HUFFENC_AC1_80` writer
pub type W = crate::W<HUFFENC_AC1_80rs>;
///Field `HCODE160` reader - Huffman code 160
pub type HCODE160_R = crate::FieldReader;
///Field `HCODE160` writer - Huffman code 160
pub type HCODE160_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN160` reader - Huffman length 160
pub type HLEN160_R = crate::FieldReader;
///Field `HLEN160` writer - Huffman length 160
pub type HLEN160_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE161` reader - Huffman code 161
pub type HCODE161_R = crate::FieldReader;
///Field `HCODE161` writer - Huffman code 161
pub type HCODE161_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN161` reader - Huffman length 161
pub type HLEN161_R = crate::FieldReader;
///Field `HLEN161` writer - Huffman length 161
pub type HLEN161_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 160
    #[inline(always)]
    pub fn hcode160(&self) -> HCODE160_R {
        HCODE160_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 160
    #[inline(always)]
    pub fn hlen160(&self) -> HLEN160_R {
        HLEN160_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 161
    #[inline(always)]
    pub fn hcode161(&self) -> HCODE161_R {
        HCODE161_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 161
    #[inline(always)]
    pub fn hlen161(&self) -> HLEN161_R {
        HLEN161_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_80")
            .field("hcode160", &self.hcode160())
            .field("hlen160", &self.hlen160())
            .field("hcode161", &self.hcode161())
            .field("hlen161", &self.hlen161())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 160
    #[inline(always)]
    pub fn hcode160(&mut self) -> HCODE160_W<'_, HUFFENC_AC1_80rs> {
        HCODE160_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 160
    #[inline(always)]
    pub fn hlen160(&mut self) -> HLEN160_W<'_, HUFFENC_AC1_80rs> {
        HLEN160_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 161
    #[inline(always)]
    pub fn hcode161(&mut self) -> HCODE161_W<'_, HUFFENC_AC1_80rs> {
        HCODE161_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 161
    #[inline(always)]
    pub fn hlen161(&mut self) -> HLEN161_W<'_, HUFFENC_AC1_80rs> {
        HLEN161_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_80::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_80::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_80)*/
pub struct HUFFENC_AC1_80rs;
impl crate::RegisterSpec for HUFFENC_AC1_80rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_80::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_80rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_80::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_80rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_80 to value 0
impl crate::Resettable for HUFFENC_AC1_80rs {}
