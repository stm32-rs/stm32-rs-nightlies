///Register `HUFFSYMB77` reader
pub type R = crate::R<HUFFSYMB77rs>;
///Register `HUFFSYMB77` writer
pub type W = crate::W<HUFFSYMB77rs>;
///Field `DATA308` reader - Data 308
pub type DATA308_R = crate::FieldReader;
///Field `DATA308` writer - Data 308
pub type DATA308_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA309` reader - Data 309
pub type DATA309_R = crate::FieldReader;
///Field `DATA309` writer - Data 309
pub type DATA309_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA310` reader - Data 310
pub type DATA310_R = crate::FieldReader;
///Field `DATA310` writer - Data 310
pub type DATA310_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA311` reader - Data 311
pub type DATA311_R = crate::FieldReader;
///Field `DATA311` writer - Data 311
pub type DATA311_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 308
    #[inline(always)]
    pub fn data308(&self) -> DATA308_R {
        DATA308_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 309
    #[inline(always)]
    pub fn data309(&self) -> DATA309_R {
        DATA309_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 310
    #[inline(always)]
    pub fn data310(&self) -> DATA310_R {
        DATA310_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 311
    #[inline(always)]
    pub fn data311(&self) -> DATA311_R {
        DATA311_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB77")
            .field("data308", &self.data308())
            .field("data309", &self.data309())
            .field("data310", &self.data310())
            .field("data311", &self.data311())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 308
    #[inline(always)]
    pub fn data308(&mut self) -> DATA308_W<'_, HUFFSYMB77rs> {
        DATA308_W::new(self, 0)
    }
    ///Bits 8:15 - Data 309
    #[inline(always)]
    pub fn data309(&mut self) -> DATA309_W<'_, HUFFSYMB77rs> {
        DATA309_W::new(self, 8)
    }
    ///Bits 16:23 - Data 310
    #[inline(always)]
    pub fn data310(&mut self) -> DATA310_W<'_, HUFFSYMB77rs> {
        DATA310_W::new(self, 16)
    }
    ///Bits 24:31 - Data 311
    #[inline(always)]
    pub fn data311(&mut self) -> DATA311_W<'_, HUFFSYMB77rs> {
        DATA311_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb77::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb77::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFSYMB77)*/
pub struct HUFFSYMB77rs;
impl crate::RegisterSpec for HUFFSYMB77rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb77::R`](R) reader structure
impl crate::Readable for HUFFSYMB77rs {}
///`write(|w| ..)` method takes [`huffsymb77::W`](W) writer structure
impl crate::Writable for HUFFSYMB77rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB77 to value 0
impl crate::Resettable for HUFFSYMB77rs {}
