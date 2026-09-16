///Register `HUFFSYMB80` reader
pub type R = crate::R<HUFFSYMB80rs>;
///Register `HUFFSYMB80` writer
pub type W = crate::W<HUFFSYMB80rs>;
///Field `DATA320` reader - Data 320
pub type DATA320_R = crate::FieldReader;
///Field `DATA320` writer - Data 320
pub type DATA320_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA321` reader - Data 321
pub type DATA321_R = crate::FieldReader;
///Field `DATA321` writer - Data 321
pub type DATA321_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA322` reader - Data 322
pub type DATA322_R = crate::FieldReader;
///Field `DATA322` writer - Data 322
pub type DATA322_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA323` reader - Data 323
pub type DATA323_R = crate::FieldReader;
///Field `DATA323` writer - Data 323
pub type DATA323_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 320
    #[inline(always)]
    pub fn data320(&self) -> DATA320_R {
        DATA320_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 321
    #[inline(always)]
    pub fn data321(&self) -> DATA321_R {
        DATA321_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 322
    #[inline(always)]
    pub fn data322(&self) -> DATA322_R {
        DATA322_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 323
    #[inline(always)]
    pub fn data323(&self) -> DATA323_R {
        DATA323_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB80")
            .field("data320", &self.data320())
            .field("data321", &self.data321())
            .field("data322", &self.data322())
            .field("data323", &self.data323())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 320
    #[inline(always)]
    pub fn data320(&mut self) -> DATA320_W<'_, HUFFSYMB80rs> {
        DATA320_W::new(self, 0)
    }
    ///Bits 8:15 - Data 321
    #[inline(always)]
    pub fn data321(&mut self) -> DATA321_W<'_, HUFFSYMB80rs> {
        DATA321_W::new(self, 8)
    }
    ///Bits 16:23 - Data 322
    #[inline(always)]
    pub fn data322(&mut self) -> DATA322_W<'_, HUFFSYMB80rs> {
        DATA322_W::new(self, 16)
    }
    ///Bits 24:31 - Data 323
    #[inline(always)]
    pub fn data323(&mut self) -> DATA323_W<'_, HUFFSYMB80rs> {
        DATA323_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb80::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb80::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFSYMB80)*/
pub struct HUFFSYMB80rs;
impl crate::RegisterSpec for HUFFSYMB80rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb80::R`](R) reader structure
impl crate::Readable for HUFFSYMB80rs {}
///`write(|w| ..)` method takes [`huffsymb80::W`](W) writer structure
impl crate::Writable for HUFFSYMB80rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB80 to value 0
impl crate::Resettable for HUFFSYMB80rs {}
