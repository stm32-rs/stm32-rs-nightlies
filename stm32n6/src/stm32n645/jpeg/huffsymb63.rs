///Register `HUFFSYMB63` reader
pub type R = crate::R<HUFFSYMB63rs>;
///Register `HUFFSYMB63` writer
pub type W = crate::W<HUFFSYMB63rs>;
///Field `DATA252` reader - Data 252
pub type DATA252_R = crate::FieldReader;
///Field `DATA252` writer - Data 252
pub type DATA252_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA253` reader - Data 253
pub type DATA253_R = crate::FieldReader;
///Field `DATA253` writer - Data 253
pub type DATA253_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA254` reader - Data 254
pub type DATA254_R = crate::FieldReader;
///Field `DATA254` writer - Data 254
pub type DATA254_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA255` reader - Data 255
pub type DATA255_R = crate::FieldReader;
///Field `DATA255` writer - Data 255
pub type DATA255_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 252
    #[inline(always)]
    pub fn data252(&self) -> DATA252_R {
        DATA252_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 253
    #[inline(always)]
    pub fn data253(&self) -> DATA253_R {
        DATA253_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 254
    #[inline(always)]
    pub fn data254(&self) -> DATA254_R {
        DATA254_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 255
    #[inline(always)]
    pub fn data255(&self) -> DATA255_R {
        DATA255_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB63")
            .field("data252", &self.data252())
            .field("data253", &self.data253())
            .field("data254", &self.data254())
            .field("data255", &self.data255())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 252
    #[inline(always)]
    pub fn data252(&mut self) -> DATA252_W<'_, HUFFSYMB63rs> {
        DATA252_W::new(self, 0)
    }
    ///Bits 8:15 - Data 253
    #[inline(always)]
    pub fn data253(&mut self) -> DATA253_W<'_, HUFFSYMB63rs> {
        DATA253_W::new(self, 8)
    }
    ///Bits 16:23 - Data 254
    #[inline(always)]
    pub fn data254(&mut self) -> DATA254_W<'_, HUFFSYMB63rs> {
        DATA254_W::new(self, 16)
    }
    ///Bits 24:31 - Data 255
    #[inline(always)]
    pub fn data255(&mut self) -> DATA255_W<'_, HUFFSYMB63rs> {
        DATA255_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb63::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb63::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFSYMB63)*/
pub struct HUFFSYMB63rs;
impl crate::RegisterSpec for HUFFSYMB63rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb63::R`](R) reader structure
impl crate::Readable for HUFFSYMB63rs {}
///`write(|w| ..)` method takes [`huffsymb63::W`](W) writer structure
impl crate::Writable for HUFFSYMB63rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB63 to value 0
impl crate::Resettable for HUFFSYMB63rs {}
